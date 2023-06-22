# Oak Rust Parser

[![Crates.io](https://img.shields.io/crates/v/oak-rust.svg)](https://crates.io/crates/oak-rust)
[![Documentation](https://docs.rs/oak-rust/badge.svg)](https://docs.rs/oak-rust)

A high-performance incremental Rust parser built on the Oak framework, providing full Rust syntax analysis, code formatting, and syntax highlighting.

## 🎯 Overview

Oak Rust is a powerful parser designed specifically for the Rust language, supporting full Rust syntax including modern language features. Built on the solid foundation of oak-core, it provides high-level convenience and detailed AST generation suitable for static analysis, code generation, formatting, and syntax highlighting.

## ✨ Key Features

- **Full Rust Syntax Support**: Supports all Rust language features, including modern specifications.
- **Complete AST Generation**: Generates a comprehensive abstract syntax tree.
- **Lexer**: Built-in tokenization with accurate position information.
- **Syntax Highlighting**: Supports highlighting for keywords, strings, numbers, comments, macros, etc.
- **Code Formatting**: Provides code formatting compliant with official style guides.
- **Error Recovery**: Gracefully handles syntax errors and provides detailed diagnostic information.
- **Incremental Parsing**: Based on the Oak framework's incremental parsing capabilities for efficient code analysis.

## 🚀 Quick Start

### Basic Parsing Example

```rust
use oak_rust::{RustLanguage, RustParser};
use oak_core::language::Language;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let language = RustLanguage::new();
    let parser = RustParser::new();
    
    let source = r#"
        fn main() {
            let x = 42;
            println!("Hello, Rust! x = {}", x);
        }
    "#;
    
    let result = language.parse(source);
    match result {
        Ok(ast) => println!("Parsing successful: {:?}", ast),
        Err(errors) => println!("Parsing error: {:?}", errors),
    }
    Ok(())
}
```

### Syntax Highlighting Example

```rust
use oak_rust::RustHighlighter;
use oak_highlight::highlighter::Highlighter;

fn main() {
    let highlighter = RustHighlighter::new();
    let code = r#"
        fn fibonacci(n: u32) -> u32 {
            match n {
                0 => 0,
                1 => 1,
                _ => fibonacci(n - 1) + fibonacci(n - 2),
            }
        }
    "#;
    
    let highlights = highlighter.highlight(code);
    for (start, end, kind) in highlights {
        println!("Highlight range: {}..{}, Type: {:?}", start, end, kind);
    }
}
```

### Code Formatting Example

```rust
use oak_rust::RustFormatter;

fn main() {
    let formatter = RustFormatter::new();
    let unformatted_code = "fn main(){let x=42;println!(\"x={}\",x);}";
    
    let formatted = formatter.format(unformatted_code);
    println!("Formatted code:\n{}", formatted);
}
```

## 📋 Parsing Examples

### Function Parsing
```rust
use oak_rust::{RustLanguage, RustParser};

let language = RustLanguage::new();
let source = r#"
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
"#;
```
