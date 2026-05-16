# Calculator

A simple arithmetic expression calculator written in Rust.

## Features

- Basic arithmetic: `+`, `-`, `*`, `/`
- Operator precedence and parentheses
- Floating point numbers and unary minus

## Usage

cargo run -- "2 + 3 * (4 - 1)"

## Architecture

- `validation` — input validation
- `lexer` — tokenization
- `parser` — number parsing
- `core` — AST construction and evaluation

## Roadmap

- [ ] Variables
- [ ] Functions (`sin`, `cos`, `sqrt`)
- [ ] Full interpreter
