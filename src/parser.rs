use crate::lexer::{self, Lexer, Token};

#[derive(PartialEq, Debug)]
enum NexusBlock<'a> {
    TaxaBlock(usize, Vec<&'a str>),
    TreesBlock,
}

impl<'a> NexusBlock<'a> {
    fn build_taxa_block(
        dimensions: usize,
        tax_labels: Vec<&'a str>,
    ) -> Result<NexusBlock, ParsingError> {
        if dimensions != tax_labels.len() {
            Err(ParsingError::TaxaDimensionsMismatch)
        } else {
            Ok(NexusBlock::TaxaBlock(dimensions, tax_labels))
        }
    }
}

#[derive(PartialEq, Debug)]
struct Nexus<'a> {
    blocks: Vec<NexusBlock<'a>>,
}

impl<'a> Nexus<'a> {
    fn new() -> Self {
        Nexus { blocks: vec![] }
    }
}

#[derive(PartialEq, Debug)]
enum ParsingError {
    MissingNexusTag,
    InvalidBlock,
    MissingToken(String),
    UnexpectedToken,
    InvalidNumber,
    InvalidList,
    TaxaDimensionsMismatch,
}

struct NexusParser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> NexusParser<'a> {
    fn new(lexer: Lexer<'a>) -> Self {
        Self { lexer }
    }

    fn parse(&mut self) -> Result<Nexus, ParsingError> {
        self.parse_nexus_tag()?;
        self.parse_and_ignore_whitespace();

        let mut nexus = Nexus::new();

        while let Some(block) = self.parse_block()? {
            nexus.blocks.push(block);
        }

        Ok(nexus)
    }

    fn parse_nexus_tag(&mut self) -> Result<(), ParsingError> {
        self.try_and_parse_expected_word("#NEXUS")
    }

    fn parse_block(&mut self) -> Result<Option<NexusBlock<'a>>, ParsingError> {
        if self.lexer.peek() == None {
            return Ok(None);
        }

        self.parse_and_ignore_whitespace();
        self.try_and_parse_expected_word("begin")?;
        self.parse_and_ignore_whitespace();

        return Ok(self.parse_taxa_block()?);

        if let Ok(block) = self.parse_taxa_block() {
            return Ok(block);
        }

        if let Ok(block) = self.parse_trees_block() {
            return Ok(block);
        }

        Err(ParsingError::InvalidBlock)
    }

    fn parse_taxa_block(&mut self) -> Result<Option<NexusBlock<'a>>, ParsingError> {
        self.try_and_parse_expected_word("taxa")?;
        self.parse_and_ignore_whitespace();
        self.try_and_parse_expected_eos()?;
        self.parse_and_ignore_whitespace();

        self.try_and_parse_expected_word("Dimensions")?;
        self.parse_and_ignore_whitespace();
        let dimension = self.parse_uint()?;
        self.try_and_parse_expected_eos()?;

        self.parse_and_ignore_whitespace();
        self.try_and_parse_expected_word("TaxLabels")?;
        self.parse_and_ignore_whitespace();
        let taxa_labels = self.parse_words()?;
        self.try_and_parse_expected_eos()?;

        self.parse_and_ignore_whitespace();
        self.try_and_parse_expected_word("end")?;
        self.parse_and_ignore_whitespace();
        self.try_and_parse_expected_eos()?;

        Ok(Some(NexusBlock::build_taxa_block(dimension, taxa_labels)?))
    }

    fn parse_trees_block(&mut self) -> Result<Option<NexusBlock<'a>>, ParsingError> {
        self.try_and_parse_expected_word("trees")?;
        self.parse_and_ignore_whitespace();
        self.try_and_parse_expected_eos()?;
        self.parse_and_ignore_whitespace();

        self.try_and_parse_expected_word("end")?;
        self.parse_and_ignore_whitespace();
        self.try_and_parse_expected_eos()?;

        Ok(Some(NexusBlock::TreesBlock))
    }

    fn parse_and_ignore_whitespace(&mut self) {
        while let Some(Token::Whitespace(_)) = &self.lexer.peek() {
            self.lexer.next();
        }
    }

    fn parse_uint(&mut self) -> Result<usize, ParsingError> {
        if let Some(Token::Word(word)) = self.lexer.next() {
            if let Ok(num) = word.parse() {
                return Ok(num);
            }
        }
        Err(ParsingError::InvalidNumber)
    }

    fn parse_words(&mut self) -> Result<Vec<&'a str>, ParsingError> {
        let mut labels = vec![];

        while self.lexer.peek() != Some(Token::EOS) {
            self.parse_and_ignore_whitespace();
            match self.lexer.next() {
                Some(Token::Word(word)) => labels.push(word),
                _ => return Err(ParsingError::InvalidList),
            }
            self.parse_and_ignore_whitespace();
        }

        Ok(labels)
    }

    fn try_and_parse_expected<F>(&mut self, predicate: F) -> Result<(), ParsingError>
    where
        F: Fn(Option<Token>) -> bool,
    {
        if predicate(self.lexer.peek()) {
            self.lexer.next();
            Ok(())
        } else {
            Err(ParsingError::UnexpectedToken)
        }
    }

    fn try_and_parse_expected_word(&mut self, expected_word: &str) -> Result<(), ParsingError> {
        self.try_and_parse_expected(|token| {
            if let Some(Token::Word(word)) = token {
                word.eq_ignore_ascii_case(expected_word)
            } else {
                false
            }
        })
        .map_err(|_| ParsingError::MissingToken(String::from(expected_word)))
    }

    fn try_and_parse_expected_eos(&mut self) -> Result<(), ParsingError> {
        self.try_and_parse_expected(|token| {
            if let Some(Token::EOS) = token {
                true
            } else {
                false
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_nexus() {
        let text = "#NEXUS";
        let lexer = Lexer::new(text);
        let mut parser = NexusParser::new(lexer);
        assert_eq!(parser.parse(), Ok(Nexus::new()));

        let text = "#nexus";
        let lexer = Lexer::new(text);
        let mut parser = NexusParser::new(lexer);
        assert_eq!(parser.parse(), Ok(Nexus::new()));
    }

    // #[test]
    fn test_trees_block() {
        let text = "#NEXUS
        BEGIN TAXA;
        END;";
        let lexer = Lexer::new(text);
        let mut parser = NexusParser::new(lexer);
        assert_eq!(
            parser.parse(),
            Ok(Nexus {
                blocks: vec![NexusBlock::TaxaBlock(0, vec![])]
            })
        );
    }

    #[test]
    fn test_taxa_block() {
        let text = "#NEXUS
        BEGIN taxa;
        DIMENSIONS 2;
        TAXLABELS Apes Humans;
        END;";
        let lexer = Lexer::new(text);
        let mut parser = NexusParser::new(lexer);
        assert_eq!(
            parser.parse(),
            Ok(Nexus {
                blocks: vec![NexusBlock::TaxaBlock(2, vec!["Apes", "Humans"])]
            })
        );
    }

    #[test]
    fn test_taxa_block_with_missing_pieces() {
        let text = "#NEXUS
        BEGIN taxa;
        DIMENSIONS 10;
        END;";
        let lexer = Lexer::new(text);
        let mut parser = NexusParser::new(lexer);
        assert_eq!(
            parser.parse(),
            Err(ParsingError::MissingToken(String::from("TaxLabels")))
        );
        
        let text = "#NEXUS
        BEGIN taxa;
        DIMENSIONS;
        END;";
        let lexer = Lexer::new(text);
        let mut parser = NexusParser::new(lexer);
        assert_eq!(
            parser.parse(),
            Err(ParsingError::InvalidNumber)
        );

        let text = "#NEXUS
        BEGIN taxa;
        TAXLABELS Apes Humans;
        END;";
        let lexer = Lexer::new(text);
        let mut parser = NexusParser::new(lexer);
        assert_eq!(
            parser.parse(),
            Err(ParsingError::MissingToken(String::from("Dimensions")))
        );

        let text = "#NEXUS
        BEGIN taxa;
        DIMENSIONS 2
        TAXLABELS;
        END;";
        let lexer = Lexer::new(text);
        let mut parser = NexusParser::new(lexer);
        assert_eq!(
            parser.parse(),
            Err(ParsingError::UnexpectedToken)
        );
    }

    #[test]
    fn test_taxa_block_dimension_mismatch() {
        let text = "#NEXUS
        BEGIN taxa;
        DIMENSIONS 2;
        TAXLABELS human ape gorilla;
        END;";
        let lexer = Lexer::new(text);
        let mut parser = NexusParser::new(lexer);
        assert_eq!(
            parser.parse(),
            Err(ParsingError::TaxaDimensionsMismatch)
        );
    }
}
