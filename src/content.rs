use std::str::Chars;

struct Parsed<'a, T> {
    result: T,
    iter: Chars<'a>,
}

#[derive(Debug)]
pub enum ParsingError<'a> {
    ExpectedTokenNotFoundError(Chars<'a>, String),
}

trait Parsable {
    fn parse(iter: Chars) -> Result<Parsed<&Self>, ParsingError>;
}

struct Word {
    word: String,
}

impl Parsable for Word {
    fn parse(iter: Chars) -> Result<Parsed<&Self>, ParsingError> {
        let new_iter = iter.cloned();
        iter.collect()
    }
}
