# Oak Python Parser

[![Crates.io](https://img.shields.io/crates/v/oak-python.svg)](https://crates.io/crates/oak-python)
[![Documentation](https://docs.rs/oak-python/badge.svg)](https://docs.rs/oak-python)

High-performance incremental Python parser for the oak ecosystem with flexible configuration, optimized for static analysis and code generation.

## 🎯 Overview

Oak Python is a robust parser for Python, designed to handle complete Python syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for static analysis and code generation.

## ✨ Features

- **Complete Python Syntax**: Supports all Python features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_python::{Parser, PythonLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
def greet(name):
    print(f"Hello, {name}!")

greet("World")
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed Python successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Function Parsing
```rust
use oak_python::{Parser, PythonLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
def add(a, b):
    return a + b
"#);

let result = parser.parse(&source);
println!("Function parsed successfully.");
```

### Class Parsing
```rust
use oak_python::{Parser, PythonLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
class Person:
    def __init__(self, name):
        self.name = name
"#);

let result = parser.parse(&source);
println!("Class parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_python::{Parser, PythonLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("x = 42");
let result = parser.parse(&source);
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_python::{Parser, PythonLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
def greet(name)
    print(f"Hello, {name}!")
# Missing colon
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

- **Module**: Root container for Python programs
- **Function**: Python functions and methods
- **Class**: Python class definitions
- **Statement**: Various statement types (assignment, if, loop, etc.)
- **Expression**: Various expression types (binary, unary, call, etc.)

## 📊 Performance

- **Streaming**: Parse large Python files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak Python integrates seamlessly with:

- **Static Analysis**: Code quality and security analysis
- **Code Generation**: Generating code from Python AST
- **IDE Support**: Language server protocol compatibility
- **Refactoring**: Automated code refactoring
- **Documentation**: Generating documentation from Python code

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete Python program parsing
- Function and class analysis
- Code transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-python) or open [issues](https://github.com/ygg-lang/oaks/issues).