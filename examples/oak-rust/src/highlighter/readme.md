# Rust Syntax Highlighter

This module provides syntax highlighting for Rust source code, supporting various syntax elements of the Rust language.

## Overview

`RustHighlighter` is a syntax highlighter specifically designed for the Rust language. It implements the `Highlighter` trait and can recognize and highlight various syntax elements in Rust code, including keywords, string literals, number literals, comments, and macro calls.

## Core Components

### RustHighlighter

The main syntax highlighter struct, providing the following features:

- **Lexer-based Highlighting**: Fast basic syntax highlighting.
- **Parser-based Highlighting**: More accurate highlighting based on semantic analysis.
- **Multiple Syntax Element Support**: Keywords, strings, numbers, comments, macros, etc.

## Usage

### Basic Usage

```rust
use oak_rust::highlighter::RustHighlighter;
use oak_highlight::highlighter::Highlighter;

// Create a highlighter instance
let highlighter = RustHighlighter::new();

// Perform syntax highlighting on Rust code
let code = r#"
fn main() {
    let x = 42;
    println!("Hello, world! x = {}", x);
}
"#;

let highlights = highlighter.highlight(code);
```

### Using Parser Mode

```rust
// Create a highlighter with parser mode enabled
let highlighter = RustHighlighter::with_parser();
let highlights = highlighter.highlight(code);
```

## Supported Rust Language Features

### Keyword Highlighting

Supports highlighting for all Rust keywords, including:

- **Strict Keywords**: `as`, `break`, `const`, `continue`, `crate`, `else`, `enum`, `extern`, `false`, `fn`, `for`, `if`, `impl`, `in`, `let`, `loop`, `match`, `mod`, `move`, `mut`, `pub`, `ref`, `return`, `self`, `Self`, `static`, `struct`, `super`, `trait`, `true`, `type`, `unsafe`, `use`, `where`, `while`
- **Reserved Keywords**: `abstract`, `become`, `box`, `do`, `final`, `macro`, `override`, `priv`, `typeof`, `unsized`, `virtual`, `yield`
- **Weak Keywords**: `union`, `'static`
- **Edition-specific Keywords**: `async`, `await`, `dyn`, `try`

### String Literal Highlighting

Supports multiple string formats:

- **Normal Strings**: `"hello"`
- **Raw Strings**: `r"hello"`, `r#"hello"#`
- **Byte Strings**: `b"hello"`
- **Raw Byte Strings**: `br"hello"`, `br#"hello"#`
- **Character Literals**: `'a'`, `'\n'`
- **Byte Characters**: `b'a'`

### Number Literal Highlighting

Supports various number formats:

- **Integers**: `42`, `0x2A`, `0o52`, `0b101010`
- **Floats**: `3.14`, `1e10`, `1.0f32`
- **With Type Suffixes**: `42u32`, `3.14f64`
- **With Separators**: `1_000_000`

### Comment Highlighting

Supports Rust's comment formats:

- **Line Comments**: `// This is a line comment`
- **Block Comments**: `/* This is a block comment */`
- **Nested Block Comments**: `/* outer /* inner */ comment */`
- **Doc Comments**: `/// Doc comment`, `//! Module doc`

### Macro Call Highlighting

Identifies and highlights macro calls:

- **Function-like Macros**: `println!()`, `vec![]`
- **Attribute Macros**: `#[derive(Debug)]`
- **Custom Macros**: User-defined macro calls.

## Configuration Options

### Highlighting Modes
