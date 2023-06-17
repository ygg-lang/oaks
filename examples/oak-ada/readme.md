# Oak Ada Parser

[![Crates.io](https://img.shields.io/crates/v/oak-ada.svg)](https://crates.io/crates/oak-ada)
[![Documentation](https://docs.rs/oak-ada/badge.svg)](https://docs.rs/oak-ada)

High-performance incremental Ada parser for the oak ecosystem with flexible configuration, optimized for embedded systems and safety-critical applications.

## 🎯 Overview

Oak Ada is a robust parser for Ada 2012, designed to handle complete Ada syntax including SPARK annotations and modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for Ada analysis and tooling.

## ✨ Features

- **Complete Ada 2012 Syntax**: Supports all Ada features including modern specifications
- **SPARK Support**: Handles SPARK annotations and contracts
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_ada::{Parser, AdaLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
procedure Hello is
begin
    Put_Line("Hello, Ada!");
end Hello;
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed Ada procedure successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Package Parsing
```rust
use oak_ada::{Parser, AdaLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
package My_Package is
    procedure Say_Hello;
    function Get_Value return Integer;
end My_Package;

package body My_Package is
    procedure Say_Hello is
    begin
        Put_Line("Hello from Ada!");
    end Say_Hello;
    
    function Get_Value return Integer is
    begin
        return 42;
    end Get_Value;
end My_Package;
"#);

let result = parser.parse(&source);
println!("Parsed Ada package successfully.");
```

### Procedure Parsing
```rust
use oak_ada::{Parser, AdaLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
procedure Calculate_Area(Width, Height : in Float; Area : out Float) is
begin
    Area := Width * Height;
    Put_Line("Area calculated successfully.");
end Calculate_Area;
"#);

let result = parser.parse(&source);
println!("Parsed Ada procedure successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_ada::{Parser, AdaLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("procedure Hello is begin null; end Hello;");
let result = parser.parse(&source);
// Token information is available in the parse result
```

### Error Handling
```rust
use oak_ada::{Parser, AdaLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
procedure Broken is
    X : Integer := "not a number"; -- Type mismatch
    Y : Integer  -- Missing assignment or semicolon
begin
    Put_Line("Hello" -- Missing closing parenthesis
end Broken; -- Missing semicolon before end
"#);

let result = parser.parse(&source);
if let Err(e) = result.result {
    println!("Parse error: {:?}", e);
}
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **Package**: Root container for Ada packages
- **Procedure**: Ada procedures
- **Declarations**: Variable, constant, type declarations
- **Statements**: Assignment, if, loop, case statements

## 📊 Performance

- **Streaming**: Parse large Ada files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak-ada integrates seamlessly with:

- **Static Analysis**: Code quality and security analysis
- **Code Generation**: Generating code from Ada AST
- **IDE Support**: Language server protocol compatibility
- **Refactoring**: Automated code refactoring
- **Documentation**: Generating documentation from Ada code

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete Ada package parsing
- Procedure and function analysis
- Code transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-ada) or open [issues](https://github.com/ygg-lang/oaks/issues).