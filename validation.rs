//! Validate input and represent type of an error if occured
//! If validate succeed create new struct [`Lexer`]

use core::error;

use crate::{errors::Error, lexer::Lexer};

#[derive(Debug, Clone)]
pub struct Validation;

impl Validation {
    /// Validates a string slice through several stages and returns a [`Lexer`] on success.
    ///     
    /// # Errors
    ///
    /// Returns [`Error::EmptyParens`] if input contains only empty parenthesis `()`.
    /// Returns [`Error::InvalidCharacter`] if input contains non-numeric characters.
    /// Returns [`Error::NotValidData`] if provided data is invalid.
    /// Returns [`Error::UnmatchedOpenParen`] if an opening parenthesis has no matching closing parenthesis
    /// Returns [`Error::UnmatchedCloseParen`] if a closing parenthesis has no matching opening parenthesis.
    pub fn pre_validate(arguments: &str) -> Result<Lexer, Error> {
        let valid_symbols = vec!['+', '-', '*', '/', '.'];
        if arguments.contains("()") {
            return Err(Error::EmptyParens);
        }

        let input_to_char: Vec<_> = arguments
            .chars()
            .filter(|char| !char.is_whitespace())
            .collect();

        match input_to_char {
            _ if input_to_char.is_empty() => return Err(Error::EmptyInput),

            _ if input_to_char.iter().any(|char| {
                !char.is_numeric() && !valid_symbols.contains(&char) && !['(', ')'].contains(&char)
            }) =>
            {
                return Err(Error::InvalidCharacter);
            }

            _ => {}
        }

        if valid_symbols.contains(input_to_char.first().unwrap())
            || valid_symbols.contains(input_to_char.last().unwrap())
        {
            return Err(Error::NotValidData);
        }
        Validation::valid_parentheses(&input_to_char)?;

        for windows in input_to_char.windows(2) {
            match windows {
                ['+', '*']
                | ['+', '/']
                | ['*', '+']
                | ['*', '/']
                | ['/', '*']
                | ['/', '+']
                | ['-', '*']
                | ['-', '/']
                | ['-', '+'] => return Err(Error::NotValidData),
                ['-', '.']
                | ['.', '-']
                | ['.', '+']
                | ['+', '.']
                | ['/', '.']
                | ['.', '/']
                | ['.', '*']
                | ['*', '.'] => return Err(Error::NotValidData),

                [a, b] if a == b && ['+', '*', '/', '.'].contains(b) => {
                    return Err(Error::NotValidData);
                }
                _ => {}
            }
        }

        Ok(Lexer {
            validate_input: input_to_char,
            position: 0,
        })
    }

    /// Determine if parentheses are balanced.
    ///
    /// # Errors
    /// Returns [`Error::UnmatchedOpenParen`] if an opening parenthesis has no matching closing parenthesis.
    ///
    /// Returns [`Error::UnmatchedCloseParen`] if a closing parenthesis has no matching opening parenthesis.
    fn valid_parentheses(input: &[char]) -> Result<(), Error> {
        let mut stack: Vec<char> = Vec::new();

        for i in input {
            match i {
                '(' => stack.push('('),
                ')' => {
                    if stack.pop().is_none() {
                        return Err(Error::UnmatchedCloseParen);
                    }
                }
                _ => {}
            }
        }
        if stack.is_empty() {
            return Ok(());
        } else {
            return Err(Error::UnmatchedOpenParen);
        }
    }
}

#[cfg(test)]
mod test {

    mod pre_validate {
        use crate::validation::Validation;

        use super::*;

        #[test]
        fn empty_string() {
            let args = "";
            assert!(Validation::pre_validate(&args).is_err());
        }

        #[test]
        fn string_with_alpha() {
            let args = "a+5+8+9";
            assert!(Validation::pre_validate(&args).is_err());
        }

        #[test]
        fn test_invalid_operator_pairs() {
            let invalid_cases = [
                "+*", "*+", "6+*3", "-*", "7/*8", "7+5*/5", "+-/*", "5/*", "+/*", "/+", "/***",
                "/=",
            ];

            for case in &invalid_cases {
                assert!(Validation::pre_validate(case).is_err());
            }
        }

        #[test]
        fn test_double_dots_and_repeats() {
            let invalid_cases = [
                "//",
                "..",
                "++",
                "**",
                "************",
                "4..+",
                "9++",
                "7/5//9",
                "4**7",
                "==",
                "22-44++2",
            ];

            for case in &invalid_cases {
                assert!(
                    Validation::pre_validate(case).is_err(),
                    "Should fail for lone symbol: {:?}",
                    case
                );
            }
        }

        #[test]
        fn test_misplaced_equals() {
            let invalid_cases = ["=5+2", "2+2=4+4"];

            for case in &invalid_cases {
                assert!(
                    Validation::pre_validate(case).is_err(),
                    "Should fail for misplaced '=': {:?}",
                    case
                );
            }
        }

        #[test]
        fn test_trailing_operators() {
            let invalid_cases = ["5+", "10.", "-3*", "*5", "5 + 5/ ", "+10", "/10*11"];

            for case in &invalid_cases {
                assert!(
                    Validation::pre_validate(case).is_err(),
                    "Should fail for lone symbol: {:?}",
                    case
                );
            }
        }

        #[test]
        fn test_only_symbols() {
            let invalid_cases = [".", "-", "+", "="];

            for case in &invalid_cases {
                assert!(
                    Validation::pre_validate(case).is_err(),
                    "Should fail for lone symbol: {:?}",
                    case
                );
            }
        }

        #[test]
        fn valid_expression_with_operators() {
            let valid_cases = ["8 + 9.12 / 5", "1 + 2", "3*4", "10-5", "8/2"];

            for case in &valid_cases {
                assert!(
                    Validation::pre_validate(case).is_ok(),
                    "Should pass for: {:?}",
                    case
                );
            }
        }
    }

    mod valid_parentheses {
        use crate::validation::Validation;

        use super::*;

        #[test]
        fn valid_cases() {
            let valid_cases = vec![
                vec!['(', ')'],
                vec!['(', '(', ')', ')'],
                vec!['(', '1', '+', '2', ')'],
                vec!['(', '(', '1', '+', '2', ')', '*', '3', ')'],
            ];

            for case in &valid_cases {
                assert!(Validation::valid_parentheses(case).is_ok());
            }
        }
        #[test]
        fn unmatched_open_paren() {
            let invalid_cases = vec![vec!['(', '1', '+', '2'], vec!['(', '(', '1', '+', '2', ')']];

            for case in &invalid_cases {
                assert!(
                    Validation::valid_parentheses(case).is_err(),
                    "Should fail for: {:?}",
                    case
                );
            }
        }

        #[test]
        fn unmatched_close_paren() {
            let invalid_cases = vec![vec!['1', '+', '2', ')'], vec!['(', '1', '+', '2', ')', ')']];

            for case in &invalid_cases {
                assert!(
                    Validation::valid_parentheses(case).is_err(),
                    "Should fail for: {:?}",
                    case
                );
            }
        }

        #[test]
        fn interleaved_parens() {
            let invalid_cases = vec![vec![')', '('], vec![')', '1', '+', '2', '(']];

            for case in &invalid_cases {
                assert!(
                    Validation::valid_parentheses(case).is_err(),
                    "Should fail for: {:?}",
                    case
                );
            }
        }
    }
}
