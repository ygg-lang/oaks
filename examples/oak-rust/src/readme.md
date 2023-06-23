# 🛠️ Rust Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-rust`.

## 🚦 Quick Start

### Basic Parsing Example

The following is a standard workflow for parsing a Rust function with traits and generics:

```rust
use oak_rust::{RustParser, SourceText, RustLanguage};

fn main() {
    // 1. Prepare source code
    let code = r#"
        pub trait Summary {
            fn summarize(&self) -> String;
        }

        pub struct Article {
            pub headline: String,
            pub content: String,
        }

        impl Summary for Article {
            fn summarize(&self) -> String {
                format!("{}...", &self.headline[0..20])
            }
        }
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser
    let config = RustLanguage::new();
    let parser = RustParser::new(&config);

    // 3. Execute parsing
    let result = parser.parse(&source);

    // 4. Handle results
    if result.is_success() {
        println!("Parsing successful! AST node count: {}", result.node_count());
    } else {
        eprintln!("Errors found during parsing.");
    }
}
```

## 🔍 Core API Usage

### 1. Syntax Tree Traversal
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract Rust constructs like trait definitions, struct fields, or async blocks.

### 2. Incremental Parsing
No need to re-parse the entire crate when small changes occur in a file:
```rust
// Assuming you have an old parse result 'old_result' and new source text 'new_source'
let new_result = parser.reparse(&new_source, &old_result);
```

### 3. Diagnostics
`oak-rust` provides rich error contexts specifically tailored for Rust developers, handling complex scenarios like lifetime mismatches or macro expansion errors:
```rust
for diag in result.diagnostics() {
    println!("[{}:{}] {}", diag.line, diag.column, diag.message);
}
```

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes Rust source text into a stream of tokens, handling keywords, operators, literals, and raw strings.
- **Parser**: Syntax analyzer based on the Pratt parsing algorithm to handle Rust's expression precedence, complex generic syntax, and macro invocations.
- **AST**: A strongly-typed syntax abstraction layer designed for high-performance Rust analysis tools, linters, and IDEs.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/](tests/) for handling of various Rust edge cases and compiler versions.
