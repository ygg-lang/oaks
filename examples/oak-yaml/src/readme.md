# 🛠️ YAML Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-yaml`.

## 🚦 Quick Start

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
oak-yaml = { path = "..." }
```

### Basic Parsing Example

The following is a standard workflow for parsing a YAML document:

```rust
use oak_yaml::{YamlParser, SourceText, YamlLanguage};

fn main() {
    // 1. Prepare source code
    let code = r#"
        version: "3.8"
        services:
          web:
            image: "nginx:latest"
            ports:
              - "80:80"
          db:
            image: "postgres:13"
            environment:
              POSTGRES_PASSWORD: example
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser
    let config = YamlLanguage::new();
    let parser = YamlParser::new(&config);

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
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract YAML mappings, sequences, scalars, and handle anchors/aliases.

### 2. Incremental Parsing
No need to re-parse massive YAML configuration files when small changes occur:
```rust
// Assuming you have an old parse result 'old_result' and new source text 'new_source'
let new_result = parser.reparse(&new_source, &old_result);
```

### 3. Diagnostics
`oak-yaml` provides precise error feedback for malformed YAML, such as indentation errors, unmatched brackets, or invalid tag usage:
```rust
for diag in result.diagnostics() {
    println!("[{}:{}] {}", diag.line, diag.column, diag.message);
}
```

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes YAML source text into a stream of tokens, handling indentation, block vs. flow styles, and various scalar formats.
- **Parser**: Syntax analyzer based on the Pratt parsing algorithm to handle YAML's hierarchical structure and complex value types.
- **AST**: A strongly-typed syntax abstraction layer designed for high-performance YAML analysis, formatting, and validation tools.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/](tests/) for handling of various YAML features and edge cases.
