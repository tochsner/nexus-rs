use std::collections::HashMap;

use crate::{
    lexer::{Lexer, Token},
    nexus::{Nexus, NexusBlock},
};

#[derive(PartialEq, Debug)]
pub enum ParsingError {
    MissingNexusTag,
    MissingEOS,
    InvalidBlock,
    MissingToken(String),
    UnexpectedToken,
    InvalidNumber,
    InvalidList,
    TaxaDimensionsMismatch,
    UnexpectedFileEnd,
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self { lexer }
    }

    pub fn parse(&mut self) -> Result<Nexus, ParsingError> {
        let mut nexus = Nexus::new();

        self.parse_nexus_tag()?;

        while let Some(block) = self.parse_block()? {
            nexus.blocks.push(block);
        }

        Ok(nexus)
    }

    fn parse_nexus_tag(&mut self) -> Result<&str, ParsingError> {
        self.parse_keyword("#NEXUS")
            .map_err(|_| ParsingError::MissingNexusTag)
    }

    fn parse_block(&mut self) -> Result<Option<NexusBlock<'a>>, ParsingError> {
        if self.lexer.peek() == None {
            return Ok(None);
        }

        self.parse_keyword("begin")?;

        let mut first_block_error: Option<ParsingError> = None;

        match self.try_and_parse_taxa_block() {
            Ok(block) => return Ok(block),
            Err(error) if first_block_error == None => first_block_error = Some(error),
            _ => {}
        }

        match self.try_and_parse_trees_block() {
            Ok(block) => return Ok(block),
            Err(error) if first_block_error == None => first_block_error = Some(error),
            _ => {}
        }

        Err(first_block_error.unwrap_or(ParsingError::InvalidBlock))
    }

    // taxa block parsing

    fn try_and_parse_taxa_block(&mut self) -> Result<Option<NexusBlock<'a>>, ParsingError> {
        self.try_and_parse_keyword("taxa")?;
        self.parse_eos()?;

        self.parse_keyword("Dimensions")?;
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
        }

        Ok(labels)
    }

    // trees block parsing

    fn try_and_parse_trees_block(&mut self) -> Result<Option<NexusBlock<'a>>, ParsingError> {
        self.try_and_parse_keyword("trees")?;
        self.parse_eos()?;

        let translations = self.parse_taxa_translations()?;

        self.parse_keyword("end")?;
        self.parse_eos()?;

        Ok(Some(NexusBlock::build_trees_block(translations)?))
    }

    fn parse_taxa_translations(&mut self) -> Result<HashMap<&'a str, &'a str>, ParsingError> {
        if self.try_and_parse_keyword("Translate").is_err() {
            return Ok(HashMap::new());
        }

        Err(ParsingError::InvalidBlock)
    }

    // atomic parsers

    fn parse_eos(&mut self) -> Result<(), ParsingError> {
        self.parse_and_ignore_whitespace();

        match self.lexer.next() {
            Some(Token::EOS) => Ok(()),
            _ => Err(ParsingError::MissingEOS),
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

    fn try_and_parse_keyword(&mut self, expected_word: &str) -> Result<(), ParsingError> {
        self.parse_and_ignore_whitespace();

        match self.lexer.peek() {
            Some(Token::Word(word)) if word.eq_ignore_ascii_case(expected_word) => {
                self.lexer.next();
                self.parse_and_ignore_whitespace();
                Ok(())
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
                        Some(Token::Word(_)) => continue,
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
                        _ => return Err(ParsingError::UnexpectedToken),
                    }
                }
            }
            Some(_) => Err(ParsingError::UnexpectedToken),
            None => Err(ParsingError::UnexpectedFileEnd),
        }
    }

    fn parse_and_ignore_whitespace(&mut self) {
        while let Some(Token::Whitespace(_)) = &self.lexer.peek() {
            self.lexer.next();
        }
    }
}
