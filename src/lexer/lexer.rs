use crate::lexer::tokens::Token;
use logos::Logos;
use std::ops::Range;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos()]
pub enum LexerToken {
    #[token(";")]
    EOS,
    #[regex(r"\[#(?P<comment>[^\]]*)\]")]
    Comment,
    #[regex(r"[\x00-\x06\t\n ]+")]
    Whitespace,
    #[regex(r"[()\[\]{}\/\\,:=*'`<>~]")]
    Punctuation,
    #[regex(r"[^\x00-\x06\t\n ()\[\]{}\/\\,;:=*'`<>~]+")]
    Word,
    #[regex(r"'(?:[^']|'')*'")]
    QuotedWord,
    #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", priority = 3)]
    // priority has to be higher than the one for Word
    Number,
}

pub struct Lexer<'a> {
    content: &'a str,
    tokens: Vec<Token<'a>>,
    ranges: Vec<Range<usize>>,
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a str) -> Self {
        let lexer = LexerToken::lexer(content).spanned();
        let mut tokens = vec![];
        let mut ranges = vec![];

        for (result, range) in lexer {
            let slice = &content[range.clone()];

            let token = match result {
                Ok(lexer_token) => match lexer_token {
                    LexerToken::EOS => Token::EOS,
                    LexerToken::Comment => Token::Comment(
                        &content[range.clone()].trim_matches(|c| c == '[' || c == '#' || c == ']'),
                    ),
                    LexerToken::Whitespace => Token::Whitespace(slice),
                    LexerToken::Punctuation => Token::Punctuation(slice),
                    LexerToken::Word => Token::Word(slice),
                    LexerToken::QuotedWord => Token::QuotedWord(&slice[1..slice.len() - 1]),
                    LexerToken::Number => {
                        if let Ok(number) = &slice.parse::<i32>() {
                            Token::Integer(*number)
                        } else {
                            Token::Float(slice.parse::<f64>().unwrap())
                        }
                    }
                },
                Err(_) => {
                    dbg!(tokens);
                    panic!("Tokenization failed.")
                }
            };

            tokens.push(token);
            ranges.push(range);
        }

        Self {
            content,
            tokens,
            ranges,
        }
    }

    pub fn get(&self, index: usize) -> Option<&Token> {
        self.tokens.get(index)
    }
    pub fn slice_from_to(&self, from_token: usize, to_token: usize) -> &'a str {
        let start = self.ranges.get(from_token).unwrap().start;
        let end = self.ranges.get(to_token - 1).unwrap().end;
        &self.content[start..end]
    }
    pub fn slice(&self, token: usize) -> &'a str {
        let start = self.ranges.get(token).unwrap().start;
        let end = self.ranges.get(token).unwrap().end;
        &self.content[start..end]
    }
}
