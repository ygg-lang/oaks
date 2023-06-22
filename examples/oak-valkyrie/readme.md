# Oak Valkyrie Parser

[![Crates.io](https://img.shields.io/crates/v/oak-valkyrie.svg)](https://crates.io/crates/oak-valkyrie)
[![Documentation](https://docs.rs/oak-valkyrie/badge.svg)](https://docs.rs/oak-valkyrie)

High-performance incremental Valkyrie parser for the oak ecosystem with flexible configuration, optimized for modern systems programming with advanced type safety and concurrency features.

## 🎯 Overview

Oak Valkyrie is a robust parser for the Valkyrie programming language, designed to handle complete Valkyrie syntax including modern language features and advanced type system. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for Valkyrie analysis and tooling.

## ✨ Features

- **Complete Valkyrie Syntax**: Supports all Valkyrie features including modern specifications
- **Advanced Type System**: Handles generics, traits, and type inference
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_valkyrie::{ValkyrieParser, ValkyrieLanguage};
use oak_core::{Parser, source::SourceText, parser::ParseSession};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let language = ValkyrieLanguage::default();
    let parser = ValkyrieParser::new(&language);
    let source = SourceText::new(r#"
namespace main {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}
    "#);
    
    let mut cache = ParseSession::default();
    let result = parser.parse(&source, &[], &mut cache);
    println!("Parsed Valkyrie module successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Module Parsing
```rust
use oak_valkyrie::{ValkyrieParser, ValkyrieLanguage};
use oak_core::{Parser, source::SourceText, parser::ParseSession};

let language = ValkyrieLanguage::default();
let parser = ValkyrieParser::new(&language);
let source = SourceText::new(r#"
namespace math {
    pub struct Point {
        x: f64,
        y: f64,
    }
    
    impl Point {
        pub fn new(x: f64, y: f64) -> Self {
            Self { x, y }
        }
        
        pub fn distance(&self, other: &Point) -> f64 {
            ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
        }
    }
}
"#);

let mut cache = ParseSession::default();
let result = parser.parse(&source, &[], &mut cache);
println!("Parsed Valkyrie module successfully.");
```

### Trait Parsing
```rust
use oak_valkyrie::{ValkyrieParser, ValkyrieLanguage};
use oak_core::{Parser, source::SourceText, parser::ParseSession};

let language = ValkyrieLanguage::default();
let parser = ValkyrieParser::new(&language);
let source = SourceText::new(r#"
pub trait Drawable {
    fn draw(&self);
    fn area(&self) -> f64;
    
    fn describe(&self) -> String {
        format!("Shape with area: {}", self.area())
    }
}

pub struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing circle with radius: {}", self.radius);
    }
    
    fn area(&self) -> f64 {
        3.14159 * self.radius * self.radius
    }
}
"#);

let mut cache = ParseSession::default();
let result = parser.parse(&source, &[], &mut cache);
println!("Parsed Valkyrie module successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_valkyrie::{ValkyrieParser, ValkyrieLanguage};
use oak_core::{Parser, source::SourceText, parser::ParseSession};

let language = ValkyrieLanguage::default();
let parser = ValkyrieParser::new(&language);
let source = SourceText::new("fn main() { let x = 42; println!(\"{}\", x); }");
let mut cache = ParseSession::default();
let result = parser.parse(&source, &[], &mut cache);
// Token information is available in the parse result
```

### Error Handling
```rust
use oak_valkyrie::{ValkyrieParser, ValkyrieLanguage};
use oak_core::{Parser, source::SourceText, parser::ParseSession};

let language = ValkyrieLanguage::default();
let parser = ValkyrieParser::new(&language);
let source = SourceText::new(r#"
fn broken_function() -> i32 {
    let x: i32 = "not a number"; // Type mismatch
    return x; // Type mismatch in return
}

fn invalid_syntax() { // Missing return type
    let y = 1 // Missing semicolon
}
"#);

let mut cache = ParseSession::default();
let result = parser.parse(&source, &[], &mut cache);
if let Err(e) = result.result {
    println!("Parse error: {:?}", e);
}
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **Module**: Module definitions with visibility
- **Function**: Function definitions with parameters and return types
- **Struct**: Struct definitions with fields
- **Enum**: Enumeration definitions with variants
- **Trait**: Trait definitions for shared behavior
- **Impl**: Implementation blocks for types
- **Statement**: Assignment, if, match, loop statements
- **Expression**: Binary, unary, method call expressions
- **Pattern**: Pattern matching constructs

## 📊 Performance

- **Streaming**: Parse large Valkyrie files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak-valkyrie integrates seamlessly with:

- **Static Analysis**: Code quality and security analysis
- **Code Generation**: Generating executable code from Valkyrie AST
- **IDE Support**: Language server protocol compatibility
- **Refactoring**: Automated code refactoring
- **Documentation**: Generating documentation from Valkyrie code

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete Valkyrie module parsing
- Trait and implementation analysis
- Pattern matching processing
- Integration with build systems

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-valkyrie) or open [issues](https://github.com/ygg-lang/oaks/issues).