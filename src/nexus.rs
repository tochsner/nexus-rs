use std::collections::HashMap;

use crate::{parser::ParsingError, tree::Tree};

#[derive(PartialEq, Debug)]
pub enum NexusBlock<'a> {
    TaxaBlock(usize, Vec<&'a str>),
    TreesBlock(HashMap<&'a str, &'a str>, Vec<Tree<'a>>),
}

impl<'a> NexusBlock<'a> {
    pub fn build_taxa_block(
        dimensions: usize,
        tax_labels: Vec<&'a str>,
    ) -> Result<NexusBlock<'a>, ParsingError> {
        if dimensions != tax_labels.len() {
            Err(ParsingError::TaxaDimensionsMismatch)
        } else {
            Ok(NexusBlock::TaxaBlock(dimensions, tax_labels))
        }
    }
    pub fn build_trees_block(
        translations: HashMap<&'a str, &'a str>,
        trees: Vec<Tree<'a>>,
    ) -> Result<NexusBlock<'a>, ParsingError> {
        // verify that we have at most one translation per taxa
        let mut unique_taxa_with_translation = translations.values().collect::<Vec<&&str>>();
        unique_taxa_with_translation.sort();
        unique_taxa_with_translation.dedup();
        if translations.len() != unique_taxa_with_translation.len() {
            return Err(ParsingError::DuplicateTranslations);
        }

        Ok(NexusBlock::TreesBlock(translations, trees))
    }
}

#[derive(PartialEq, Debug)]
pub struct Nexus<'a> {
    pub blocks: Vec<NexusBlock<'a>>,
}

impl<'a> Nexus<'a> {
    pub fn build(blocks: Vec<NexusBlock<'a>>) -> Result<Self, ParsingError> {
        let mut all_taxa: Vec<&'a str> = vec![];
        let mut all_translated_taxa: Vec<&'a str> = vec![];
        for block in &blocks {
            if let NexusBlock::TaxaBlock(_, taxa) = block {
                all_taxa.extend(taxa);
            }

            if let NexusBlock::TreesBlock(translations, _) = block {
                all_translated_taxa.extend(translations.values());
            }
        }

        // verify that only known taxa have translations
        let unknown_translated_taxa = all_translated_taxa
            .iter()
            .filter(|t| !all_taxa.contains(t))
            .collect::<Vec<_>>();
        if !unknown_translated_taxa.is_empty() {
            dbg!(all_translated_taxa.clone());
            return Err(ParsingError::TranslationForUnknownTaxa);
        }

        Ok(Nexus { blocks })
    }
}
