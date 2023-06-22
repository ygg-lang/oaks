# Oak C# Parser

[![Crates.io](https://img.shields.io/crates/v/oak-csharp.svg)](https://crates.io/crates/oak-csharp)
[![Documentation](https://docs.rs/oak-csharp/badge.svg)](https://docs.rs/oak-csharp)

High-performance incremental C# parser for the oak ecosystem with flexible configuration, optimized for code analysis and compilation.

## 🎯 Overview

Oak C# is a robust parser for C#, designed to handle complete C# syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for static analysis and code generation.

## ✨ Features

- **Complete C# Syntax**: Supports all C# features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_csharp::{CSharpParser, CSharpLanguage};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut session = ParseSession::<CSharpLanguage>::default();
    let parser = CSharpParser::new();
    let source = SourceText::new(r#"
using System;

class Program {
    static void Main() {
        Console.WriteLine("Hello, C#!");
    }
}
    "#);
    
    let result = parser.parse(&[], &mut session);
    println!("Parsed C# successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Class Parsing
```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_csharp::{CSharpParser, CSharpLanguage};

let mut session = ParseSession::<CSharpLanguage>::default();
let parser = CSharpParser::new();
let source = SourceText::new(r#"
public class Calculator {
    public int Add(int a, int b) {
        return a + b;
    }
    
    public int Subtract(int a, int b) {
        return a - b;
    }
}
"#);

let result = parser.parse(&[], &mut session);
println!("Class parsed successfully.");
```

### Interface Parsing
```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_csharp::{CSharpParser, CSharpLanguage};

let mut session = ParseSession::<CSharpLanguage>::default();
let parser = CSharpParser::new();
let source = SourceText::new(r#"
public interface IDrawable {
    void Draw();
    double Area { get; }
}

public class Circle : IDrawable {
    public double Radius { get; set; }
    
    public void Draw() {
        Console.WriteLine($"Drawing circle with radius {Radius}");
    }
    
    public double Area => Math.PI * Radius * Radius;
}
"#);

let result = parser.parse(&[], &mut session);
println!("Interface parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_csharp::{CSharpParser, CSharpLanguage};

let mut session = ParseSession::<CSharpLanguage>::default();
let parser = CSharpParser::new();
let source = SourceText::new("int x = 42;");
let result = parser.parse(&[], &mut session);
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_csharp::{CSharpParser, CSharpLanguage};

let mut session = ParseSession::<CSharpLanguage>::default();
let parser = CSharpParser::new();
let source = SourceText::new(r#"
public class Calculator {
    public int Add(int a, int b) {
        return a + b
    }
}
# Missing semicolon
"#);

let result = parser.parse(&[], &mut session);
if let Some(errors) = result.result.err() {
    println!("Parse errors found: {:?}", errors);
} else {
    println!("Parsed successfully.");
}
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **CompilationUnit**: Root container for C# programs
- **ClassDeclaration**: C# class definitions
- **InterfaceDeclaration**: Interface definitions
- **MethodDeclaration**: Method declarations
- **PropertyDeclaration**: Property declarations
- **Statement**: Various statement types
- **Expression**: Various expression types

## 📊 Performance

- **Streaming**: Parse large C# files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak C# integrates seamlessly with:

- **Static Analysis**: Code quality and security analysis
- **Code Generation**: Generating code from C# AST
- **IDE Support**: Language server protocol compatibility
- **Refactoring**: Automated code refactoring
- **Documentation**: Generating documentation from C# code

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete C# program parsing
- Class and interface analysis
- Code transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-csharp) or open [issues](https://github.com/ygg-lang/oaks/issues).