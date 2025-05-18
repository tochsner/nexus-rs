#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        lexer::Lexer,
        nexus::NexusBlock,
        parser::{Parser, ParsingError},
    };

    #[test]
    fn test_translations_block() {
        let text = "#NEXUS
        BEGIN taxa;
            DIMENSIONS 4;
            TAXLABELS Apes Humans Gorilla 'Chimpanz''ee';
        END;

        BEGIN trees;
            Translate
                some very arbitrary text & some weird symbols cool: Apes,
                some other arbitrary text + some other symbols- Humans,
                Gorilla         Gorilla,
                Schimpansen 'Chimpanz''ee';
        END;
        ";
        let lexer = Lexer::new(text);
        let mut parser = Parser::new(lexer);
        let result = parser.parse().unwrap();
        assert_eq!(
            result.blocks.get(1),
            Some(&NexusBlock::TreesBlock(HashMap::from([
                (
                    "some very arbitrary text & some weird symbols cool:",
                    "Apes"
                ),
                ("some other arbitrary text + some other symbols-", "Humans"),
                ("Gorilla", "Gorilla"),
                ("Schimpansen", "Chimpanz''ee"),
            ])))
        );
    }

    #[test]
    fn test_duplicate_translations() {
        let text = "#NEXUS
        BEGIN taxa;
            DIMENSIONS 2;
            TAXLABELS Apes Humans;
        END;

        BEGIN trees;
            Translate
                Affen Apes,
                Affen Humans;
        END;";
        let lexer = Lexer::new(text);
        let mut parser = Parser::new(lexer);
        assert_eq!(parser.parse(), Err(ParsingError::DuplicateTranslations));
    }

    #[test]
    fn test_multiple_translations_for_taxa() {
        let text = "#NEXUS
        BEGIN taxa;
            DIMENSIONS 2;
            TAXLABELS Apes Humans;
        END;

        BEGIN trees;
            Translate
                Affen Apes,
                Affen2 Apes,
                Menschen Humans;
        END;";
        let lexer = Lexer::new(text);
        let mut parser = Parser::new(lexer);
        assert_eq!(parser.parse(), Err(ParsingError::DuplicateTranslations));
    }

    #[test]
    fn test_translations_for_unknown_taxa() {
        let text = "#NEXUS
        BEGIN taxa;
            DIMENSIONS 2;
            TAXLABELS Apes Humans;
        END;

        BEGIN trees;
            Translate Gorillas Gorillas;
        END;";
        let lexer = Lexer::new(text);
        let mut parser = Parser::new(lexer);
        assert_eq!(parser.parse(), Err(ParsingError::TranslationForUnknownTaxa));
    }
}
