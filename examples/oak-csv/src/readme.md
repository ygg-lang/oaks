# 🛠️ CSV Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-csv`.

## 🚦 Quick Start

### Basic Parsing Example

The following is a standard workflow for parsing a simple CSV string:

```rust
use oak_csv::{CsvParser, SourceText, CsvLanguage};

fn main() {
    // 1. Prepare source code
    let code = r#"
        id,name,email
        1,John Doe,john@example.com
        2,Jane Smith,jane@example.com
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser
    let config = CsvLanguage::new();
    let parser = CsvParser::new(&config);

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
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract CSV constructs like headers, rows, and individual fields.

### 2. Incremental Parsing
No need to re-parse massive CSV files when small changes occur:
```rust
// Assuming you have an old parse result 'old_result' and new source text 'new_source'
let new_result = parser.reparse(&new_source, &old_result);
```

### 3. Diagnostics
`oak-csv` provides rich error contexts specifically tailored for CSV data:
```rust
for diag in result.diagnostics() {
    println!("[{}:{}] {}", diag.line, diag.column, diag.message);
}
```

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes CSV source text into a stream of tokens, handling field delimiters, row separators, and complex quoting logic.
- **Parser**: Syntax analyzer based on the Pratt parsing algorithm to handle the structural layout of records and fields.
- **AST**: A strongly-typed syntax abstraction layer designed for building high-performance data processing tools and editors.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/](tests/) for handling of various CSV dialects and edge cases.
