use std::fs;

use lexer::{Lexer, Tokens};
use parser::Parser;

mod lexer;
mod misc_parser_tests;
mod nexus;
mod parser;
mod taxa_parser_tests;
mod translations_parser_tests;
mod tree;
mod trees_parser_tests;

pub fn parse_file(path: &str) {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");

    let lexer = Lexer::new(&contents);
    let tokens = Tokens::new(&lexer);

    let mut parser = Parser::new(tokens);
    let result = parser.parse().unwrap();

    for block in result.blocks {
        if let nexus::NexusBlock::TreesBlock(_, trees) = block {
            println!("{}", trees.len())
        }
    }
}
