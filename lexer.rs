//! Token types and lexer for tokenizing input strings.

use crate::{errors::Error, parser::Parser};

/// Represents multiple token's type.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    /// Represents number type with type `f64``.
    Number(f64),
    /// Represents operator `+`.
    Plus,
    /// Represents operator `-`.
    Minus,
    /// Represents operator `*`.
    Multiply,
    /// Represents operator `/`.
    Divide,
    /// Represents open parenthesis `(`.
    OpenParen,
    /// Represents closing parenthesis `)`.
    CloseParen,
    /// Represents ending's token. At actuall momemt not implemented yet.
    EOF,
}

/// Represents a single lexical token with its type, raw text, and position.
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub position: usize,
}

/// Holds validated input and current position for tokenization.
#[derive(Default)]
pub struct Lexer {
    pub validate_input: Vec<char>,
    pub position: usize,
}

/// Converts the validated char vector into a vector of [`Token`]s.
///
/// Uses a sliding window algorithm to detect numbers and operators.
/// Numbers are parsed via [`Parser::parse_number`].
///
/// # Errors
///
/// Returns [`Error::TrailingSeparator`] if a number ends with a decimal point.
impl Lexer {
    pub fn tokenize(&self) -> Result<Vec<Token>, Error> {
        let mut token: Vec<Token> = Vec::new();

        let mut left_pointer: usize = 0;
        let mut right_pointer: usize = 0;

        loop {
            match self.validate_input.get(left_pointer) {
                Some(number)
                    if number.is_ascii_digit()
                        || self.is_unary_minus(&self.validate_input, left_pointer)
                        || number == &'.' =>
                {
                    left_pointer += 1;
                }
                Some(oper) if !oper.is_ascii_digit() => {
                    if left_pointer > right_pointer {
                        let to_parse = &self.validate_input[right_pointer..left_pointer];
                        let num = Parser::parse_number(&to_parse)?;

                        let token_num = Token {
                            token_type: TokenType::Number(num),
                            lexeme: to_parse.iter().collect::<String>(),
                            position: right_pointer,
                        };

                        token.push(token_num);
                    }
                    let token_operation_type = match oper {
                        '+' => TokenType::Plus,
                        '-' => TokenType::Minus,
                        '*' => TokenType::Multiply,
                        '/' => TokenType::Divide,
                        '(' => TokenType::OpenParen,
                        ')' => TokenType::CloseParen,
                        _ => TokenType::EOF,
                    };

                    let token_operation = Token {
                        token_type: token_operation_type,
                        lexeme: oper.to_string(),
                        position: left_pointer,
                    };

                    token.push(token_operation);

                    left_pointer += 1;
                    right_pointer = left_pointer;
                }

                None => {
                    if left_pointer > right_pointer {
                        let to_parse =
                            &self.validate_input[right_pointer..self.validate_input.len()];
                        let num = Parser::parse_number(&to_parse)?;
                        let token_num = Token {
                            token_type: TokenType::Number(num),
                            lexeme: to_parse.iter().collect::<String>(),
                            position: right_pointer,
                        };
                        token.push(token_num);
                    }
                    break;
                }
                Some(_) => unreachable!(),
            }
        }

        Ok(token)
    }

    /// Returns `true` if the character at `pos` is a unary minus.
    ///
    /// A minus is unary if it appears at the start of the input or
    /// immediately after an operator or opening parenthesis.
    fn is_unary_minus(&self, input: &[char], pos: usize) -> bool {
        input[pos] == '-'
            && match pos.checked_sub(1).and_then(|i| input.get(i)) {
                None | Some('+') | Some('-') | Some('*') | Some('/') | Some('(') | Some(')') => {
                    true
                }
                _ => false,
            }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod tokenize {
        use super::*;

        #[test]
        fn test_tokenize() {
            let input = Lexer {
                validate_input: vec!['1', '+', '2', '3'],
                position: 0,
            };
            let res = input.tokenize().unwrap();

            let types: Vec<TokenType> = res.iter().map(|t| t.token_type).collect();
            let lexeme: Vec<String> = res.iter().map(|t| t.lexeme.clone()).collect();
            let position: Vec<usize> = res.iter().map(|t| t.position).collect();

            dbg!(&types);
            dbg!(&lexeme);
            dbg!(&position);
            assert_eq!(
                types,
                vec![
                    TokenType::Number(1.0),
                    TokenType::Plus,
                    TokenType::Number(23.0)
                ]
            );
            assert_eq!(lexeme, vec!["1", "+", "23"]);
            assert_eq!(position, vec![0, 1, 2]);
        }

        #[test]
        fn negative_number() {
            let input = Lexer {
                validate_input: vec!['-', '1'],
                position: 0,
            };
            let res = input.tokenize().unwrap();

            let types: Vec<TokenType> = res.iter().map(|t| t.token_type).collect();
            let lexeme: Vec<String> = res.iter().map(|t| t.lexeme.clone()).collect();
            let position: Vec<usize> = res.iter().map(|t| t.position).collect();

            dbg!(&types);
            dbg!(&lexeme);
            dbg!(&position);
            assert_eq!(types, vec![TokenType::Number(-1.0)]);
            assert_eq!(lexeme, vec!["-1"]);
            assert_eq!(position, vec![0]);
        }

        #[test]
        fn float_number() {
            let input = Lexer {
                validate_input: vec!['1', '.', '5'],
                position: 0,
            };
            let res = input.tokenize().unwrap();

            let types: Vec<TokenType> = res.iter().map(|t| t.token_type).collect();
            let lexeme: Vec<String> = res.iter().map(|t| t.lexeme.clone()).collect();
            let position: Vec<usize> = res.iter().map(|t| t.position).collect();

            dbg!(&types);
            dbg!(&lexeme);
            dbg!(&position);
            assert_eq!(types, vec![TokenType::Number(1.5)]);
            assert_eq!(lexeme, vec!["1.5"]);
            assert_eq!(position, vec![0]);
        }

        #[test]
        fn negative_float_number() {
            let input = Lexer {
                validate_input: vec!['-', '1', '.', '5'],
                position: 0,
            };
            let res = input.tokenize().unwrap();

            let types: Vec<TokenType> = res.iter().map(|t| t.token_type).collect();
            let lexeme: Vec<String> = res.iter().map(|t| t.lexeme.clone()).collect();
            let position: Vec<usize> = res.iter().map(|t| t.position).collect();

            dbg!(&types);
            dbg!(&lexeme);
            dbg!(&position);
            assert_eq!(types, vec![TokenType::Number(-1.5)]);
            assert_eq!(lexeme, vec!["-1.5"]);
            assert_eq!(position, vec![0]);
        }

        #[test]
        fn negative_number_in_expression() {
            let input = Lexer {
                validate_input: vec!['5', '+', '-', '3'],
                position: 0,
            };
            let res = input.tokenize().unwrap();

            let types: Vec<TokenType> = res.iter().map(|t| t.token_type).collect();
            let lexeme: Vec<String> = res.iter().map(|t| t.lexeme.clone()).collect();
            let position: Vec<usize> = res.iter().map(|t| t.position).collect();

            dbg!(&types);
            dbg!(&lexeme);
            dbg!(&position);
            assert_eq!(
                types,
                vec![
                    TokenType::Number(5.0),
                    TokenType::Plus,
                    TokenType::Number(-3.0)
                ]
            );
            assert_eq!(lexeme, vec!["5", "+", "-3"]);
            assert_eq!(position, vec![0, 1, 2]);
        }

        #[test]
        fn negative_number_in_expression_with_minus() {
            let input = Lexer {
                validate_input: vec!['5', '-', '-', '3'],
                position: 0,
            };
            let res = input.tokenize().unwrap();

            let types: Vec<TokenType> = res.iter().map(|t| t.token_type).collect();
            let lexeme: Vec<String> = res.iter().map(|t| t.lexeme.clone()).collect();
            let position: Vec<usize> = res.iter().map(|t| t.position).collect();

            dbg!(&types);
            dbg!(&lexeme);
            dbg!(&position);
            assert_eq!(
                types,
                vec![
                    TokenType::Number(5.0),
                    TokenType::Minus,
                    TokenType::Number(-3.0)
                ]
            );
            assert_eq!(lexeme, vec!["5", "-", "-3"]);
            assert_eq!(position, vec![0, 1, 2]);
        }

        #[test]
        fn negative_number_after_open_paren() {
            let input = Lexer {
                validate_input: vec!['(', '-', '3', ')'],
                position: 0,
            };
            let res = input.tokenize().unwrap();

            let types: Vec<TokenType> = res.iter().map(|t| t.token_type).collect();
            dbg!(&types);
            assert_eq!(
                types,
                vec![
                    TokenType::OpenParen,
                    TokenType::Number(-3.0),
                    TokenType::CloseParen
                ]
            );
        }
    }
}
