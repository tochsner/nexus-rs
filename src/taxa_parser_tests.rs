#[cfg(test)]
mod tests {
    use crate::{
        lexer::{Lexer, Tokens},
        nexus::{Nexus, NexusBlock},
        parser::{Parser, ParsingError},
    };

    #[test]
    fn test_taxa_block() {
        let text = "#NEXUS
        BEGIN taxa;
        DIMENSIONS ntax=5;
        TAXLABELS Apes 'Humans' 'Gor' 'Gor''illas' 'Gor''ill''as';
        END;";
        let lexer = Lexer::new(text);
        let tokens = Tokens::new(&lexer);
        let mut parser = Parser::new(tokens);
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
    fn test_taxa_block_with_different_whitespace() {
        let text = "#NEXUS
        BEGIN taxa;
        DIMENSIONS ntax=5;
        TAXLABELS
            Apes
            'Humans'
            'Gor'
            'Gor''illas'
            'Gor''ill''as'
        ;
        END;";
        let lexer = Lexer::new(text);
        let tokens = Tokens::new(&lexer);
        let mut parser = Parser::new(tokens);
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
        DIMENSIONS ntax=10;
        END;";
        let lexer = Lexer::new(text);
        let tokens = Tokens::new(&lexer);
        let mut parser = Parser::new(tokens);
        assert_eq!(
            parser.parse(),
            Err(ParsingError::MissingToken(String::from("TaxLabels")))
        );

        let text = "#NEXUS
        BEGIN taxa;
        DIMENSIONS 0;
        END;";
        let lexer = Lexer::new(text);
        let tokens = Tokens::new(&lexer);
        let mut parser = Parser::new(tokens);
        assert_eq!(
            parser.parse(),
            Err(ParsingError::MissingToken(String::from("ntax")))
        );

        let text = "#NEXUS
        BEGIN taxa;
        DIMENSIONS ntax=;
        END;";
        let lexer = Lexer::new(text);
        let tokens = Tokens::new(&lexer);
        let mut parser = Parser::new(tokens);
        assert_eq!(parser.parse(), Err(ParsingError::InvalidNumber));

        let text = "#NEXUS
        BEGIN taxa;
        TAXLABELS Apes Humans;
        END;";
        let lexer = Lexer::new(text);
        let tokens = Tokens::new(&lexer);
        let mut parser = Parser::new(tokens);
        assert_eq!(
            parser.parse(),
            Err(ParsingError::MissingToken(String::from("Dimensions")))
        );

        let text = "#NEXUS
        BEGIN taxa;
        DIMENSIONS ntax=2
        TAXLABELS;
        END;";
        let lexer = Lexer::new(text);
        let tokens = Tokens::new(&lexer);
        let mut parser = Parser::new(tokens);
        assert_eq!(parser.parse(), Err(ParsingError::MissingEOS));
    }

    #[test]
    fn test_taxa_block_dimension_mismatch() {
        let text = "#NEXUS
        BEGIN taxa;
        DIMENSIONS ntax=2;
        TAXLABELS human ape gorilla;
        END;";
        let lexer = Lexer::new(text);
        let tokens = Tokens::new(&lexer);
        let mut parser = Parser::new(tokens);
        assert_eq!(parser.parse(), Err(ParsingError::TaxaDimensionsMismatch));
    }

    #[test]
    fn test_taxa_block_with_empty_labels() {
        let text = "#NEXUS
        BEGIN taxa;
        DIMENSIONS ntax=0;
        TAXLABELS;
        END;";
        let lexer = Lexer::new(text);
        let tokens = Tokens::new(&lexer);
        let mut parser = Parser::new(tokens);
        assert_eq!(
            parser.parse(),
            Ok(Nexus {
                blocks: vec![NexusBlock::TaxaBlock(0, vec![])]
            })
        );
    }

    #[test]
    fn test_taxa_block_with_special_characters() {
        let text = "#NEXUS
        BEGIN taxa;
        DIMENSIONS ntax=3;
        TAXLABELS 'Species@1' 'Species#2' 'Species$3';
        END;";
        let lexer = Lexer::new(text);
        let tokens = Tokens::new(&lexer);
        let mut parser = Parser::new(tokens);
        assert_eq!(
            parser.parse(),
            Ok(Nexus {
                blocks: vec![NexusBlock::TaxaBlock(
                    3,
                    vec!["Species@1", "Species#2", "Species$3"]
                )]
            })
        );
    }

    #[test]
    fn test_taxa_block_with_whitespace_labels() {
        let text = "#NEXUS
        BEGIN taxa;
        DIMENSIONS ntax=2;
        TAXLABELS 'Species 1' 'Species 2';
        END;";
        let lexer = Lexer::new(text);
        let tokens = Tokens::new(&lexer);
        let mut parser = Parser::new(tokens);
        assert_eq!(
            parser.parse(),
            Ok(Nexus {
                blocks: vec![NexusBlock::TaxaBlock(2, vec!["Species 1", "Species 2"])]
            })
        );
    }
}
