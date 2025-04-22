use std::str::Chars;

use crate::characters::{is_character, is_punctuation, is_whitespace};

#[derive(Debug)]
pub enum Token {
    Punctuation(char),
    Word(String),
}

#[derive(Debug)]
pub struct Content<'a>(Chars<'a>);

impl Content<'_> {
    pub fn next(mut self) -> Option<char> {
        self.0.next()
    }

    pub fn parse_token(&self) -> Option<Token> {
        let mut without_whitespace = self.0.skip_while(is_whitespace).peekable();

        let first_character = without_whitespace.peek()?;

        if is_punctuation(&first_character) {
            return Some(Token::Punctuation(*first_character));
        }

        let token: String = without_whitespace.take_while(is_character).collect();

        if !token.ends_with("'") {
            return Some(Token::Word(token));
        }

        // we have the special case where two consecutive quotation marks
        // still count as a token (e.g. my''name is one token)

        Some(Token::Word(token))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_token() {
        let content_string = String::from("  token;  sda");
        let content = Content(content_string.chars());
        
        let token = content.parse_token().unwrap();
        match token {
            Token::Word(word) => assert_eq!(word, "token"),
            _ => panic!("No word detected."),
        }

        let token = content.parse_token().unwrap();
        match token {
            Token::Word(word) => assert_eq!(word, "token"),
            _ => panic!("No word detected."),
        }

        let content_string = String::from(" ;  token    sda");
        let content = Content(content_string.chars());
        match content.parse_token().unwrap() {
            Token::Punctuation(punctuation) => assert_eq!(punctuation, ';'),
            _ => panic!("No punctuation detected."),
        }
    }

    // #[test]
    // fn test_parse_token_with_quotation_marks() {
    //     let content_string = String::from("  'token';  sda");
    //     let content = Content(content_string.chars());

    //     match content.parse_token().unwrap() {
    //         Token::Word(word) => assert_eq!(word, "token"),
    //         _ => panic!("No word detected."),
    //     }

    //     let content_string = String::from("  'to''ken';  sda");
    //     let content = Content(content_string.chars());

    //     match content.parse_token().unwrap() {
    //         Token::Word(word) => assert_eq!(word, "token"),
    //         _ => panic!("No word detected."),
    //     }
    // }
}
