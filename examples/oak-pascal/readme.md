# Oak Pascal Parser

[![Crates.io](https://img.shields.io/crates/v/oak-pascal.svg)](https://crates.io/crates/oak-pascal)
[![Documentation](https://docs.rs/oak-pascal/badge.svg)](https://docs.rs/oak-pascal)

High-performance incremental Pascal parser for the oak ecosystem with flexible configuration, optimized for legacy code analysis and educational purposes.

## 🎯 Overview

Oak Pascal is a robust parser for Pascal, designed to handle complete Pascal syntax including legacy and modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for code analysis and educational purposes.

## ✨ Features

- **Complete Pascal Syntax**: Supports all Pascal features including legacy specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_pascal::{Parser, PascalLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
program HelloWorld;
begin
    writeln('Hello, World!');
end.
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed Pascal successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Program Parsing
```rust
use oak_pascal::{Parser, PascalLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
program Calculator;
uses
    SysUtils;
var
    x, y: integer;
begin
    x := 10;
    y := 20;
    writeln(x + y);
end.
"#);

let result = parser.parse(&source);
println!("Program parsed successfully.");
```

### Procedure Parsing
```rust
use oak_pascal::{Parser, PascalLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
procedure Greet(name: string);
begin
    writeln('Hello, ', name);
end;
"#);

let result = parser.parse(&source);
println!("Procedure parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_pascal::{Parser, PascalLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("program Test; begin end.");
let result = parser.parse(&source);
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_pascal::{Parser, PascalLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
program Test
begin
    writeln('Missing semicolon')
end.
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

- **PascalProgram**: Root container for Pascal programs
- **Procedure**: Procedure declarations
- **Function**: Function declarations
- **Variable**: Variable declarations with types
- **Statement**: Various statement types (assignment, if, while, for, etc.)
- **Expression**: Arithmetic, logical, and relational expressions

## 📊 Performance

- **Streaming**: Parse large Pascal files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak Pascal integrates seamlessly with:

- **Legacy Code Analysis**: Analyze and understand legacy Pascal codebases
- **Educational Tools**: Build programming language learning platforms
- **IDE Support**: Language server protocol compatibility
- **Documentation Tools**: Extract documentation from Pascal source code
- **Migration Tools**: Convert Pascal code to other languages

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete Pascal program parsing
- Procedure and function analysis
- Variable and type extraction
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-pascal) or open [issues](https://github.com/ygg-lang/oaks/issues).