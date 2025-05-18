#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{lexer::Lexer, nexus::NexusBlock, parser::Parser};

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
}
