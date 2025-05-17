use std::collections::HashMap;

use crate::parser::ParsingError;

#[derive(PartialEq, Debug)]
pub enum NexusBlock<'a> {
    TaxaBlock(usize, Vec<&'a str>),
    TreesBlock(HashMap<&'a str, &'a str>),
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
    ) -> Result<NexusBlock<'a>, ParsingError> {
        Ok(NexusBlock::TreesBlock(translations))
    }
}

#[derive(PartialEq, Debug)]
pub struct Nexus<'a> {
    pub blocks: Vec<NexusBlock<'a>>,
}

impl<'a> Nexus<'a> {
    pub fn new() -> Self {
        Nexus { blocks: vec![] }
    }
}
