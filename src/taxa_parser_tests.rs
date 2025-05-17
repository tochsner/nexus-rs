#[cfg(test)]
mod tests {
    use crate::{
        lexer::Lexer,
        nexus::{Nexus, NexusBlock},
        parser::{Parser, ParsingError},
    };

    #[test]
    fn test_taxa_block() {
        let text = "#NEXUS
        BEGIN taxa;
        DIMENSIONS 5;
        TAXLABELS Apes 'Humans' 'Gor' 'Gor''illas' 'Gor''ill''as';
        END;";
        let lexer = Lexer::new(text);
        let mut parser = Parser::new(lexer);
        assert_eq!(
            parser.parse(),
            Ok(Nexus {
                blocks: vec![NexusBlock::TaxaBlock(
                    5,
                    vec!["Apes", "Humans", "Gor", "Gor''illas", "Gor''ill''as"]
                )]
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
        let mut parser = Parser::new(lexer);
        assert_eq!(
            parser.parse(),
            Err(ParsingError::MissingToken(String::from("TaxLabels")))
        );

        let text = "#NEXUS
        BEGIN taxa;
        DIMENSIONS;
        END;";
        let lexer = Lexer::new(text);
        let mut parser = Parser::new(lexer);
        assert_eq!(parser.parse(), Err(ParsingError::InvalidNumber));

        let text = "#NEXUS
        BEGIN taxa;
        TAXLABELS Apes Humans;
        END;";
        let lexer = Lexer::new(text);
        let mut parser = Parser::new(lexer);
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
        let mut parser = Parser::new(lexer);
        assert_eq!(parser.parse(), Err(ParsingError::MissingEOS));
    }

    #[test]
    fn test_taxa_block_dimension_mismatch() {
        let text = "#NEXUS
        BEGIN taxa;
        DIMENSIONS 2;
        TAXLABELS human ape gorilla;
        END;";
        let lexer = Lexer::new(text);
        let mut parser = Parser::new(lexer);
        assert_eq!(parser.parse(), Err(ParsingError::TaxaDimensionsMismatch));
    }
}
