# Oak C++ Parser

[![Crates.io](https://img.shields.io/crates/v/oak-cpp.svg)](https://crates.io/crates/oak-cpp)
[![Documentation](https://docs.rs/oak-cpp/badge.svg)](https://docs.rs/oak-cpp)

High-performance incremental C++ parser for the oak ecosystem with flexible configuration, optimized for code analysis and compilation.

## 🎯 Overview

Oak-cpp is a robust parser for C++, designed to handle complete C++ syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for static analysis and code generation.

## ✨ Features

- **Complete C++ Syntax**: Supports all C++ features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_cpp::CppParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = CppParser::new();
    let cpp_code = r#"
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
    "#;
    
    let program = parser.parse_program(cpp_code)?;
    println!("Parsed C++ program successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Class Parsing
```rust
use oak_cpp::{CppParser, ast::ClassDefinition};

let parser = CppParser::new();
let cpp_code = r#"
    class MyClass {
    public:
        int myMethod(int x) { return x * 2; }
    };
"#;

let program = parser.parse_program(cpp_code)?;
if let Some(ClassDefinition { name, .. }) = program.classes.get(0) {
    println!("Parsed class: {}", name);
}
```

### Template Parsing
```rust
use oak_cpp::{CppParser, ast::TemplateDeclaration};

let parser = CppParser::new();
let cpp_code = r#"
    template <typename T>
    T max(T a, T b) {
        return (a > b) ? a : b;
    }
"#;

let program = parser.parse_program(cpp_code)?;
if let Some(TemplateDeclaration { name, .. }) = program.templates.get(0) {
    println!("Parsed template: {}", name);
}
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_cpp::{CppParser, lexer::Token};

let parser = CppParser::new();
let tokens = parser.tokenize("int main() { return 0; }")?;
for token in tokens {
    println!("{:?}", token.kind);
}
```

### Error Handling
```rust
use oak_cpp::CppParser;

let parser = CppParser::new();
let invalid_cpp = r#"
    int main() {
        std::cout << "Hello, World!" << std::endl
        return 0;
    }
"#;

match parser.parse_program(invalid_cpp) {
    Ok(program) => println!("Parsed C++ program successfully."),
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

Oak-cpp integrates seamlessly with:

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