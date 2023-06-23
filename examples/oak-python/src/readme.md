# 🛠️ Python Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-python`.

## 🚦 Quick Start

### Basic Parsing Example

The following is a standard workflow for parsing a Python function with type hints and decorators:

```rust
use oak_python::{PythonParser, SourceText, PythonLanguage};

fn main() {
    // 1. Prepare source code
    let code = r#"
        @dataclass
        class User:
            name: str
            age: int
            
            async fn greet(self) -> str:
                return f"Hello, {self.name}!"
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser
    let config = PythonLanguage::new();
    let parser = PythonParser::new(&config);

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
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract Python-specific constructs like class definitions, indentation-based blocks, type annotations, or decorators.

### 2. Incremental Parsing
No need to re-parse the entire module when small changes occur:
```rust
// Assuming you have an old parse result 'old_result' and new source text 'new_source'
let new_result = parser.reparse(&new_source, &old_result);
```

### 3. Diagnostics
`oak-python` provides rich error contexts specifically tailored for Python developers, handling indentation errors and modern syntax requirements:
```rust
for diag in result.diagnostics() {
    println!("[{}:{}] {}", diag.line, diag.column, diag.message);
}
```

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes Python source text into a stream of tokens, including complex indentation/dedentation logic and support for f-strings.
- **Parser**: Syntax analyzer based on the Pratt parsing algorithm to handle Python's expression precedence, structural pattern matching, and block identification.
- **AST**: A strongly-typed syntax abstraction layer designed for high-performance Python analysis tools, linters, and IDEs.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/](tests/) for handling of various Python versions and edge cases.
