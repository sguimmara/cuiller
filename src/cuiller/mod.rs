use regex::Regex;
use std::fmt::{self};

#[derive(Debug, PartialEq, Eq)]
pub enum InputType {
    Digits,
    Alphanumeric,
}

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

pub fn validate_input(text: &str) -> Result<InputType, InputError> {
    let sanitized = text.trim();
    if sanitized.is_empty() {
        return Err(InputError::EmptyString);
    }

    let digits = Regex::new(r"^[[:digit:]]+$").unwrap();

    if digits.is_match(sanitized) {
        return Ok(InputType::Digits);
    }

    Err(InputError::InvalidString)
}

#[cfg(test)]
mod test {

    mod validate_input {
        use crate::cuiller::{validate_input, InputError, InputType};

        #[test]
        fn it_should_return_error_for_the_empty_string() {
            assert_eq!(validate_input(""), Err(InputError::EmptyString));
        }

        #[test]
        fn it_should_return_true_for_numeric_strings() {
            assert_eq!(
                validate_input("165109840130190916510490"),
                Ok(InputType::Digits)
            );
            assert_eq!(validate_input("0"), Ok(InputType::Digits));
            assert_eq!(validate_input("1"), Ok(InputType::Digits));
            assert_eq!(validate_input("1645098409406"), Ok(InputType::Digits));
            assert_eq!(validate_input("-1"), Err(InputError::InvalidString));
        }
    }
}
