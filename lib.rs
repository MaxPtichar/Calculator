pub mod core;
pub mod errors;
pub mod lexer;
pub mod parser;
pub mod validation;

pub use core::Node;
pub use errors::Error;
pub use validation::Validation;

pub fn calculate(input: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let valid_string = Validation::pre_validate(input)?;

    let tokens = valid_string.tokenize()?;

    let tree = Node::build_tree(&tokens);

    let result = Node::eval(&tree)?;

    Ok(result)
}

pub fn run(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    if args.is_empty() {
        return Err(Box::new(Error::EmptyInput));
    }
    if args.len() > 1 {
        return Err(Box::new(Error::TooManyArgs));
    }

    let input = &args[0];

    let result = calculate(&input)?;

    println!("Output: {}", result);

    Ok(())
}
