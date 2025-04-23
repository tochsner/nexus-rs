use regex::Regex;

#[derive(Debug, PartialEq)]
enum Token<'a> {
    EOS,
    Comment(&'a str),
    Whitespace(&'a str),
    Punctuation(&'a str),
    Word(&'a str),
}

struct Lexer<'a> {
    content: &'a str,
    cursor: usize,
}

impl<'a> Lexer<'a> {
    fn new(content: &'a str) -> Self {
        Self { content, cursor: 0 }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.content.len() <= self.cursor {
            return None;
        }

        let context = &self.content[self.cursor..];

        let eos_regex = Regex::new(r"^;").unwrap();
        if let Some(res) = eos_regex.find(&context) {
            self.cursor += res.len();
            return Some(Token::EOS);
        };

        let comment_regex = Regex::new(r"^\[#(?P<comment>[^\]]*)\]").unwrap();
        if let Some(res) = comment_regex.captures(&context) {
            self.cursor += res.get(0).unwrap().len();
            return Some(Token::Comment(res.name("comment").unwrap().as_str()));
        };

        let whitespace_regex = Regex::new(r"^[\x00-\x06\t\n ]+").unwrap();
        if let Some(res) = whitespace_regex.find(&context) {
            self.cursor += res.len();
            return Some(Token::Whitespace(res.as_str()));
        };

        let punctuation_regex: Regex = Regex::new(r"^[()\[\]{}\/\\,;:=*'`<>~]").unwrap();
        if let Some(res) = punctuation_regex.find(&context) {
            self.cursor += res.len();
            return Some(Token::Punctuation(res.as_str()));
        };

        let word_regex: Regex =
            Regex::new(r"^[^?!.*\x00-\x06\t\n ()\[\]{}\/\\,;:=*'`<>~]*").unwrap();
        if let Some(res) = word_regex.find(&context) {
            self.cursor += res.len();
            return Some(Token::Word(res.as_str()));
        };

        None
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
