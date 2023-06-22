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
use oak_d::{DLanguage, DParser};
use oak_core::{Parser, source::SourceText, parser::session::ParseSession};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let language = DLanguage::default();
    let parser = DParser::new(&language);
    let mut session = ParseSession::<DLanguage>::default();
    let source = SourceText::new(r#"
import std.stdio;

void main() {
    writeln("Hello, World!");
}
"#);
    
    let result = parser.parse(&source, &[], &mut session);
    println!("Parsed D program successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Basic Program Parsing
```rust
use oak_d::{DLanguage, DParser};
use oak_core::{Parser, source::SourceText, parser::session::ParseSession};

let language = DLanguage::default();
let parser = DParser::new(&language);
let mut session = ParseSession::<DLanguage>::default();
let source = SourceText::new("import std.stdio;\n\nvoid main() {\n    writeln(\"Hello, World!\");\n}");

let result = parser.parse(&source, &[], &mut session);
println!("Parsed D program successfully.");
```

### Class and Template Parsing
```rust
use oak_d::{DLanguage, DParser};
use oak_core::{Parser, source::SourceText, parser::session::ParseSession};

let language = DLanguage::default();
let parser = DParser::new(&language);
let mut session = ParseSession::<DLanguage>::default();
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

let result = parser.parse(&source, &[], &mut session);
println!("Parsed D program with classes and templates successfully.");
```

### Function Declaration Parsing
```rust
use oak_d::{DLanguage, DParser};
use oak_core::{Parser, source::SourceText, parser::session::ParseSession};

let language = DLanguage::default();
let parser = DParser::new(&language);
let mut session = ParseSession::<DLanguage>::default();
let source = SourceText::new(r#"
int calculate(int a, int b, string operation) {
    switch (operation) {
        case "add": return a + b;
        case "subtract": return a - b;
        case "multiply": return a * b;
        default: return 0;
    }
}
"#);

let result = parser.parse(&source, &[], &mut session);
println!("Parsed D program with function declarations successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_d::{DLanguage, DParser};
use oak_core::{Parser, source::SourceText, parser::session::ParseSession};

let language = DLanguage::default();
let parser = DParser::new(&language);
let mut session = ParseSession::<DLanguage>::default();
let source = SourceText::new("void main() {}");
let result = parser.parse(&source, &[], &mut session);
// Token information is available in the parse result
```

### Error Handling
```rust
use oak_d::{DLanguage, DParser};
use oak_core::{Parser, source::SourceText, parser::session::ParseSession};

let language = DLanguage::default();
let parser = DParser::new(&language);
let mut session = ParseSession::<DLanguage>::default();
let source = SourceText::new(r#"
void main() {
    writeln("Hello, World!") // Missing semicolon
}
"#);

let result = parser.parse(&source, &[], &mut session);
if let Err(e) = result.result {
    println!("Parse error: {:?}", e);
}
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