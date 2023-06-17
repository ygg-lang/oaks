# Oak D Parser

[![Crates.io](https://img.shields.io/crates/v/oak-d.svg)](https://crates.io/crates/oak-d)
[![Documentation](https://docs.rs/oak-d/badge.svg)](https://docs.rs/oak-d)

High-performance incremental D parser for the oak ecosystem with flexible configuration, optimized for code analysis and compilation.

## 🎯 Overview

Oak-d is a robust parser for D, designed to handle complete D syntax including modern features like templates, mixins, and functional programming constructs. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for code analysis and compilation.

## ✨ Features

- **Complete D Syntax**: Supports all D features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_d::DParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = DParser::new();
    let d_code = r#"
import std.stdio;

void main() {
    writeln("Hello, World!");
}
"#;
    
    let program = parser.parse(d_code)?;
    println!("Parsed D program with {} declarations", program.declarations.len());
    Ok(())
}
```

## 📋 Parsing Examples

### Basic Program Parsing
```rust
use oak_d::{DParser, ParseOptions};

let parser = DParser::new(ParseOptions::default());
let result = parser.parse_program("import std.stdio;\n\nvoid main() {\n    writeln(\"Hello, World!\");\n}");

match result {
    Ok(program) => println!("Parsed D program: {:?}", program),
    Err(error) => eprintln!("Parse error: {}", error),
}
```

### Class and Template Parsing
```rust
use oak_d::{DParser, ParseOptions};

let parser = DParser::new(ParseOptions::default());
let d_code = r#"
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
"#;

let result = parser.parse_program(d_code);
match result {
    Ok(program) => {
        println!("Parsed D program with classes and templates");
        // Process the AST for classes and templates
    }
    Err(error) => eprintln!("Parse error: {}", error),
}
```

### Function Declaration Parsing
```rust
use oak_d::{DParser, ast::{Declaration, Function}};

let parser = DParser::new();
let d_code = r#"
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
"#;

let program = parser.parse(d_code)?;
for decl in &program.declarations {
    if let Declaration::Function(func) = decl {
        println!("Function: {} with {} parameters", func.name, func.parameters.len());
    }
}
```

## 🔧 Advanced Features

### Lexer Integration
```rust
use oak_d::{DLexer, DSyntaxKind, DLanguage};
use oak_core::lexer::Lexer;

let lexer = DLexer::new("import std.stdio;\nvoid main() { writeln(\"Hello\"); }");
let tokens: Vec<_> = lexer.tokenize().collect();

for token in tokens {
    println!("Token: {:?} at {:?}", token.kind, token.span);
}
```

### Template and Mixin Analysis
```rust
use oak_d::{DParser, ast::{Declaration, Template, Mixin}};

let parser = DParser::new();
let d_code = r#"
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
"#;

let program = parser.parse(d_code)?;
for decl in &program.declarations {
    match decl {
        Declaration::Template(temp) => {
            println!("Template: {} with {} parameters", temp.name, temp.params.len());
        }
        Declaration::Mixin(mixin) => {
            println!("Mixin: {} with {} members", mixin.name, mixin.members.len());
        }
        _ => {}
    }
}
```

### Import and Module Analysis
```rust
use oak_d::{DParser, ast::{Declaration, Import}};

let parser = DParser::new();
let d_code = r#"
module myapp.main;

import std.stdio : writeln, writefln;
import std.algorithm : map, filter;
import std.array : array;
import core.thread : Thread;

void main() {
    writeln("Application started");
}
"#;

let program = parser.parse(d_code)?;
for decl in &program.declarations {
    if let Declaration::Import(import) = decl {
        println!("Import: {} (selective: {})", import.module_path, import.symbols.is_some());
    }
}
```

## 🏗️ AST Structure

### Program Structure

```rust
pub struct Program {
    pub declarations: Vec<Declaration>,
    pub module_info: ModuleInfo,
}

pub enum Declaration {
    Function(Function),
    Class(Class),
    Struct(Struct),
    Import(Import),
    Template(Template),
}

pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Type,
    pub body: Block,
}
```

## 📊 Performance

- **Parse large D source files without loading entirely into memory**
- **Incremental parsing support for real-time code analysis**
- **Memory Efficient: Smart AST node allocation**
- **Optimized for compilation and static analysis workflows**

## 🔗 Integration

Oak-d integrates seamlessly with:

- **Static Analysis**: Code quality and security analysis
- **Code Generation**: Generating code from D AST
- **IDE Support**: Language server protocol compatibility
- **Refactoring**: Automated code refactoring
- **Documentation**: Generating documentation from D code

## 📚 Examples

### Complete D Program Parsing

```rust
use oak_d::DParser;

let parser = DParser::new();
let d_code = r#"
module hello;

import std.stdio;

void main()
{
    writeln("Hello, World!");
}
"#;

let program = parser.parse(d_code)?;
println!("Parsed D module with {} declarations", program.declarations.len());
```

### Function and Template Analysis

```rust
use oak_d::{DParser, ast::{Declaration, Function}};

let parser = DParser::new();
let d_code = r#"
template MyTemplate(T) {
    T process(T value) {
        return value * 2;
    }
}

int add(int a, int b) {
    return a + b;
}
"#;

let program = parser.parse(d_code)?;
for decl in &program.declarations {
    match decl {
        Declaration::Function(func) => {
            println!("Function: {} returns {}", func.name, func.return_type);
        }
        Declaration::Template(temp) => {
            println!("Template: {}", temp.name);
        }
        _ => {}
    }
}
```

Check out the [examples](examples/) directory for more comprehensive examples:

- Basic D parsing and AST generation
- Custom analysis and transformation
- Performance benchmarks
- Integration with build tools
- Language server implementation

## 🤝 Contributing

Contributions are welcome! Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-d) or open [issues](https://github.com/ygg-lang/oaks/issues).

---

**Oak-d** - High-performance D parser for the oak ecosystem 🚀