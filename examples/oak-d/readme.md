# Oak D Parser

[![Crates.io](https://img.shields.io/crates/v/oak-d.svg)](https://crates.io/crates/oak-d)
[![Documentation](https://docs.rs/oak-d/badge.svg)](https://docs.rs/oak-d)

High-performance incremental D parser for the oak ecosystem with flexible configuration, optimized for code analysis and compilation.

## 🎯 Overview

Oak D is a robust parser for D, designed to handle complete D syntax including modern features like templates, mixins, and functional programming constructs. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for code analysis and compilation.

## ✨ Features

- **Complete D Syntax**: Supports all D features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_d::{Parser, DLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
import std.stdio;

void main() {
    writeln("Hello, World!");
}
"#);
    
    let result = parser.parse(&source);
    println!("Parsed D program successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Basic Program Parsing
```rust
use oak_d::{Parser, DLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("import std.stdio;\n\nvoid main() {\n    writeln(\"Hello, World!\");\n}");

let result = parser.parse(&source);
println!("Parsed D program successfully.");
```

### Class and Template Parsing
```rust
use oak_d::{Parser, DLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
class MyClass(T) {
    private T value;
    
    this(T val) {
        this.value = val;
    }
    
    T getValue() {
        return value;
    }
}

auto obj = new MyClass!int(42);
"#);

let result = parser.parse(&source);
println!("Parsed D program with classes and templates successfully.");
```

### Function Declaration Parsing
```rust
use oak_d::{Parser, DLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
int calculate(int a, int b, string operation) {
    switch (operation) {
        case "add": return a + b;
        case "subtract": return a - b;
        case "multiply": return a * b;
        default: return 0;
    }
}

void printResult(int result) {
    writefln("Result: %d", result);
}
"#);

let result = parser.parse(&source);
println!("Parsed D functions successfully.");
```

## 🔧 Advanced Features

### Lexer Integration
```rust
use oak_d::{Parser, DLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("import std.stdio;\nvoid main() { writeln(\"Hello\"); }");
let result = parser.parse(&source);
// Token information is available in the parse result
```

### Template and Mixin Analysis
```rust
use oak_d::{Parser, DLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
template Vector3(T) {
    struct Vector3 {
        T x, y, z;
        
        T length() const {
            import std.math : sqrt;
            return sqrt(x*x + y*y + z*z);
        }
    }
}

mixin template Logger() {
    void log(string msg) {
        writefln("[%s] %s", __FUNCTION__, msg);
    }
}
"#);

let result = parser.parse(&source);
println!("Parsed D templates and mixins successfully.");
```

### Import and Module Analysis
```rust
use oak_d::{Parser, DLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
module myapp.main;

import std.stdio : writeln, writefln;
import std.algorithm : map, filter;
import std.array : array;
import core.thread : Thread;

void main() {
    writeln("Application started");
}
"#);

let result = parser.parse(&source);
println!("Parsed D imports and modules successfully.");
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **Program**: Root container for D programs
- **Module**: Module declarations
- **Import**: Import statements
- **Function**: Function definitions
- **Class**: Class definitions
- **Template**: Template declarations
- **Mixin**: Mixin declarations

## 📊 Performance

- **Streaming**: Parse large D files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak D integrates seamlessly with:

- **Compilers**: Front-end for D compilers
- **Static Analysis Tools**: Code quality and security analysis
- **IDE Support**: Language server protocol compatibility
- **Code Generation**: Generating code from AST

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete D program parsing
- Class and template analysis
- Code transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-d) or open [issues](https://github.com/ygg-lang/oaks/issues).