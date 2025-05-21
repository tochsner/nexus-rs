use std::collections::HashMap;

use indextree::{Arena, NodeId};

use crate::{
    lexer::{Token, Tokens},
    nexus::{Nexus, NexusBlock},
    tree::{Tree, TreeNode},
};

#[derive(PartialEq, Debug)]
pub enum ParsingError {
    // misc
    MissingNexusTag,
    MissingEOS,
    InvalidBlock,
    MissingToken(String),
    UnexpectedToken(String),
    InvalidNumber,
    UnexpectedFileEnd,
    // taxa block
    InvalidList,
    TaxaDimensionsMismatch,
    // trees block
    DuplicateTranslations,
    TranslationForUnknownTaxa,
}

pub struct Parser<'a> {
    tokens: Tokens<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Tokens<'a>) -> Self {
        Self { tokens }
    }

    pub fn parse(&mut self) -> Result<Nexus, ParsingError> {
        self.parse_nexus_tag()?;

        let mut blocks: Vec<NexusBlock<'a>> = vec![];
        while let Some(block) = self.parse_block()? {
            blocks.push(block);
        }

        Nexus::build(blocks)
    }

    fn parse_nexus_tag(&mut self) -> Result<&str, ParsingError> {
        self.parse_keyword("#NEXUS")
            .map_err(|_| ParsingError::MissingNexusTag)
    }

    fn parse_block(&mut self) -> Result<Option<NexusBlock<'a>>, ParsingError> {
        self.parse_and_ignore_whitespace();

        if self.tokens.peek().is_none() {
            return Ok(None);
        }

        self.parse_keyword("begin")?;

        if self.try_parser(|s| s.parse_keyword("taxa")).is_ok() {
            return self.parse_taxa_block();
        }
        if self.try_parser(|s| s.parse_keyword("trees")).is_ok() {
            return self.parse_trees_block();
        }

        Err(ParsingError::InvalidBlock)
    }

    // taxa block parsing

    fn parse_taxa_block(&mut self) -> Result<Option<NexusBlock<'a>>, ParsingError> {
        self.parse_eos()?;

        self.parse_keyword("Dimensions")?;
        self.parse_keyword("ntax")?;
        self.parse_punctuation("=")?;
        let dimension = self.parse_uint()?;
        self.parse_eos()?;

        self.parse_keyword("TaxLabels")?;
        let taxa_labels = self.parse_words()?;
        self.parse_eos()?;

        self.parse_keyword("end")?;
        self.parse_eos()?;

        Ok(Some(NexusBlock::build_taxa_block(dimension, taxa_labels)?))
    }

    fn parse_words(&mut self) -> Result<Vec<&'a str>, ParsingError> {
        let mut labels = vec![];

        while self.tokens.peek() != Some(&Token::EOS) {
            match self.parse_word() {
                Ok(word) => labels.push(word),
                _ => return Err(ParsingError::InvalidList),
            }
            self.parse_and_ignore_whitespace();
        }

        Ok(labels)
    }

    // trees block parsing

    fn parse_trees_block(&mut self) -> Result<Option<NexusBlock<'a>>, ParsingError> {
        self.parse_eos()?;

        let translations = self.parse_taxa_translations()?;
        let trees = self.parse_trees()?;

        self.parse_keyword("end")?;
        self.parse_eos()?;

        Ok(Some(NexusBlock::build_trees_block(translations, trees)?))
    }

    fn parse_taxa_translations(&mut self) -> Result<HashMap<&'a str, &'a str>, ParsingError> {
        if self.try_parser(|s| s.parse_keyword("Translate")).is_err() {
            return Ok(HashMap::new());
        }

        if self.try_parser(|s| s.parse_eos()).is_ok() {
            return Ok(HashMap::new());
        }

        let mut translations = HashMap::new();

        let mut translation_start = self.tokens.cursor();
        let mut translation_end = self.tokens.cursor();

        loop {
            match self.tokens.next() {
                Some(Token::Whitespace(_)) => {
                    let translated_taxa_name = self
                        .tokens
                        .slice_from_to(translation_start, translation_end);

                    // test if this is the last translated taxa

                    if let Ok(actual_taxa_name) = self.try_parser(|s| {
                        let taxa_name = s.parse_word()?;
                        s.parse_eos()?;
                        Ok(taxa_name)
                    }) {
                        if translations
                            .insert(translated_taxa_name, actual_taxa_name)
                            .is_some()
                        {
                            // there is already a translation with this key
                            return Err(ParsingError::DuplicateTranslations);
                        }
                        return Ok(translations);
                    }

                    // test if this a translated taxa which is not the last

                    if let Ok(actual_taxa_name) = self.try_parser(|s| {
                        let taxa_name = s.parse_word()?;
                        s.parse_punctuation(",")?;
                        Ok(taxa_name)
                    }) {
                        translations.insert(translated_taxa_name, actual_taxa_name);
                        translation_start = self.tokens.cursor();
                    }
                }
                None => return Err(ParsingError::UnexpectedFileEnd),
                _ => translation_end = self.tokens.cursor(),
            };
        }
    }

    fn parse_trees(&mut self) -> Result<Vec<Tree<'a>>, ParsingError> {
        let mut trees = vec![];

        while let Ok(tree) = self.try_parser(|s| s.parse_tree()) {
            trees.push(tree);
        }

        Ok(trees)
    }

    fn parse_tree(&mut self) -> Result<Tree<'a>, ParsingError> {
        self.parse_keyword("TREE")?;
        let tree_name = self.parse_word()?;
        self.parse_punctuation("=")?;
        let arena = self.parse_nexus()?;
        self.parse_eos()?;
        Ok(Tree {
            tree: arena,
            name: tree_name,
            rooted: false,
        })
    }

    fn parse_nexus(&mut self) -> Result<Arena<TreeNode<'a>>, ParsingError> {
        self.parse_and_ignore_whitespace();

        let mut arena = Arena::new();
        self.parse_nexus_subtree(&mut arena, true)?;

        Ok(arena)
    }

    fn parse_nexus_subtree(
        &mut self,
        arena: &mut Arena<TreeNode<'a>>,
        is_root: bool,
    ) -> Result<NodeId, ParsingError> {
        if self.try_parser(|s| s.parse_punctuation("(")).is_ok() {
            let child_id_1 = self.parse_nexus_subtree(arena, false)?;
            self.parse_punctuation(",")?;

            let child_id_2 = self.parse_nexus_subtree(arena, false)?;
            self.parse_punctuation(")")?;

            let subtree_root_node = match is_root {
                true => TreeNode::new_root(),
                false => TreeNode::new_internal(),
            };

            let subtree_root_node = match self.try_parser(|s| {
                s.parse_punctuation(":")?;
                s.parse_f64()
            }) {
                Ok(length) => subtree_root_node.with_length(length),
                _ => subtree_root_node,
            };

            let subtree_root_node_id = arena.new_node(subtree_root_node);
            subtree_root_node_id.append(child_id_1, arena);
            subtree_root_node_id.append(child_id_2, arena);

            return Ok(subtree_root_node_id);
        }

        if let Ok(taxon) = self.try_parser(|s| s.parse_word()) {
            let leaf = TreeNode::new_leaf(taxon);

            let leaf = match self.try_parser(|s| {
                s.parse_punctuation(":")?;
                s.parse_f64()
            }) {
                Ok(length) => leaf.with_length(length),
                _ => leaf,
            };

            let leaf_node_id = arena.new_node(leaf);

            return Ok(leaf_node_id);
        }

        Err(ParsingError::MissingEOS)
    }

    // atomic parsers

    fn parse_eos(&mut self) -> Result<(), ParsingError> {
        self.parse_and_ignore_whitespace();

        match self.tokens.next() {
            Some(Token::EOS) => Ok(()),
            _ => Err(ParsingError::MissingEOS),
        }
    }

    fn parse_punctuation(&mut self, expected_punctuation: &str) -> Result<&'a str, ParsingError> {
        self.parse_and_ignore_whitespace();

        match self.tokens.next() {
            Some(Token::Punctuation(punct)) if punct == &expected_punctuation => {
                self.parse_and_ignore_whitespace();
                Ok(punct)
            }
            _ => Err(ParsingError::MissingToken(String::from(
                expected_punctuation,
            ))),
        }
    }

    fn parse_uint(&mut self) -> Result<usize, ParsingError> {
        self.parse_and_ignore_whitespace();

        if let Some(Token::Integer(number)) = self.tokens.next() {
            Ok(*number as usize)
        } else {
            Err(ParsingError::InvalidNumber)
        }
    }

    fn parse_f64(&mut self) -> Result<f64, ParsingError> {
        self.parse_and_ignore_whitespace();

        match self.tokens.next() {
            Some(Token::Integer(number)) => Ok(f64::from(*number)),
            Some(Token::Float(number)) => Ok(*number),
            _ => Err(ParsingError::InvalidNumber),
        }
    }

    fn parse_keyword(&mut self, expected_word: &str) -> Result<&'a str, ParsingError> {
        self.parse_and_ignore_whitespace();

        match self.tokens.next() {
            Some(Token::Word(word)) if word.eq_ignore_ascii_case(expected_word) => {
                self.parse_and_ignore_whitespace();
                Ok(word)
            }
            a => Err(ParsingError::MissingToken(String::from(expected_word))),
        }
    }

    fn parse_word(&mut self) -> Result<&'a str, ParsingError> {
        self.parse_and_ignore_whitespace();

        match self.tokens.next() {
            Some(Token::Word(word)) => Ok(word),
            Some(Token::QuotedWord(word)) => Ok(word),
            Some(token) => Err(ParsingError::UnexpectedToken(token.to_string())),
            None => Err(ParsingError::UnexpectedFileEnd),
        }
    }

    fn parse_and_ignore_whitespace(&mut self) {
        while let Some(Token::Whitespace(_)) = &self.tokens.peek() {
            self.tokens.next();
        }
    }

    fn try_parser<T, F>(&mut self, parser: F) -> Result<T, ParsingError>
    where
        F: FnOnce(&mut Self) -> Result<T, ParsingError>,
    {
        let initial_cursor = self.tokens.cursor();

        match parser(self) {
            Ok(result) => Ok(result),
            Err(error) => {
                self.tokens.set_cursor(initial_cursor);
                Err(error)
            }
        }
    }
}
