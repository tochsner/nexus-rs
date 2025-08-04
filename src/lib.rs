use std::fs;

pub use lexer::{lexer::Lexer, tokens::Tokens};
pub use parser::parser::Parser;
pub use types::nexus::{Nexus, NexusBlock};

mod lexer;
mod parser;
mod types;

pub fn parse_file(path: &str) -> Nexus {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");

    let lexer = Lexer::new(&contents);
    let tokens = Tokens::new(&lexer);

    let mut parser = Parser::new(tokens);
    let result = parser.parse().unwrap();

    result
}
