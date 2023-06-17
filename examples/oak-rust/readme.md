# Oak Rust Parser

[![Crates.io](https://img.shields.io/crates/v/oak-rust.svg)](https://crates.io/crates/oak-rust)
[![Documentation](https://docs.rs/oak-rust/badge.svg)](https://docs.rs/oak-rust)

High-performance incremental Rust parser for the oak ecosystem with flexible configuration, optimized for static analysis and code generation.

## 🎯 Overview

Oak Rust is a robust parser for Rust, designed to handle complete Rust syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for static analysis and code generation.

## ✨ Features

- **Complete Rust Syntax**: Supports all Rust features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_rust::{Parser, RustLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
        fn main() {
            println!("Hello, Rust!");
        }
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed Rust successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Function Parsing
```rust
use oak_rust::{Parser, RustLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
"#);

let result = parser.parse(&source);
println!("Parsed Rust function successfully.");
```

### Struct Parsing
```rust
use oak_rust::{Parser, RustLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
    struct Point {
        x: f64,
        y: f64,
    }
"#);

let result = parser.parse(&source);
println!("Parsed Rust struct successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_rust::{Parser, RustLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("let x = 42;");
let result = parser.parse(&source);
// Token information is available in the parse result
```

### Error Handling
```rust
use oak_rust::{Parser, RustLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
    fn main() {
        println!("Hello, Rust!")
    // Missing closing brace
"#);

let result = parser.parse(&source);
if let Err(e) = result.result {
    println!("Parse error: {:?}", e);
}
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **Crate**: Root container for Rust programs
- **Function**: Rust functions and methods
- **Struct**: Rust struct definitions
- **Enum**: Rust enum definitions
- **Impl**: Implementation blocks
- **Statement**: Various statement types
- **Expression**: Various expression types

## 📊 Performance

- **Streaming**: Parse large Rust files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak Rust integrates seamlessly with:

- **Static Analysis**: Code quality and security analysis
- **Code Generation**: Generating code from Rust AST
- **IDE Support**: Language server protocol compatibility
- **Refactoring**: Automated code refactoring
- **Documentation**: Generating documentation from Rust code

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete Rust program parsing
- Function and struct analysis
- Code transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-rust) or open [issues](https://github.com/ygg-lang/oaks/issues).