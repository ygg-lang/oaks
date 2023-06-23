# 🛠️ C++ Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-cpp`.

## 🚦 Quick Start

### Basic Parsing Example

The following is a standard workflow for parsing a C++ class:

```rust
use oak_cpp::{CppParser, SourceText, CppLanguage};

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
    let config = CppLanguage::new();
    let parser = CppParser::new(&config);

    // 3. Execute parsing
    let result = parser.parse(&source);

    // 4. Handle results
    if result.is_success() {
        println!("Parsing successful! AST node count: {}", result.node_count());
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
// Assuming you have an old parse result 'old_result' and new source text 'new_source'
let new_result = parser.reparse(&new_source, &old_result);
```

### 3. Diagnostics
`oak-cpp` provides rich error contexts specifically tailored for C++ developers, handling complex error scenarios like template instantiation failures:
```rust
for diag in result.diagnostics() {
    println!("[{}:{}] {}", diag.line, diag.column, diag.message);
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
