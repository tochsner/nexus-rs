use std::{fmt::Display, iter::Peekable};

use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    EOS,
    Comment(&'a str),
    Whitespace(&'a str),
    Punctuation(&'a str),
    Word(&'a str),
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::EOS =>  write!(f, "EOS"),
            Token::Comment(comment) =>  write!(f, "Comment: {}", comment),
            Token::Whitespace(_) =>  write!(f, "Whitespace"),
            Token::Punctuation(punctuation) =>  write!(f, "Punctuation: {}", punctuation),
            Token::Word(word) =>  write!(f, "Word: {}", word),
        }
    }
}

pub struct Lexer<'a> {
    content: &'a str,
    cursor: usize,

    eos_regex: Regex,
    comment_regex: Regex,
    whitespace_regex: Regex,
    punctuation_regex: Regex,
    word_regex: Regex,
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a str) -> Self {
        Self {
            content,
            cursor: 0,
            eos_regex: Regex::new(r"^;").unwrap(),
            comment_regex: Regex::new(r"^\[#(?P<comment>[^\]]*)\]").unwrap(),
            whitespace_regex: Regex::new(r"^[\x00-\x06\t\n ]+").unwrap(),
            punctuation_regex: Regex::new(r"^[()\[\]{}\/\\,;:=*'`<>~]").unwrap(),
            word_regex: Regex::new(r"^[^?!.*\x00-\x06\t\n ()\[\]{}\/\\,;:=*'`<>~]+").unwrap(),
        }
    }

    fn next_token_and_cursor(&mut self) -> (Option<Token<'a>>, usize) {
        if self.content.len() <= self.cursor {
            return (None, self.cursor);
        }

        let context = &self.content[self.cursor..];

        if let Some(res) = self.eos_regex.find(&context) {
            return (Some(Token::EOS), self.cursor + res.len())
        };

        if let Some(res) = self.comment_regex.captures(&context) {
            return (Some(Token::Comment(res.name("comment").unwrap().as_str())), self.cursor + res.get(0).unwrap().len())
        };

        if let Some(res) = self.whitespace_regex.find(&context) {
            return (Some(Token::Whitespace(res.as_str())), self.cursor + res.len())
        };

        if let Some(res) = self.punctuation_regex.find(&context) {
            return (Some(Token::Punctuation(res.as_str())), self.cursor + res.len())
        };

        if let Some(res) = self.word_regex.find(&context) {
            return (Some(Token::Word(res.as_str())), self.cursor + res.len())
        };

        (None, self.cursor)
    }

    pub fn peek(&mut self) -> Option<Token> {
        self.next_token_and_cursor().0
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn slice(&self, from: usize) -> &'a str {
        &self.content[from..self.cursor]
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let (next_token, next_cursor) = self.next_token_and_cursor();
        self.cursor = next_cursor;
        next_token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new("#NEXUS;\nBEGIN TREES;   \t  word[#some comment()]other_word;");

        assert_eq!(lexer.next(), Some(Token::Word("#NEXUS")));
        assert_eq!(lexer.next(), Some(Token::EOS));
        assert_eq!(lexer.next(), Some(Token::Whitespace("\n")));
        assert_eq!(lexer.next(), Some(Token::Word("BEGIN")));
        assert_eq!(lexer.next(), Some(Token::Whitespace(" ")));
        assert_eq!(lexer.next(), Some(Token::Word("TREES")));
        assert_eq!(lexer.next(), Some(Token::EOS));
        assert_eq!(lexer.next(), Some(Token::Whitespace("   \t  ")));
        assert_eq!(lexer.next(), Some(Token::Word("word")));
        assert_eq!(lexer.next(), Some(Token::Comment("some comment()")));
        assert_eq!(lexer.next(), Some(Token::Word("other_word")));
        assert_eq!(lexer.next(), Some(Token::EOS));

        let mut lexer = Lexer::new(
            "#NEXUS
Begin Taxa;
Dimensions NTax=4;
End;
End;",
        );

        assert_eq!(lexer.next(), Some(Token::Word("#NEXUS")));
        assert_eq!(lexer.next(), Some(Token::Whitespace("\n")));
        assert_eq!(lexer.next(), Some(Token::Word("Begin")));
        assert_eq!(lexer.next(), Some(Token::Whitespace(" ")));
        assert_eq!(lexer.next(), Some(Token::Word("Taxa")));
        assert_eq!(lexer.next(), Some(Token::EOS));
        assert_eq!(lexer.next(), Some(Token::Whitespace("\n")));
        assert_eq!(lexer.next(), Some(Token::Word("Dimensions")));
        assert_eq!(lexer.next(), Some(Token::Whitespace(" ")));
        assert_eq!(lexer.next(), Some(Token::Word("NTax")));
        assert_eq!(lexer.next(), Some(Token::Punctuation("=")));
        assert_eq!(lexer.next(), Some(Token::Word("4")));
        assert_eq!(lexer.next(), Some(Token::EOS));
        assert_eq!(lexer.next(), Some(Token::Whitespace("\n")));
        assert_eq!(lexer.next(), Some(Token::Word("End")));
        assert_eq!(lexer.next(), Some(Token::EOS));
        assert_eq!(lexer.next(), Some(Token::Whitespace("\n")));
        assert_eq!(lexer.next(), Some(Token::Word("End")));
        assert_eq!(lexer.next(), Some(Token::EOS));
        assert_eq!(lexer.next(), None);
    }
}
