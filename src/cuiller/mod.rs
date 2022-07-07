/*!
Provides functions and types to validate and generate QR codes.
*/

use regex::Regex;
use std::fmt::{self};

/// Identifies the type of string that was input.
#[derive(Debug, PartialEq, Eq)]
pub enum InputType {
    /// Input is digits only.
    Digits,
    /// Input is alphanumeric characters.
    Alphanumeric,
}

/// Error type for invalid inputs.
#[derive(Debug, PartialEq, Eq)]
pub enum InputError {
    EmptyString,
    InvalidString,
}

impl std::error::Error for InputError {}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputError::EmptyString => write!(f, "error: empty input"),
            InputError::InvalidString => {
                write!(f, "invalid input, expected : digits | alphanumeric")
            }
        }
    }
}

/// Checks if the input string is a valid QR-code input.
pub fn validate_input(text: &str) -> Result<InputType, InputError> {
    let sanitized = text.trim();

    if sanitized.is_empty() {
        return Err(InputError::EmptyString);
    }

    let digits = Regex::new(r"^[[:digit:]]+$").unwrap();

    if digits.is_match(sanitized) {
        return Ok(InputType::Digits);
    }

    let alphanum = Regex::new(r"^[[:alnum:]]+$").unwrap();

    if alphanum.is_match(sanitized) {
        return Ok(InputType::Alphanumeric);
    }

    Err(InputError::InvalidString)
}

#[cfg(test)]
mod test {

    mod validate_input {
        use crate::cuiller::{validate_input, InputError, InputType};

        #[test]
        fn it_should_fail_for_the_empty_string() {
            assert_eq!(validate_input(""), Err(InputError::EmptyString));
        }

        #[test]
        fn it_should_succeed_for_numeric_strings() {
            assert_eq!(
                validate_input("165109840130190916510490"),
                Ok(InputType::Digits)
            );
            assert_eq!(validate_input("0"), Ok(InputType::Digits));
            assert_eq!(validate_input("1"), Ok(InputType::Digits));
            assert_eq!(validate_input("1645098409406"), Ok(InputType::Digits));
            assert_eq!(validate_input("-1"), Err(InputError::InvalidString));
        }

        #[test]
        fn it_should_succeed_for_alphanumeric_strings() {
            assert_eq!(validate_input("h3ll0w0rld"), Ok(InputType::Alphanumeric));
            assert_eq!(
                validate_input("helloWorld!"),
                Err(InputError::InvalidString)
            );
        }
    }
}
