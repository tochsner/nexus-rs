use std::fs;

use lexer::Lexer;
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

    println!("Lexing...");
    let lexer = Lexer::new(&contents);

    println!("Parsing...");
    let mut parser = Parser::new(lexer);
    let result = parser.parse().unwrap();

    println!("Parsing done.");
    println!("OK: {}", result.blocks.len());

    for block in result.blocks {
        if let nexus::NexusBlock::TreesBlock(_, trees) = block { println!("{}", trees.len()) }
    }
}
