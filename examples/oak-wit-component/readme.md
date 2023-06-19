# Oak WIT Component Parser

[![Crates.io](https://img.shields.io/crates/v/oak-wit-component.svg)](https://crates.io/crates/oak-wit-component)
[![Documentation](https://docs.rs/oak-wit-component/badge.svg)](https://docs.rs/oak-wit-component)

High-performance incremental WIT Component parser for the oak ecosystem with flexible configuration, optimized for WebAssembly Interface Types processing.

## 🎯 Overview

Oak WIT Component is a robust parser for WIT (WebAssembly Interface Types) components, designed to handle complete WIT syntax including modern specifications. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for WebAssembly component analysis and code generation.

## ✨ Features

- **Complete WIT Syntax**: Supports all WIT features including interface definitions
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_wit_component::{Parser, WitComponentLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
        package example:calculator;

        interface calculator {
            add: func(a: f32, b: f32) -> f32;
            subtract: func(a: f32, b: f32) -> f32;
        }

        world calculator-world {
            import calculator;
        }
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed WIT Component successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Interface Parsing
```rust
use oak_wit_component::{Parser, WitComponentLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
    interface math {
        add: func(a: f64, b: f64) -> f64;
        multiply: func(a: f64, b: f64) -> f64;
    }
"#);

let result = parser.parse(&source);
println!("Interface parsed successfully.");
```

### World Parsing
```rust
use oak_wit_component::{Parser, WitComponentLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
    world example {
        import wasi:clocks/monotonic-clock@0.2.0;
        export example:interface;
    }
"#);

let result = parser.parse(&source);
println!("World parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_wit_component::{Parser, WitComponentLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("interface test { func: func(); }");
let result = parser.parse(&source);
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_wit_component::{Parser, WitComponentLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
    interface invalid {
        func: func(a: f32
    // Missing closing parenthesis
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

- **Package**: WIT package definitions
- **Interface**: Interface definitions with functions and types
- **World**: World definitions with imports and exports
- **Function**: Function definitions with parameters and results
- **Type**: Type definitions including records, variants, and primitives

## 📊 Performance

- **Streaming**: Parse large WIT files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak WIT Component integrates seamlessly with:

- **WebAssembly Tools**: Component analysis and generation
- **IDE Support**: Language server protocol compatibility for WIT
- **Code Generation**: Generating bindings from WIT interfaces
- **Component Validation**: Validating WIT component specifications
- **Documentation**: Generating documentation from WIT interfaces

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete WIT component parsing
- Interface and world analysis
- Component validation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-wit-component) or open [issues](https://github.com/ygg-lang/oaks/issues).