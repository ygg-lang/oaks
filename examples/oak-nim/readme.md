# Oak Nim Parser

[![Crates.io](https://img.shields.io/crates/v/oak-nim.svg)](https://crates.io/crates/oak-nim)
[![Documentation](https://docs.rs/oak-nim/badge.svg)](https://docs.rs/oak-nim)

High-performance incremental Nim parser for the oak ecosystem with flexible configuration, optimized for systems programming and metaprogramming.

## 🎯 Overview

Oak Nim is a robust parser for Nim, designed to handle complete Nim syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for static analysis and code generation.

## ✨ Features

- **Complete Nim Syntax**: Supports all Nim features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_nim::{Parser, NimLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
proc add(a, b: int): int =
    result = a + b

echo "Hello, Nim!"
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed Nim successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Procedure Parsing
```rust
use oak_nim::{Parser, NimLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
proc factorial(n: int): int =
    if n <= 1:
        return 1
    else:
        return n * factorial(n - 1)
"#);

let result = parser.parse(&source);
println!("Procedure parsed successfully.");
```

### Object Parsing
```rust
use oak_nim::{Parser, NimLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
type
    Person = object
        name: string
        age: int
    
    Animal = ref object of RootObj
        species: string
        age: int
"#);

let result = parser.parse(&source);
println!("Object definitions parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_nim::{Parser, NimLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("let x = 42");
let result = parser.parse(&source);
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_nim::{Parser, NimLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
# Invalid Nim code example
proc broken_function(
    echo "Hello"
# Missing closing parenthesis and return
"#);

let result = parser.parse(&source);
if let Some(errors) = result.result.err() {
    println!("Parse errors found: {:?}", errors);
} else {
    println!("Parsed successfully.");
}
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **NimProgram**: Root container for Nim programs
- **Procedure**: Nim procedures and methods
- **Type**: Type definitions including objects and enums
- **Statement**: Various statement types including control flow
- **Expression**: Various expression types including operators
- **Module**: Module declarations and imports

## 📊 Performance

- **Streaming**: Parse large Nim files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak Nim integrates seamlessly with:

- **Static Analysis**: Code quality and security analysis
- **Code Generation**: Generating code from Nim AST
- **IDE Support**: Language server protocol compatibility
- **Refactoring**: Automated code refactoring
- **Documentation**: Generating documentation from Nim code

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete Nim program parsing
- Procedure and type analysis
- Code transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-nim) or open [issues](https://github.com/ygg-lang/oaks/issues).