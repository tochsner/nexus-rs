use std::str::Chars;

#[derive(Debug)]
pub enum TokenizationError<'a> {
    ExpectedTokenNotFoundError(Chars<'a>, String)
}
