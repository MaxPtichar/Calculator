//! Error types for the calculator parser and evaluator.
//!
//! All errors implement [`fmt::Display`] and [`std::error::Error`],
//! so they can be used with standart `?` operator and error-handling
//! libraries.

use std::{
    ffi::os_str::Display,
    fmt::{self, write},
};

/// Represents all errors that can occur during validation, parsing and evaluation,
/// used across the following modules: core, lexer, lib, parser, validation.
#[derive(Debug)]
pub enum Error {
    /// Returned when received more than one arguments in the commandline.
    TooManyArgs,
    /// Returned when the provided data is invalid.
    NotValidData,
    /// Returned when user entered empty input.
    EmptyInput,
    /// Returned during validations when input contains non-numeric characters
    InvalidCharacter,
    /// Returned during parsing stage when vector have multiple dots.
    MultipleDots,
    /// Returned when first or last element of chars vector have operator or decimal point.
    TrailingSeparator,
    /// Returned after validations on parenthesis when missing closing parenthesis.
    UnmatchedOpenParen,
    /// Returned after validations on parenthesis when missing open parenthesis.
    UnmatchedCloseParen,
    /// Returned when input contain only an empty parenthesis.
    EmptyParens,
    /// Returned when divisor equal to zero.
    DivideByZero,
}

/// Implementation for each [`Error`] with description.
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Error::TooManyArgs => "Too many arguments. Use double quotes for the expression.",
            Error::NotValidData => "The provided data format is invalid",
            Error::EmptyInput => "No input provided. Please wrap your expression in quotes",
            Error::InvalidCharacter => "Input contains non-numeric characters",
            Error::MultipleDots => "Number cannot have more than one decimal point",
            Error::TrailingSeparator => "Number cannot end with a decimal point",
            Error::UnmatchedOpenParen => "Missing closing parenthesis",
            Error::UnmatchedCloseParen => "Unexpected closing parenthesis",
            Error::EmptyParens => "Empty parentheses are not allowed",
            Error::DivideByZero => "Cannot divide by zero",
        };

        write!(f, "Parsing error: {}", message)
    }
}

impl std::error::Error for Error {}
