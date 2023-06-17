# Oak C# Parser

[![Crates.io](https://img.shields.io/crates/v/oak-csharp.svg)](https://crates.io/crates/oak-csharp)
[![Documentation](https://docs.rs/oak-csharp/badge.svg)](https://docs.rs/oak-csharp)

High-performance incremental C# parser for the oak ecosystem with flexible configuration, optimized for code analysis and compilation.

## 🎯 Overview

Oak-csharp is a robust parser for C#, designed to handle complete C# syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for static analysis and code generation.

## ✨ Features

- **Complete C# Syntax**: Supports all C# features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start


## 📋 Parsing Examples

### Class Parsing
```rust
use oak_csharp::{CsharpParser, ast::ClassDefinition};

let parser = CsharpParser::new();
let csharp_code = r#"
public class Calculator {
    public int Add(int a, int b) {
        return a + b;
    }
}
"#;

let program = parser.parse_program(csharp_code)?;
if let Some(ClassDefinition { name, .. }) = program.classes.get(0) {
    println!("Parsed class: {}", name);
}
```

### Method Parsing
```rust
use oak_csharp::{CsharpParser, ast::MethodDefinition};

let parser = CsharpParser::new();
let csharp_code = r#"
public static string Greet(string name) {
    return $"Hello, {name}!";
}
"#;

let method = parser.parse_method(csharp_code)?;
println!("Method name: {}", method.name);
```toml
[dependencies]
oak-csharp = "0.1"
```

Basic example:

```rust
use oak_csharp::CsharpParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = CsharpParser::new();
    let csharp_code = r#"
using System;

class Program {
    static void Main() {
        Console.WriteLine("Hello, C#!");
    }
}
"#;
    
    let program = parser.parse_program(csharp_code)?;
    println!("Parsed C# program successfully.");
    Ok(())
}
```

## Advanced Features

### Customizing the Parser

The `oak` library allows for flexible customization of the parser. You can modify the grammar rules or add new ones to suit your specific needs. Refer to the `oak` documentation for more details on parser customization.

### Error Recovery

`Oak of csharp` can be extended with error recovery mechanisms to handle malformed C# code gracefully, allowing for partial parsing and better resilience in real-world scenarios.

## AST Structure

The generated AST for C# code provides a hierarchical representation of the source. For instance, a simple class definition might result in an AST structure similar to this:

```rust
// Simplified AST representation for:
// public class MyClass { /* ... */ }
pex_csharp::ast::Node::ClassDefinition {
    modifiers: vec![
        pex_csharp::ast::Modifier::Public,
    ],
    name: "MyClass".to_string(),
    members: vec![
        // ... method definitions, field definitions, etc.
    ],
}
```

## Performance

`Oak of csharp` is designed for performance. Benchmarks show efficient parsing of large C# codebases. Optimizations include memoization, efficient backtracking, and direct AST construction.

## 🔗 Integration

Oak-csharp integrates seamlessly with:

- **Static Analysis**: Code quality and security analysis
- **Code Generation**: Generating code from C# AST
- **IDE Support**: Language server protocol compatibility
- **Refactoring**: Automated code refactoring
- **Documentation**: Generating documentation from C# code

## Examples

Explore the `examples` directory within the `oak-csharp` project for more usage examples and demonstrations of specific C# language features being parsed.

## Contributing

Contributions to `Oak of csharp` are welcome! If you find a bug or have a feature request, please open an issue on the GitHub repository. For major changes, please open a discussion first.