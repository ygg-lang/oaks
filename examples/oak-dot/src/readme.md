# 🛠️ DOT Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-dot`.

## 🚦 Quick Start

### Basic Parsing Example

The following is a standard workflow for parsing a simple DOT graph:

```rust
use oak_dot::{DotParser, SourceText, DotLanguage};

fn main() {
    // 1. Prepare source code
    let code = r#"
        digraph G {
            main -> parse -> execute;
            main -> init;
            main -> cleanup;
            execute -> make_string;
            execute -> printf;
            init -> make_string;
            main -> printf;
            execute -> compare;
        }
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser
    let config = DotLanguage::new();
    let parser = DotParser::new(&config);

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
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract DOT constructs like graph definitions, node statements, edge connections, or attribute lists.

### 2. Incremental Parsing
No need to re-parse the entire graph description when small changes occur:
```rust
// Assuming you have an old parse result 'old_result' and new source text 'new_source'
let new_result = parser.reparse(&new_source, &old_result);
```

### 3. Diagnostics
`oak-dot` provides rich error contexts specifically tailored for DOT developers:
```rust
for diag in result.diagnostics() {
    println!("[{}:{}] {}", diag.line, diag.column, diag.message);
}
```

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes DOT source text into a stream of tokens, handling keywords, operators (like `->` or `--`), and complex identifiers.
- **Parser**: Syntax analyzer based on the Pratt parsing algorithm to handle DOT's graph-based structure and attribute nesting.
- **AST**: A strongly-typed syntax abstraction layer designed for building high-performance graph analysis tools and visualizers.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/](tests/) for handling of various DOT edge cases and styling attributes.
