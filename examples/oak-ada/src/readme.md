# 🛠️ Ada Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-ada`.

## 🚦 Quick Start

### Basic Parsing Example

The following is a standard workflow for parsing an Ada package specification:

```rust
use oak_ada::{AdaParser, SourceText, AdaLanguage};

fn main() {
    // 1. Prepare source code
    let code = r#"
        package Hello_World is
           procedure Say_Hello;
        end Hello_World;
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser
    let config = AdaLanguage::new();
    let parser = AdaParser::new(&config);

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
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract Ada constructs like package declarations, procedure signatures, or tasking statements.

### 2. Incremental Parsing
No need to re-parse the entire file when small changes occur:
```rust
// Assuming you have an old parse result 'old_result' and new source text 'new_source'
let new_result = parser.reparse(&new_source, &old_result);
```

### 3. Diagnostics
`oak-ada` provides rich error contexts specifically tailored for Ada developers:
```rust
for diag in result.diagnostics() {
    println!("[{}:{}] {}", diag.line, diag.column, diag.message);
}
```

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes Ada source text into a stream of tokens, handling keywords (case-insensitive), operators, and numeric literals.
- **Parser**: Syntax analyzer based on the Pratt parsing algorithm to handle Ada's structural declarations and expression precedence.
- **AST**: A strongly-typed syntax abstraction layer designed for building high-performance Ada analysis tools and IDEs.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/](tests/) for handling of various Ada edge cases and language versions.
