# Oak Pascal Parser

[![Crates.io](https://img.shields.io/crates/v/oak-pascal.svg)](https://crates.io/crates/oak-pascal)
[![Documentation](https://docs.rs/oak-pascal/badge.svg)](https://docs.rs/oak-pascal)

High-performance incremental Pascal parser for the oak ecosystem with flexible configuration, optimized for legacy code analysis and educational purposes.

## 🎯 Overview

Oak of pascal is a robust parser for Pascal, designed to handle complete Pascal syntax including legacy and modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for code analysis and educational purposes.

## ✨ Features

- **Complete Pascal Syntax**: Supports all Pascal features including legacy specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_pascal::PascalParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = PascalParser::new();
    let pascal_code = r#"
program HelloWorld;
begin
    writeln('Hello, World!');
end.
    "#;
    
    let program = parser.parse_program(pascal_code)?;
    println!("Parsed Pascal program successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Program Parsing
```rust
use oak_pascal::{PascalParser, ast::Program};

let parser = PascalParser::new();
let pascal_code = r#"
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
"#;

let program = parser.parse_program(pascal_code)?;
println!("Program name: {}", program.name);
```

### Procedure Parsing
```rust
use oak_pascal::{PascalParser, ast::Procedure};

let parser = PascalParser::new();
let procedure_code = r#"
procedure Greet(name: string);
begin
    writeln('Hello, ', name);
end;
"#;

let procedure = parser.parse_procedure(procedure_code)?;
println!("Procedure name: {}", procedure.name);
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_pascal::{PascalParser, lexer::Token};

let parser = PascalParser::new();
let tokens = parser.tokenize("program Test; begin end.")?;
for token in tokens {
    println!("{:?}", token.kind);
}
```

### Error Handling
```rust
use oak_pascal::PascalParser;

let parser = PascalParser::new();
let invalid_pascal = r#"
program Test
begin
    writeln('Missing semicolon')
end.
"#;

match parser.parse_program(invalid_pascal) {
    Ok(program) => println!("Parsed Pascal program successfully."),
    Err(e) => {
        println!("Parse error at line {} column {}: {}", 
            e.line(), e.column(), e.message());
        if let Some(context) = e.context() {
            println!("Error context: {}", context);
        }
    }
}
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **Program**: Root container for Pascal programs
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

Oak of pascal integrates seamlessly with:

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