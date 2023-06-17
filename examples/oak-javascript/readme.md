# Oak JavaScript Parser

[![Crates.io](https://img.shields.io/crates/v/oak-javascript.svg)](https://crates.io/crates/oak-javascript)
[![Documentation](https://docs.rs/oak-javascript/badge.svg)](https://docs.rs/oak-javascript)

High-performance incremental JavaScript parser for the oak ecosystem with flexible configuration, optimized for static analysis and code generation.

## 🎯 Overview

Oak JavaScript is a robust parser for JavaScript, designed to handle complete JavaScript syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for static analysis and code generation.

## ✨ Features

- **Complete JavaScript Syntax**: Supports all JavaScript features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_javascript::{Parser, JavaScriptLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
        function greet(name) {
            console.log('Hello, ' + name + '!');
        }
        
        greet('World');
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed JavaScript successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Function Parsing
```rust
use oak_javascript::{Parser, JavaScriptLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
    function add(a, b) {
        return a + b;
    }
"#);

let result = parser.parse(&source);
println!("Parsed JavaScript function successfully.");
```

### Object Parsing
```rust
use oak_javascript::{Parser, JavaScriptLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"{name: "Alice", age: 30}"#);

let result = parser.parse(&source);
println!("Parsed JavaScript object successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_javascript::{Parser, JavaScriptLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("const x = 42;");
let result = parser.parse(&source);
// Token information is available in the parse result
```

### Error Handling
```rust
use oak_javascript::{Parser, JavaScriptLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
    function greet(name) {
        console.log('Hello, ' + name + '!')
    // Missing closing brace
"#);

let result = parser.parse(&source);
if let Err(e) = result.result {
    println!("Parse error: {:?}", e);
}
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **Script**: Root container for JavaScript programs
- **Function**: JavaScript functions and methods
- **VariableDeclaration**: Variable and constant declarations
- **Expression**: Various expression types (binary, unary, call, etc.)
- **Statement**: Control flow, loops, conditionals

## 📊 Performance

- **Streaming**: Parse large JavaScript files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak JavaScript integrates seamlessly with:

- **Static Analysis**: Code quality and security analysis
- **Code Generation**: Generating code from JavaScript AST
- **IDE Support**: Language server protocol compatibility
- **Refactoring**: Automated code refactoring
- **Documentation**: Generating documentation from JavaScript code

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete JavaScript program parsing
- Function and object analysis
- Code transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-javascript) or open [issues](https://github.com/ygg-lang/oaks/issues).