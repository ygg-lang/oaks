# 🛠️ C++ Parser Developer Guide

Cpp support for the Oak language framework.

This guide is designed to help you quickly get started with developing and integrating `oak-cpp`.

## 🚦 Quick Start

### Basic Parsing Example

The following is a standard workflow for parsing a C++ class:

```rust
use oak_cpp::{CppParser, language::CppLanguage};
use oak_core::{SourceText, parser::Parser, source::TextEdit, parser::ParseSession};

fn main() {
    // 1. Prepare source code
    let code = r#"
        #include <vector>
        
        namespace core {
            class Vector {
            public:
                void push(int value) {
                    data.push_back(value);
                }
            private:
                std::vector<int> data;
            };
        }
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser
    let config = CppLanguage::default();
    let parser = CppParser::new(&config);
    let mut cache = ParseSession::default();

    // 3. Execute parsing
    let result = parser.parse(&source, &[], &mut cache);

    // 4. Handle results
    if result.diagnostics.is_empty() {
        println!("Parsing successful!");
    } else {
        eprintln!("Errors found during parsing.");
    }
}
```

## 🔍 Core API Usage

### 1. Syntax Tree Traversal
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract C++ constructs like class definitions, template parameters, or namespace hierarchies.

### 2. Incremental Parsing
No need to re-parse the entire translation unit when small changes occur:
```rust
use oak_cpp::{CppParser, language::CppLanguage};
use oak_core::{SourceText, parser::Parser, source::TextEdit, parser::ParseSession};

fn main() {
    // Initial parsing
    let code = r#"
        class Vector {
            void push(int value) {}
        };
    "#;
    let source = SourceText::new(code);
    let config = CppLanguage::default();
    let parser = CppParser::new(&config);
    let mut cache = ParseSession::default();
    let old_result = parser.parse(&source, &[], &mut cache);

    // Updated code
    let new_code = r#"
        class Vector {
            void push(int value) {
                data.push_back(value);
            }
        };
    "#;
    let new_source = SourceText::new(new_code);
    
    // Re-parsing (simplified example)
    let new_result = parser.parse(&new_source, &[], &mut cache);
}
```

### 3. Diagnostics
`oak-cpp` provides rich error contexts specifically tailored for C++ developers, handling complex error scenarios like template instantiation failures:
```rust
use oak_cpp::{CppParser, language::CppLanguage};
use oak_core::{SourceText, parser::Parser, source::TextEdit, parser::ParseSession};

fn main() {
    let code = r#"
        class Vector {
            void push(int value) {
                return;
            }
        };
    "#;
    let source = SourceText::new(code);
    let config = CppLanguage::default();
    let parser = CppParser::new(&config);
    let mut cache = ParseSession::default();
    let result = parser.parse(&source, &[], &mut cache);

    for error in &result.diagnostics {
        println!("Error: {}", error);
    }
}
```

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes C++ source text into a stream of tokens, handling keywords, operators, and literals, including support for modern C++ features.
- **Parser**: Syntax analyzer based on the Pratt parsing algorithm to handle complex C++ expression precedence, operator overloading, and template syntax.
- **AST**: A strongly-typed syntax abstraction layer designed for high-performance C++ analysis tools.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/](tests/) for handling of various C++ standards and edge cases.
