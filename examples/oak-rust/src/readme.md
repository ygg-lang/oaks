# 📖 Rust Parser User Guide

Rust support for the Oak language framework.

This guide helps you integrate `oak-rust` into your project and perform common parsing tasks efficiently.

## 🚀 Quick Start

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

## 🔍 Core Functionality

### 1. Syntax Tree Traversal
After a successful parse, use the built-in visitor pattern or manually traverse the Green/Red Tree to extract Rust constructs like trait definitions, struct fields, or async blocks.

### 2. Incremental Parsing
Optimize performance by only re-parsing changed sections:
```rust
// Assuming you have an old parse result 'old_result' and new source text 'new_source'
let new_result = parser.reparse(&new_source, &old_result);
```

### 3. Diagnostics & Error Recovery
`oak-rust` provides detailed error contexts tailored for Rust developers:
```rust
for diag in result.diagnostics() {
    println!("[{}:{}] {}", diag.line, diag.column, diag.message);
}
```

## 🛠️ Performance & Reliability

- **High-Fidelity AST**: Retains all trivia (whitespace and comments), making it ideal for code formatting and refactoring tools.
- **Fault Tolerance**: Automatically recovers from syntax errors to provide as much information as possible from the rest of the file.
- **Memory Efficiency**: Leverages immutable data structures (Green Trees) for low-overhead tree management.


