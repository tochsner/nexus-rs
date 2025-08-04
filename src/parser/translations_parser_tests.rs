#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        lexer::{lexer::Lexer, tokens::Tokens},
        parser::parser::{Parser, ParsingError},
        types::nexus::NexusBlock,
    };

    #[test]
    fn test_translations_block() {
        let text = "#NEXUS
        BEGIN taxa;
            DIMENSIONS ntax=5;
            TAXLABELS Apes Humans 'Gorilla 1' 'Gorilla, 2;' 'Chimpanz''ee';
        END;

        BEGIN trees;
            Translate
                some very arbitrary text & some weird symbols cool: Apes,
                some other arbitrary text + some other symbols- Humans,
                Gorilla         'Gorilla 1',
                Gorilla2         'Gorilla, 2;',
                Schimpansen 'Chimpanz''ee'
                ;
        END;
        ";
        let lexer = Lexer::new(text);
        let tokens = Tokens::new(&lexer);
        let mut parser = Parser::new(tokens);
        let result = parser.parse().unwrap();
        assert_eq!(
            result.blocks.get(1),
            Some(&NexusBlock::TreesBlock(
                HashMap::from([
                    (
                        "some very arbitrary text & some weird symbols cool:".to_string(),
                        "Apes".to_string()
                    ),
                    (
                        "some other arbitrary text + some other symbols-".to_string(),
                        "Humans".to_string()
                    ),
                    ("Gorilla".to_string(), "Gorilla 1".to_string()),
                    ("Gorilla2".to_string(), "Gorilla, 2;".to_string()),
                    ("Schimpansen".to_string(), "Chimpanz''ee".to_string()),
                ]),
                vec![]
            ))
        );
    }

    #[test]
    fn test_translations_block_with_different_whitespace() {
        let text = "#NEXUS
        BEGIN taxa;
            DIMENSIONS NTAX = 4;
            TAXLABELS Apes Humans Gorilla 'Chimpanz''ee';
        END;

        BEGIN trees;
            Translate some very arbitrary text & some weird symbols cool: Apes, some other arbitrary text + some other symbols- Humans, Gorilla         Gorilla, Schimpansen 'Chimpanz''ee';
        END;
        ";
        let lexer = Lexer::new(text);
        let tokens = Tokens::new(&lexer);
        let mut parser = Parser::new(tokens);
        let result = parser.parse().unwrap();
        assert_eq!(
            result.blocks.get(1),
            Some(&NexusBlock::TreesBlock(
                HashMap::from([
                    (
                        "some very arbitrary text & some weird symbols cool:".to_string(),
                        "Apes".to_string()
                    ),
                    (
                        "some other arbitrary text + some other symbols-".to_string(),
                        "Humans".to_string()
                    ),
                    ("Gorilla".to_string(), "Gorilla".to_string()),
                    ("Schimpansen".to_string(), "Chimpanz''ee".to_string()),
                ]),
                vec![]
            ))
        );
    }

    #[test]
    fn test_numerical_translations() {
        let text = "#NEXUS
        BEGIN taxa;
            DIMENSIONS NTAX = 4;
            TAXLABELS 1 2 3 4;
        END;

        BEGIN trees;
            Translate
                0 1,
                1 2,
                2 3,
                3 4
            ;
        END;
        ";
        let lexer = Lexer::new(text);
        let tokens = Tokens::new(&lexer);
        let mut parser = Parser::new(tokens);
        let result = parser.parse().unwrap();
        assert_eq!(
            result.blocks.get(1),
            Some(&NexusBlock::TreesBlock(
                HashMap::from([
                    ("0".to_string(), "1".to_string()),
                    ("1".to_string(), "2".to_string()),
                    ("2".to_string(), "3".to_string()),
                    ("3".to_string(), "4".to_string()),
                ]),
                vec![]
            ))
        );
    }

    #[test]
    fn test_partial_translations() {
        let text = "#NEXUS
        BEGIN taxa;
            DIMENSIONS ntax=4;
            TAXLABELS Apes Humans Gorilla 'Chimpanz''ee';
        END;

        BEGIN trees;
            Translate some very arbitrary text & some weird symbols cool: Apes;
        END;
        ";
        let lexer = Lexer::new(text);
        let tokens = Tokens::new(&lexer);
        let mut parser = Parser::new(tokens);
        let result = parser.parse().unwrap();
        assert_eq!(
            result.blocks.get(1),
            Some(&NexusBlock::TreesBlock(
                HashMap::from([(
                    "some very arbitrary text & some weird symbols cool:".to_string(),
                    "Apes".to_string()
                ),]),
                vec![]
            ))
        );
    }

    #[test]
    fn test_duplicate_translations() {
        let text = "#NEXUS
        BEGIN taxa;
            DIMENSIONS ntax=2;
            TAXLABELS Apes Humans;
        END;

        BEGIN trees;
            Translate
                Affen Apes,
                Affen Humans;
        END;";
        let lexer = Lexer::new(text);
        let tokens = Tokens::new(&lexer);
        let mut parser = Parser::new(tokens);
        assert_eq!(parser.parse(), Err(ParsingError::DuplicateTranslations));
    }

    #[test]
    fn test_multiple_translations_for_taxa() {
        let text = "#NEXUS
        BEGIN taxa;
            DIMENSIONS ntax=2;
            TAXLABELS Apes Humans;
        END;

        BEGIN trees;
            Translate
                Affen Apes,
                Affen2 Apes,
                Menschen Humans;
        END;";
        let lexer = Lexer::new(text);
        let tokens = Tokens::new(&lexer);
        let mut parser = Parser::new(tokens);
        assert_eq!(parser.parse(), Err(ParsingError::DuplicateTranslations));
    }

    #[test]
    fn test_translations_for_unknown_taxa() {
        let text = "#NEXUS
        BEGIN taxa;
            DIMENSIONS ntax=2;
            TAXLABELS Apes Humans;
        END;

        BEGIN trees;
            Translate Gorillas Gorillas;
        END;";
        let lexer = Lexer::new(text);
        let tokens = Tokens::new(&lexer);
        let mut parser = Parser::new(tokens);
        assert_eq!(parser.parse(), Err(ParsingError::TranslationForUnknownTaxa));
    }

    #[test]
    fn test_empty_translations_block() {
        let text = "#NEXUS
        BEGIN taxa;
            DIMENSIONS ntax=2;
            TAXLABELS Apes Humans;
        END;

        BEGIN trees;
            Translate;
        END;";
        let lexer = Lexer::new(text);
        let tokens = Tokens::new(&lexer);
        let mut parser = Parser::new(tokens);
        let result = parser.parse().unwrap();
        assert_eq!(
            result.blocks.get(1),
            Some(&NexusBlock::TreesBlock(
                HashMap::<String, String>::from([]),
                vec![]
            ))
        );
    }
}
