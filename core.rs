//! Builds an AST from tokens and evaluates arithmetic expressions.

use crate::{
    errors::Error,
    lexer::{Token, TokenType},
};

/// Represents a node in the Abstract Syntax Tree (AST).
///
/// [`Node::Child`] holds an `f64` value.
/// [`Node::Parent`] consists of a [`TokenType`] and two [`Node`]s wrapped in [`Box`].
pub enum Node {
    /// Leaf node holding an `f64` value.
    Child(f64),
    /// Internal node consisting of a [`TokenType`] and two child [`Node`]s wrapped in [`Box`].
    Parent(TokenType, Box<Node>, Box<Node>),
}

impl Node {
    /// Builds an AST from a slice of [`Token`]s via [`Node::parse_expr`].
    pub fn build_tree(tokens: &[Token]) -> Node {
        let mut pos: usize = 0;

        Node::parse_expr(tokens, &mut pos)
    }

    /// Recursively evaluates the AST and returns the computed `f64` value.
    ///
    /// # Errors
    ///
    /// Returns [`Error::DivideByZero`] if divisible node equal `0`.
    pub fn eval(node: &Node) -> Result<f64, Error> {
        match node {
            Node::Child(n) => Ok(*n),
            Node::Parent(op, left, right) => {
                let l = Node::eval(left)?;
                let r = Node::eval(right)?;

                match op {
                    TokenType::Plus => Ok(l + r),
                    TokenType::Minus => Ok(l - r),
                    TokenType::Multiply => Ok(l * r),
                    TokenType::Divide => {
                        if r != 0.0 {
                            Ok(l / r)
                        } else {
                            return Err(Error::DivideByZero);
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    /// Parses a term: `factor (('*' | '/') factor)*`.
    ///
    /// Handles high-priority operators `*` and `/` with left associativity.
    fn parse_term(tokens: &[Token], pos: &mut usize) -> Node {
        let mut left = Node::parse_factor(tokens, pos);

        loop {
            match tokens.get(*pos).map(|t| &t.token_type) {
                Some(TokenType::Multiply) => {
                    *pos += 1;
                    let right = Node::parse_factor(tokens, pos);
                    left = Node::Parent(TokenType::Multiply, Box::new(left), Box::new(right));
                }
                Some(TokenType::Divide) => {
                    *pos += 1;
                    let right = Node::parse_factor(tokens, pos);
                    left = Node::Parent(TokenType::Divide, Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        left
    }

    /// Parses an expression: `term (('+' | '-') term)*`.
    ///
    /// Handles low-priority operators `+` and `-` with left associativity.
    fn parse_expr(tokens: &[Token], pos: &mut usize) -> Node {
        let mut left = Node::parse_term(tokens, pos);

        loop {
            match tokens.get(*pos).map(|t| &t.token_type) {
                Some(TokenType::Plus) => {
                    *pos += 1;
                    let right = Node::parse_term(tokens, pos);
                    left = Node::Parent(TokenType::Plus, Box::new(left), Box::new(right));
                }
                Some(TokenType::Minus) => {
                    *pos += 1;
                    let right = Node::parse_term(tokens, pos);
                    left = Node::Parent(TokenType::Minus, Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        left
    }

    /// Parses a primary expression: a number or a parenthesized expression.
    ///
    /// Advances `pos` past the number or the surrounding parentheses.
    fn parse_factor(tokens: &[Token], pos: &mut usize) -> Node {
        match tokens[*pos].token_type {
            TokenType::Number(n) => {
                *pos += 1;
                Node::Child(n)
            }
            _ => {
                *pos += 1;
                let node = Node::parse_expr(tokens, pos);
                *pos += 1;
                node
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;

    use super::*;

    #[test]
    fn test_eval() {
        let tokens = Lexer {
            validate_input: vec!['1', '+', '2', '*', '3'],
            position: 0,
        }
        .tokenize()
        .unwrap();
        let tree = Node::build_tree(&tokens);
        dbg!(Node::eval(&tree));
        assert_eq!(Node::eval(&tree).unwrap(), 7.0)
    }

    #[test]
    fn test_div_minus() {
        let tokens = Lexer {
            validate_input: vec!['2', '0', '/', '2', '-', '5'],
            position: 0,
        }
        .tokenize()
        .unwrap();
        let tree = Node::build_tree(&tokens);
        dbg!(Node::eval(&tree));
        assert_eq!(Node::eval(&tree).unwrap(), 5.0)
    }

    #[test]
    fn test_all_ops() {
        let tokens = Lexer {
            validate_input: vec!['2', '0', '+', '5', '-', '1', '0', '*', '2', '/', '6'],
            position: 0,
        }
        .tokenize()
        .unwrap();
        let tree = Node::build_tree(&tokens);
        dbg!(Node::eval(&tree));
        assert_eq!(Node::eval(&tree).unwrap(), 21.666666666666668)
    }

    #[test]
    fn test_parens() {
        let tokens = Lexer {
            validate_input: vec!['(', '2', '0', '+', '5', ')', '*', '2'],
            position: 0,
        }
        .tokenize()
        .unwrap();
        let tree = Node::build_tree(&tokens);
        dbg!(Node::eval(&tree));
        assert_eq!(Node::eval(&tree).unwrap(), 50.0)
    }

    #[test]
    fn test_nested_parens() {
        let tokens = Lexer {
            validate_input: vec!['(', '(', '2', '+', '3', ')', '*', '4', ')'],
            position: 0,
        }
        .tokenize()
        .unwrap();
        let tree = Node::build_tree(&tokens);
        assert_eq!(Node::eval(&tree).unwrap(), 20.0);
    }

    #[test]
    fn test_negative_in_expr() {
        let tokens = Lexer {
            validate_input: vec!['-', '5', '+', '3'],
            position: 0,
        }
        .tokenize()
        .unwrap();
        let tree = Node::build_tree(&tokens);
        assert_eq!(Node::eval(&tree).unwrap(), -2.0);
    }

    #[test]
    fn test_float_expr() {
        let tokens = Lexer {
            validate_input: vec!['1', '.', '5', '*', '2'],
            position: 0,
        }
        .tokenize()
        .unwrap();
        let tree = Node::build_tree(&tokens);
        assert_eq!(Node::eval(&tree).unwrap(), 3.0);
    }

    #[test]
    fn divider_zero() {
        let tokens = Lexer {
            validate_input: vec!['1', '.', '5', '/', '0'],
            position: 0,
        }
        .tokenize()
        .unwrap();
        let tree = Node::build_tree(&tokens);
        assert!(Node::eval(&tree).is_err());
    }
}
