#[cfg(test)]
mod tests {
    use crate::{
        lexer::{Lexer, Tokens},
        nexus::Nexus,
        parser::{Parser, ParsingError},
    };

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
