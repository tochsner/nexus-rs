use crate::errors::TokenizationError;
use std::str;

pub fn read_expected_token(
    mut iter: str::Chars,
    expected: String,
) -> Result<str::Chars, TokenizationError> {
    let initial_iter = iter.clone();

    for character in expected.chars() {
        if iter.next() != Some(character) {
            return Err(TokenizationError::ExpectedTokenNotFoundError(
                initial_iter,
                expected,
            ));
        }
    }

    Ok(iter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_expected_token_if_expected_token_is_present() {
        let text = String::from("this is some text");

        let iter = text.chars();
        let iter = read_expected_token(iter, String::from("this ")).unwrap();
        let iter = read_expected_token(iter, String::from("is some ")).unwrap();

        assert_eq!(iter.as_str(), "text");
    }

    #[test]
    fn test_read_expected_token_if_expected_token_is_not_present() {
        let text = String::from("this is some text");

        let iter = text.chars();
        let result = read_expected_token(iter, String::from("some other text")).unwrap_err();

        match result {
            TokenizationError::ExpectedTokenNotFoundError(iterator, _) => {
                assert_eq!(iterator.as_str(), "this is some text")
            }
        }
    }
}
