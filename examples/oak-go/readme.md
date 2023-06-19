# Oak Go Parser

[![Crates.io](https://img.shields.io/crates/v/oak-go.svg)](https://crates.io/crates/oak-go)
[![Documentation](https://docs.rs/oak-go/badge.svg)](https://docs.rs/oak-go)

High-performance incremental Go parser for the oak ecosystem with flexible configuration, optimized for static analysis and code generation.

## 🎯 Overview

Oak Go is a robust parser for Go, designed to handle complete Go syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for static analysis and code generation.

## ✨ Features

- **Complete Go Syntax**: Supports all Go features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_go::{Parser, GoLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
package main

import "fmt"

func main() {
    fmt.Println("Hello, Go!")
}
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed Go successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Function Parsing
```rust
use oak_go::{Parser, GoLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
func add(a, b int) int {
    return a + b
}
"#);

let result = parser.parse(&source);
println!("Function parsed successfully.");
```

### Struct Parsing
```rust
use oak_go::{Parser, GoLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
type Person struct {
    Name string
    Age  int
}

func (p *Person) Greet() {
    fmt.Printf("Hello, I'm %s and I'm %d years old\n", p.Name, p.Age)
}
"#);

let result = parser.parse(&source);
println!("Struct parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_go::{Parser, GoLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("x := 42");
let result = parser.parse(&source);
// Token information is available in the parse result
```

### Error Handling
```rust
use oak_go::{Parser, GoLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
func main() {
    fmt.Println("Hello, Go!")
// Missing closing brace
"#);

let result = parser.parse(&source);
if let Err(e) = result.result {
    println!("Parse error: {:?}", e);
}
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **Package**: Root container for Go programs
- **Function**: Go functions and methods
- **Struct**: Go struct definitions
- **Interface**: Go interface definitions
- **Statement**: Various statement types
- **Expression**: Various expression types

## 📊 Performance

- **Streaming**: Parse large Go files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak Go integrates seamlessly with:

- **Static Analysis**: Code quality and security analysis
- **Code Generation**: Generating code from Go AST
- **IDE Support**: Language server protocol compatibility
- **Refactoring**: Automated code refactoring
- **Documentation**: Generating documentation from Go code

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete Go program parsing
- Function and struct analysis
- Code transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-go) or open [issues](https://github.com/ygg-lang/oaks/issues).