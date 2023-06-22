# Oak Vala Parser

[![Crates.io](https://img.shields.io/crates/v/oak-vala.svg)](https://crates.io/crates/oak-vala)
[![Documentation](https://docs.rs/oak-vala/badge.svg)](https://docs.rs/oak-vala)

High-performance incremental Vala parser for the oak ecosystem with flexible configuration, optimized for GObject-based application development and GNOME ecosystem integration.

## 🎯 Overview

Oak Vala is a robust parser for the Vala programming language, designed to handle complete Vala syntax including modern object-oriented features and GTK bindings. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for Vala analysis and tooling.

## ✨ Features

- **Complete Vala Syntax**: Supports all Vala features including modern specifications
- **GObject Integration**: Handles GObject type system and signals
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_vala::{ValaLanguage, ValaParser};
use oak_core::{Parser, source::SourceText, parser::session::ParseSession};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let language = ValaLanguage::default();
    let parser = ValaParser::new(&language);
    let mut session = ParseSession::<ValaLanguage>::default();
    let source = SourceText::new(r#"
public class HelloWorld : Object {
    public static int main(string[] args) {
        stdout.printf("Hello, World!\n");
        return 0;
    }
}
    "#);
    
    let result = parser.parse(&source, &[], &mut session);
    println!("Parsed Vala class successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Class Parsing
```rust
use oak_vala::{ValaLanguage, ValaParser};
use oak_core::{Parser, source::SourceText, parser::session::ParseSession};

let language = ValaLanguage::default();
let parser = ValaParser::new(&language);
let mut session = ParseSession::<ValaLanguage>::default();
let source = SourceText::new(r#"
public class Calculator : Object {
    private double _result;
    
    public double result {
        get { return _result; }
        set { _result = value; }
    }
    
    public Calculator() {
        _result = 0.0;
    }
    
    public double add(double value) {
        _result += value;
        return _result;
    }
    
    public signal void changed(double new_value);
}
"#);

let result = parser.parse(&source, &[], &mut session);
println!("Parsed Vala class successfully.");
```

### Interface Parsing
```rust
use oak_vala::{ValaLanguage, ValaParser};
use oak_core::{Parser, source::SourceText, parser::session::ParseSession};

let language = ValaLanguage::default();
let parser = ValaParser::new(&language);
let mut session = ParseSession::<ValaLanguage>::default();
let source = SourceText::new(r#"
public interface Drawable {
    public abstract void draw(Context ctx);
    
    public virtual void resize(int width, int height) {
        // Default implementation
    }
}
"#);

let result = parser.parse(&source, &[], &mut session);
println!("Parsed Vala interface successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_vala::{ValaLanguage, ValaParser};
use oak_core::{Parser, source::SourceText, parser::session::ParseSession};

let language = ValaLanguage::default();
let parser = ValaParser::new(&language);
let mut session = ParseSession::<ValaLanguage>::default();
let source = SourceText::new("public class Test { public int value; }");
let result = parser.parse(&source, &[], &mut session);
// Token information is available in the parse result
```

### Error Handling
```rust
use oak_vala::{ValaLanguage, ValaParser};
use oak_core::{Parser, source::SourceText, parser::session::ParseSession};

let language = ValaLanguage::default();
let parser = ValaParser::new(&language);
let mut session = ParseSession::<ValaLanguage>::default();
let source = SourceText::new(r#"
public class Broken {
    public int invalid_method() {
        return "not a number"; // Type mismatch
    }
    
    public int missing_semicolon // Missing semicolon
    public property invalid_prop // Missing type
}
"#);

let result = parser.parse(&source, &[], &mut session);
if let Err(e) = result.result {
    println!("Parse error: {:?}", e);
}
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **Class**: Class definitions with inheritance
- **Interface**: Interface definitions
- **Method**: Method and function definitions
- **Property**: Property definitions with getters/setters
- **Signal**: Signal definitions for event handling
- **Namespace**: Namespace declarations
- **Statements**: Assignment, if, loop, try statements
- **Expressions**: Binary, unary, method call expressions

## 📊 Performance

- **Streaming**: Parse large Vala files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak-vala integrates seamlessly with:

- **Static Analysis**: Code quality and security analysis
- **Code Generation**: Generating C code from Vala AST
- **IDE Support**: Language server protocol compatibility
- **Refactoring**: Automated code refactoring
- **Documentation**: Generating documentation from Vala code

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete Vala class parsing
- Interface and implementation analysis
- Property and signal handling
- Integration with GTK applications

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-vala) or open [issues](https://github.com/ygg-lang/oaks/issues).