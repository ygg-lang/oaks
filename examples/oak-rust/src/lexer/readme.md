# Rust Lexer

This module provides lexical analysis functionality for the Rust programming language, converting source text into a stream of tokens.

## Overview

The Lexer is the first stage of the compiler frontend, responsible for breaking down raw source text into meaningful token sequences. This module is specifically designed for the Rust language, supporting all of Rust's syntax elements.

## Core Components

### Token Types (`RustTokenType`)

Defines all possible token types in the Rust language:

- **Whitespace**: `Space`, `Newline`
- **Delimiters**: `LeftParen`, `RightParen`, `LeftBracket`, `RightBracket`, `LeftBrace`, `RightBrace`
- **Punctuation**: `Semicolon`, `Comma`, `Dot`, `Colon`, `DoubleColon`, `Question`, `At`, `Hash`, `Dollar`
- **Operators**: `Plus`, `Minus`, `Star`, `Slash`, `Percent`, `Caret`, `Ampersand`, `Pipe`, `Tilde`, `Bang`, `Eq`, `Lt`, `Gt`
- **Compound Operators**: `PlusEq`, `MinusEq`, `StarEq`, `SlashEq`, `PercentEq`, `EqEq`, `Ne`, `Le`, `Ge`, `AndAnd`, `OrOr`, `Shl`, `Shr`
- **Rust-specific Operators**: `DotDot`, `DotDotEq`, `Arrow`, `FatArrow`
- **Keywords**: `Keyword(RustKeywords)`
- **Identifiers and Literals**: `Identifier`, `IntegerLiteral`, `FloatLiteral`, `StringLiteral`, `CharLiteral`, `BoolLiteral`
- **Comments**: `LineComment`, `BlockComment`, `DocComment`
- **Special Tokens**: `Error`, `Eof`, `Lifetime`

### Keywords (`RustKeywords`)

Contains all keyword types in the Rust language:

- **Strict Keywords**: `as`, `break`, `const`, `continue`, `crate`, `else`, `enum`, `extern`, `false`, `fn`, `for`, `if`, `impl`, `in`, `let`, `loop`, `match`, `mod`, `move`, `mut`, `pub`, `ref`, `return`, `self`, `Self`, `static`, `struct`, `super`, `trait`, `true`, `type`, `unsafe`, `use`, `where`, `while`
- **Reserved Keywords**: `abstract`, `become`, `box`, `do`, `final`, `macro`, `override`, `priv`, `typeof`, `unsized`, `virtual`, `yield`
- **Weak Keywords**: `async`, `await`, `dyn`, `try`, `union`
- **Edition-specific Keywords**: `raw`

### Lexer (`RustLexer`)

The main lexer implementation, providing the following features:

- **Whitespace Handling**: Recognizes and skips spaces, tabs, newlines, etc.
- **Comment Handling**: Supports line comments (`//`) and block comments (`/* */`), including nested block comments.
- **String Literals**: Supports normal strings, raw strings, and byte strings.
- **Character Literals**: Supports characters and byte characters.
- **Number Literals**: Supports integers and floats, including type suffixes and base prefixes.
- **Identifiers and Keywords**: Uses Unicode standards to identify identifiers.
- **Operators**: Supports all Rust operators, including compound assignment operators.
- **Lifetimes**: Recognizes lifetime parameters (`'a`, `'static`, etc.).

## Usage

The lexer is typically used through `RustLexer` during Rust source code parsing:

```rust
use oak_rust::{RustLexer, RustLanguage};
use oak_core::source::StringSource;

let language = RustLanguage::default();
let lexer = RustLexer::new(&language);
let source = StringSource::new("fn main() { let x = 42; }");

// Lexical analysis will be performed automatically during parsing
```

## Rust Language Feature Support

The lexer supports all major features of the Rust language:

- **Raw Strings**: `r"string"`, `r#"string"#`, `r##"string"##`
- **Byte Strings**: `b"bytes"`, `br"raw bytes"`
- **Character Literals**: `'a'`, `'\n'`, `'\u{1F600}'`
- **Number Literals**: `42`, `3.14`, `0x1A`, `0o755`, `0b1010`, `42u32`, `3.14f64`
- **Lifetimes**: `'a`, `'static`, `'_`
- **Nested Block Comments**: `/* outer /* inner */ outer */`
- **Doc Comments**: `///`, `//!`, `/** */`, `/*! */`

## Error Handling

The lexer provides comprehensive error handling:

- **Invalid Characters**: Identifies invalid characters in the source code.
- **Unterminated Strings**: Detects string literals that are not correctly closed.
- **Invalid Number Formats**: Identifies malformed number literals.
- **Position Information**: Provides precise source code positions for all errors.

## Performance Features

- **Incremental Lexing**: Supports incremental updates, re-analyzing only changed parts.
- **Zero-copy**: Avoids string copying whenever possible.
- **Unicode Support**: Full support for Unicode identifiers and strings.
- **Memory Efficiency**: Optimized memory usage patterns.

## Design Principles

1. **Completeness**: Supports full Rust syntax specifications.
2. **Accuracy**: Precisely matches the lexical analysis behavior of the Rust compiler.
3. **Performance**: Efficient lexical analysis algorithms.
4. **Extensibility**: Easy to add support for new language features.
