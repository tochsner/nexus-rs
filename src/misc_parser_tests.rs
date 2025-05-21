#[cfg(test)]
mod tests {
    use crate::tokens::Tokens;
    use crate::{
        lexer::Lexer,
        nexus::Nexus,
        parser::{Parser, ParsingError},
    };

    impl<'a> Nexus<'a> {
        pub fn new() -> Self {
            Nexus { blocks: vec![] }
        }
    }

    #[test]
    fn test_empty_nexus() {
        let text = "#NEXUS";
        let lexer = Lexer::new(text);
        let tokens = Tokens::new(&lexer);
        let mut parser = Parser::new(tokens);
        assert_eq!(parser.parse(), Ok(Nexus::new()));

        let text = "#nexus";
        let lexer = Lexer::new(text);
        let tokens = Tokens::new(&lexer);
        let mut parser = Parser::new(tokens);
        assert_eq!(parser.parse(), Ok(Nexus::new()));

        let text = "#notnexus";
        let lexer = Lexer::new(text);
        let tokens = Tokens::new(&lexer);
        let mut parser = Parser::new(tokens);
        assert_eq!(parser.parse(), Err(ParsingError::MissingNexusTag));
    }

    #[test]
    fn test_invalid_block() {
        let text = "#NEXUS
        BEG;
        END;";
        let lexer = Lexer::new(text);
        let tokens = Tokens::new(&lexer);
        let mut parser = Parser::new(tokens);
        assert_eq!(
            parser.parse(),
            Err(ParsingError::MissingToken(String::from("begin")))
        );
    }
}
