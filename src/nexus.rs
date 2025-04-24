use crate::parser::ParsingError;

#[derive(PartialEq, Debug)]
pub enum NexusBlock<'a> {
    TaxaBlock(usize, Vec<&'a str>),
    TreesBlock,
}

impl<'a> NexusBlock<'a> {
    pub fn build_taxa_block(
        dimensions: usize,
        tax_labels: Vec<&'a str>,
    ) -> Result<NexusBlock, ParsingError> {
        if dimensions != tax_labels.len() {
            Err(ParsingError::TaxaDimensionsMismatch)
        } else {
            Ok(NexusBlock::TaxaBlock(dimensions, tax_labels))
        }
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