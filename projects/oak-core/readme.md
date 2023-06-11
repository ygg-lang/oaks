# Oak Core

[![Crates.io](https://img.shields.io/crates/v/oak-core.svg)](https://crates.io/crates/oak-core)
[![Documentation](https://docs.rs/oak-core/badge.svg)](https://docs.rs/oak-core)

The foundational parser combinator library providing core primitives for building robust parsers in Rust.

## 🎯 Overview

oak-core is the heart of the Pex ecosystem, offering a comprehensive set of parser combinator primitives that form the building blocks for all language parsers in the collection. It provides both high-level convenience functions and low-level control for custom parsing needs.

## ✨ Features

- **Zero-copy Parsing**: Return slices of input without unnecessary allocations
- **No-std Support**: Works in embedded and kernel environments
- **Composable**: Combine simple parsers into complex ones
- **Error Recovery**: Graceful handling of malformed input
- **Streaming**: Support for incremental parsing of large inputs
- **Type Safe**: Leverage Rust's type system for parser correctness

## 🚀 Quick Start

Basic example:

```rust
use oak_core::{Parser, combinator::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse a simple identifier
    let mut parser = letter().and(many(alphanumeric()));
    let result = parser.parse("hello123")?;
    println!("Parsed: {:?}", result);
    Ok(())
}
```

## 📋 Parser Combinators

### Basic Parsers

- `any()` - Parse any single character
- `letter()` - Parse alphabetic characters
- `digit()` - Parse numeric digits
- `whitespace()` - Parse whitespace
- `string("text")` - Parse literal strings
- `char('c')` - Parse specific characters

### Combinators

- `parser1.and(parser2)` - Sequential parsing
- `parser1.or(parser2)` - Alternative parsing
- `many(parser)` - Zero or more repetitions
- `some(parser)` - One or more repetitions
- `optional(parser)` - Optional parsing
- `between(open, close)` - Parse between delimiters

### Advanced Features

```rust
use oak_core::{Parser, combinator::*, error::ParseError};

// Custom parser with error handling
fn identifier() -> impl Parser<Output = String> {
    letter()
        .and(many(alphanumeric().or(char('_'))))
        .map(|(first, rest)| format!("{}{}", first, rest.into_iter().collect::<String>()))
}

// Recursive parser for nested structures
fn nested_parser() -> impl Parser<Output = Vec<char>> {
    between(char('['), char(']'))
        .and(many(any()))
        .map(|(_, chars)| chars)
}
```

## 🔧 Advanced Usage

### Custom Error Types

```rust
use oak_core::error::{ParseError, ParseResult};

#[derive(Debug, Clone)]
enum MyError {
    InvalidToken(String),
    UnexpectedEof,
}

impl ParseError for MyError {
    fn from_unexpected(input: &str, expected: &str) -> Self {
        MyError::InvalidToken(format!("Expected {}, got {}", expected, input.chars().next().unwrap_or('EOF')))
    }
}
```

### Zero-allocation Parsing

```rust
use oak_core::combinator::*;

// Return slices of input for zero-copy parsing
fn parse_word(input: &str) -> ParseResult<&str, MyError> {
    let (rest, word) = many1(letter()).parse(input)?;
    Ok((rest, word))
}
```

## 🏗️ Integration

oak-core is designed to integrate seamlessly with the broader Pex ecosystem:

- Language parsers built on oak-core
- Consistent error handling across all parsers
- Shared utilities and helpers
- Common parsing patterns

## 📊 Performance

- **Zero-copy**: Minimize allocations by returning slices
- **Streaming**: Handle large files without loading entirely into memory
- **Lazy**: Evaluate parsers only when needed
- **Optimized**: Fast path for common parsing scenarios

## 🔗 Related Crates

- [pex](../oaks) - Main unified library
- [oak-rust](../../examples/oak-rust) - Rust language parser
- [oak-json](../oak-json) - JSON parser
- [oak-highlight](../oak-highlight) - Syntax highlighter

## 📚 Examples

Check out the [examples](examples/) directory for more comprehensive usage examples:

- Basic parser combinators
- Custom error handling
- Streaming parsers
- Performance optimizations

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

---

**Pex Core** - Building the foundation for robust parsing in Rust 🦀