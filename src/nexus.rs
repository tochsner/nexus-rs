use std::str::Chars;

use crate::{errors::TokenizationError, tokenizer::read_expected_token};

pub fn read_nexus<'a>(iter: Chars<'a>) -> Result<u32, TokenizationError<'a>> {
    let _ = read_expected_token(iter, String::from("#NEXUS"))?;

    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_nexus() {
        let content = String::from("#NEXUS\n<body>");
        let result = read_nexus(content.chars()).unwrap();

        assert_eq!(result, 0);

        let content = String::from("<only body>");
        let result = read_nexus(content.chars()).unwrap_err();

        match result {
            TokenizationError::ExpectedTokenNotFoundError(_, expected) => {
                assert_eq!(expected, "#NEXUS")
            }
        }
    }
}
