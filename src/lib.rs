use std::fs;

use lexer::Lexer;
use parser::Parser;

mod lexer;
mod misc_parser_tests;
mod nexus;
mod parser;
mod taxa_parser_tests;
mod trees_parser_tests;

pub fn parse_file(path: &str) {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");

    let lexer = Lexer::new(&contents);
    let mut parser = Parser::new(lexer);
    let result = parser.parse().unwrap();
    print!("OK: {}", result.blocks.len());
}
