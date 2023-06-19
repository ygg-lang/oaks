# Oak C++ Parser

[![Crates.io](https://img.shields.io/crates/v/oak-cpp.svg)](https://crates.io/crates/oak-cpp)
[![Documentation](https://docs.rs/oak-cpp/badge.svg)](https://docs.rs/oak-cpp)

High-performance incremental C++ parser for the oak ecosystem with flexible configuration, optimized for code analysis and compilation.

## 🎯 Overview

Oak C++ is a robust parser for C++, designed to handle complete C++ syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for static analysis and code generation.

## ✨ Features

- **Complete C++ Syntax**: Supports all C++ features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_cpp::{Parser, CppLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
        #include <iostream>

        class Greeter {
        public:
            void greet() {
                std::cout << "Hello, C++!" << std::endl;
            }
        };

        int main() {
            Greeter greeter;
            greeter.greet();
            return 0;
        }
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed C++ program successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Class Parsing
```rust
use oak_cpp::{Parser, CppLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
    class MyClass {
    public:
        int myMethod(int x) { return x * 2; }
    };
    "#);

let result = parser.parse(&source);
println!("Parsed C++ class successfully.");
```

### Template Parsing
```rust
use oak_cpp::{Parser, CppLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
    template <typename T>
    T max(T a, T b) {
        return (a > b) ? a : b;
    }
    "#);

let result = parser.parse(&source);
println!("Parsed C++ template successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_cpp::{Parser, CppLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("int main() { return 0; }");
let result = parser.parse(&source);
// Token information is available in the parse result
```

### Error Handling
```rust
use oak_cpp::{Parser, CppLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
    int main() {
        std::cout << "Hello, World!" << std::endl
        return 0;
    }
    "#);

let result = parser.parse(&source);
if let Err(e) = result.result {
    println!("Parse error: {:?}", e);
}
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **Program**: Root container for C++ programs
- **ClassDefinition**: Class and struct definitions
- **FunctionDefinition**: Function declarations and definitions
- **TemplateDeclaration**: Template definitions
- **Statement**: Control flow, expressions, blocks

## 📊 Performance

- **Streaming**: Parse large C++ files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak C++ integrates seamlessly with:

- **Compilers**: Front-end for C++ compilers
- **Static Analysis Tools**: Code quality and security analysis
- **IDE Support**: Language server protocol compatibility
- **Code Generation**: Generating code from AST

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete C++ program parsing
- Class and template analysis
- Code transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-cpp) or open [issues](https://github.com/ygg-lang/oaks/issues).