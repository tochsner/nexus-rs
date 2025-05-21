use logos::Logos;
use std::fmt::Display;

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
    #[regex(r"'(?:[^\x00-\x06\t\n ()\[\]{}\/\\,;:=*'`<>~]|'')*'")]
    QuotedWord,
    #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", priority = 3)]
    // priority has to be higher than the one for Word
    Number,
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos()]
pub enum Token<'a> {
    EOS,
    Comment(&'a str),
    Whitespace(&'a str),
    Punctuation(&'a str),
    Word(&'a str),
    QuotedWord(&'a str),
    Integer(i64),
    Float(f64),
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::EOS => write!(f, "EOS"),
            Token::Comment(comment) => write!(f, "Comment: {}", comment),
            Token::Whitespace(_) => write!(f, "Whitespace"),
            Token::Punctuation(punctuation) => write!(f, "Punctuation: {}", punctuation),
            Token::Word(word) => write!(f, "Word: {}", word),
            Token::QuotedWord(word) => write!(f, "Word: {}", word),
            Token::Integer(word) => write!(f, "Number: {}", word),
            Token::Float(word) => write!(f, "Number: {}", word),
        }
    }
}

pub struct Lexer<'a> {
    tokens: Vec<Token<'a>>,
}

impl Lexer<'_> {
    pub fn get(&self, index: usize) -> Option<&Token> {
        self.tokens.get(index)
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let lexer = LexerToken::lexer(input).spanned();
        let mut tokens = vec![];

        for (result, range) in lexer {
            let slice = &input[range.clone()];

            let token = match result {
                Ok(lexer_token) => match lexer_token {
                    LexerToken::EOS => Token::EOS,
                    LexerToken::Comment => Token::Comment(
                        &input[range.clone()].trim_matches(|c| c == '[' || c == '#' || c == ']'),
                    ),
                    LexerToken::Whitespace => Token::Whitespace(slice),
                    LexerToken::Punctuation => Token::Punctuation(slice),
                    LexerToken::Word => Token::Word(slice),
                    LexerToken::QuotedWord => Token::QuotedWord(&slice[1..slice.len() - 1]),
                    LexerToken::Number => {
                        if let Ok(number) = &slice.parse::<i64>() {
                            Token::Integer(*number)
                        } else {
                            Token::Float(slice.parse::<f64>().unwrap())
                        }
                    }
                },
                Err(_) => {
                    panic!("Tokenization failed.")
                }
            };

            tokens.push(token);
        }

        Self { tokens }
    }
}

pub struct Tokens<'a> {
    lexer: &'a Lexer<'a>,
    cursor: usize,
}

impl<'a> Tokens<'a> {
    pub fn new(lexer: &'a Lexer<'a>) -> Self {
        Tokens { lexer, cursor: 0 }
    }

    pub fn peek(&mut self) -> Option<&Token> {
        self.lexer.get(self.cursor())
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn set_cursor(&mut self, new_cursor: usize) {
        self.cursor = new_cursor;
    }

    pub fn slice_from(&self, from: usize) -> &'a str {
        todo!("Needs fixing");
        // &self.content[from..self.cursor()]
    }

    pub fn slice_from_to(&self, from: usize, to: usize) -> &'a str {
        todo!("Needs fixing");
        // &self.content[from..to]
    }
}

impl<'a> Iterator for Tokens<'a>
where
    Token<'a>: Clone,
{
    type Item = &'a Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let old_cursor = self.cursor();
        self.set_cursor(old_cursor + 1);
        self.lexer.get(old_cursor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let lexer = Lexer::new("#NEXUS;\nBEGIN TREES;   \t  word[#some comment()]other_word;");
        let mut tokens = Tokens::new(&lexer);

        assert_eq!(tokens.next(), Some(&Token::Word("#NEXUS")));
        assert_eq!(tokens.next(), Some(&Token::EOS));
        assert_eq!(tokens.next(), Some(&Token::Whitespace("\n")));
        assert_eq!(tokens.next(), Some(&Token::Word("BEGIN")));
        assert_eq!(tokens.next(), Some(&Token::Whitespace(" ")));
        assert_eq!(tokens.next(), Some(&Token::Word("TREES")));
        assert_eq!(tokens.next(), Some(&Token::EOS));
        assert_eq!(tokens.next(), Some(&Token::Whitespace("   \t  ")));
        assert_eq!(tokens.next(), Some(&Token::Word("word")));
        assert_eq!(tokens.next(), Some(&Token::Comment("some comment()")));
        assert_eq!(tokens.next(), Some(&Token::Word("other_word")));
        assert_eq!(tokens.next(), Some(&Token::EOS));

        let lexer = Lexer::new(
            "#NEXUS
Begin Taxa;
Dimensions NTax=4;
End;
End;",
        );
        let mut tokens = Tokens::new(&lexer);

        assert_eq!(tokens.next(), Some(&Token::Word("#NEXUS")));
        assert_eq!(tokens.next(), Some(&Token::Whitespace("\n")));
        assert_eq!(tokens.next(), Some(&Token::Word("Begin")));
        assert_eq!(tokens.next(), Some(&Token::Whitespace(" ")));
        assert_eq!(tokens.next(), Some(&Token::Word("Taxa")));
        assert_eq!(tokens.next(), Some(&Token::EOS));
        assert_eq!(tokens.next(), Some(&Token::Whitespace("\n")));
        assert_eq!(tokens.next(), Some(&Token::Word("Dimensions")));
        assert_eq!(tokens.next(), Some(&Token::Whitespace(" ")));
        assert_eq!(tokens.next(), Some(&Token::Word("NTax")));
        assert_eq!(tokens.next(), Some(&Token::Punctuation("=")));
        assert_eq!(tokens.next(), Some(&Token::Integer(4)));
        assert_eq!(tokens.next(), Some(&Token::EOS));
        assert_eq!(tokens.next(), Some(&Token::Whitespace("\n")));
        assert_eq!(tokens.next(), Some(&Token::Word("End")));
        assert_eq!(tokens.next(), Some(&Token::EOS));
        assert_eq!(tokens.next(), Some(&Token::Whitespace("\n")));
        assert_eq!(tokens.next(), Some(&Token::Word("End")));
        assert_eq!(tokens.next(), Some(&Token::EOS));
        assert_eq!(tokens.next(), None);
    }
}
