use std::collections::HashMap;

use crate::{parser::parser::ParsingError, types::tree::Tree};

#[derive(PartialEq, Debug)]
pub enum NexusBlock {
    TaxaBlock(usize, Vec<String>),
    TreesBlock(HashMap<String, String>, Vec<Tree>),
}

impl NexusBlock {
    pub fn build_taxa_block(
        dimensions: usize,
        tax_labels: Vec<String>,
    ) -> Result<NexusBlock, ParsingError> {
        if dimensions != tax_labels.len() {
            Err(ParsingError::TaxaDimensionsMismatch)
        } else {
            Ok(NexusBlock::TaxaBlock(dimensions, tax_labels))
        }
    }
    pub fn build_trees_block(
        translations: HashMap<String, String>,
        trees: Vec<Tree>,
    ) -> Result<NexusBlock, ParsingError> {
        // verify that we have at most one translation per taxa
        let mut unique_taxa_with_translation = translations.values().collect::<Vec<&String>>();
        unique_taxa_with_translation.sort();
        unique_taxa_with_translation.dedup();
        if translations.len() != unique_taxa_with_translation.len() {
            return Err(ParsingError::DuplicateTranslations);
        }

        // verify that all tree names are unique
        let mut unique_tree_names = trees.iter().map(|t| &t.name).collect::<Vec<&String>>();
        unique_tree_names.sort();
        unique_tree_names.dedup();
        if trees.len() != unique_tree_names.len() {
            return Err(ParsingError::DuplicateTreeNames);
        }

        Ok(NexusBlock::TreesBlock(translations, trees))
    }
}

#[derive(PartialEq, Debug)]
pub struct Nexus {
    pub blocks: Vec<NexusBlock>,
}

impl Nexus {
    pub fn build(blocks: Vec<NexusBlock>) -> Result<Self, ParsingError> {
        // let mut all_taxa: Vec<String> = vec![];
        // let mut all_translated_taxa: Vec<String> = vec![];
        // for block in &blocks {
        //     if let NexusBlock::TaxaBlock(_, taxa) = block {
        //         // all_taxa.extend(*taxa);
        //     } else if let NexusBlock::TreesBlock(translations, _) = block {
        //         all_translated_taxa.extend(translations.values().map(|t| t.to_string()));
        //     }
        // }

        // // verify that only known taxa have translations
        // let unknown_translated_taxa = all_translated_taxa
        //     .iter()
        //     .filter(|t| !all_taxa.contains(t))
        //     .collect::<Vec<_>>();
        // if !unknown_translated_taxa.is_empty() {
        //     dbg!(all_translated_taxa.clone());
        //     return Err(ParsingError::TranslationForUnknownTaxa);
        // }

        Ok(Nexus { blocks })
    }
}
