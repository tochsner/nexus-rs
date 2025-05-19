use std::collections::HashMap;

use indextree::{Arena, NodeId};

use crate::{
    lexer::{Lexer, Token},
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
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self { lexer }
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

        if self.lexer.peek() == None {
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

        while self.lexer.peek() != Some(Token::EOS) {
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

        let mut translation_start = self.lexer.cursor();
        let mut translation_end = self.lexer.cursor();

        loop {
            match self.lexer.next() {
                Some(Token::Whitespace(_)) => {
                    let translated_taxa_name =
                        self.lexer.slice_from_to(translation_start, translation_end);

                    // test if this is the last translated taxa

                    if let Ok(actual_taxa_name) = self.try_parser(|s| {
                        let taxa_name = s.parse_word()?;
                        s.parse_eos()?;
                        Ok(taxa_name)
                    }) {
                        if translations.insert(translated_taxa_name, actual_taxa_name) != None {
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
                        translation_start = self.lexer.cursor();
                    }
                }
                None => return Err(ParsingError::UnexpectedFileEnd),
                _ => translation_end = self.lexer.cursor(),
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
        self.parse_nexus_subtree(&mut arena)?;

        Ok(arena)
    }

    fn parse_nexus_subtree(
        &mut self,
        arena: &mut Arena<TreeNode<'a>>,
    ) -> Result<NodeId, ParsingError> {
        if self.try_parser(|s| s.parse_punctuation("(")).is_ok() {
            let subtree_root = match arena.is_empty() {
                true => arena.new_node(TreeNode::new_root()),
                false => arena.new_node(TreeNode::new_internal()),
            };

            subtree_root.append(self.parse_nexus_subtree(arena)?, arena);
            self.parse_punctuation(",")?;
            subtree_root.append(self.parse_nexus_subtree(arena)?, arena);
            self.parse_punctuation(")")?;

            return Ok(subtree_root);
        }

        if let Ok(taxon) = self.try_parser(|s| s.parse_word()) {
            let leaf = arena.new_node(TreeNode::new_leaf(taxon));
            return Ok(leaf);
        }

        Err(ParsingError::MissingEOS)
    }

    // atomic parsers

    fn parse_eos(&mut self) -> Result<(), ParsingError> {
        self.parse_and_ignore_whitespace();

        match self.lexer.next() {
            Some(Token::EOS) => Ok(()),
            _ => Err(ParsingError::MissingEOS),
        }
    }

    fn parse_punctuation(&mut self, expected_punctuation: &str) -> Result<&'a str, ParsingError> {
        self.parse_and_ignore_whitespace();

        match self.lexer.next() {
            Some(Token::Punctuation(punct)) if punct == expected_punctuation => {
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

        let Some(Token::Word(word)) = self.lexer.next() else {
            return Err(ParsingError::InvalidNumber);
        };

        let Ok(num) = word.parse() else {
            return Err(ParsingError::InvalidNumber);
        };

        self.parse_and_ignore_whitespace();
        return Ok(num);
    }

    fn parse_keyword(&mut self, expected_word: &str) -> Result<&'a str, ParsingError> {
        self.parse_and_ignore_whitespace();

        match self.lexer.next() {
            Some(Token::Word(word)) if word.eq_ignore_ascii_case(expected_word) => {
                self.parse_and_ignore_whitespace();
                Ok(word)
            }
            _ => Err(ParsingError::MissingToken(String::from(expected_word))),
        }
    }

    fn parse_word(&mut self) -> Result<&'a str, ParsingError> {
        self.parse_and_ignore_whitespace();

        match self.lexer.next() {
            Some(Token::Word(word)) => Ok(word),
            // the next token is a quotation mark, we have a quoted word
            Some(Token::Punctuation("'")) => {
                let start_cursor = self.lexer.cursor();

                loop {
                    match self.lexer.next() {
                        Some(Token::Punctuation("'")) => {
                            // we have two cases:
                            //      either, this is the final quotation mark,
                            //      or, there is a pair of quotation marks
                            if self.lexer.peek() == Some(Token::Punctuation("'")) {
                                self.lexer.next();
                                continue;
                            }

                            // the word is finished, we return the word without the last quotation mark
                            let concatenated_word =
                                self.lexer.slice(start_cursor, self.lexer.cursor() - 1);
                            return Ok(concatenated_word);
                        }
                        None => return Err(ParsingError::UnexpectedFileEnd),
                        _ => continue,
                    }
                }
            }
            Some(token) => Err(ParsingError::UnexpectedToken(token.to_string())),
            None => Err(ParsingError::UnexpectedFileEnd),
        }
    }

    fn parse_and_ignore_whitespace(&mut self) {
        while let Some(Token::Whitespace(_)) = &self.lexer.peek() {
            self.lexer.next();
        }
    }

    fn try_parser<T, F>(&mut self, parser: F) -> Result<T, ParsingError>
    where
        F: FnOnce(&mut Self) -> Result<T, ParsingError>,
    {
        let initial_cursor = self.lexer.cursor();

        match parser(self) {
            Ok(result) => Ok(result),
            Err(error) => {
                self.lexer.set_cursor(initial_cursor);
                Err(error)
            }
        }
    }
}
