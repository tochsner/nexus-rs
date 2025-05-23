use std::fs;

use lexer::{lexer::Lexer, tokens::Tokens};
use parser::parser::Parser;
use types::nexus::NexusBlock;

mod lexer;
mod parser;
mod types;

pub fn parse_file(path: &str) {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");

    let lexer = Lexer::new(&contents);
    let tokens = Tokens::new(&lexer);

    let mut parser = Parser::new(tokens);
    let result = parser.parse().unwrap();

    for block in result.blocks {
        if let NexusBlock::TreesBlock(_, trees) = block {
            println!("{}", trees.len())
        }
    }
}
