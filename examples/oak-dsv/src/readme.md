# 🛠️ DSV Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-dsv`.

## 🚦 Quick Start

### Basic Parsing Example

The following is a standard workflow for parsing a simple DSV string (like CSV):

```rust
use oak_dsv::{DsvParser, DsvLanguage, Dsv};
use oak_core::{Parser, source::SourceText, parser::session::ParseSession};

const MY_CSV: DsvLanguage = DsvLanguage { field_separator: ',', quote_char: '"' };
type MyCsv = Dsv<MY_CSV>;

fn main() {
    // 1. Prepare source code
    let code = "id,name,email\n1,John Doe,john@example.com\n2,Jane Smith,jane@example.com";
    let source = SourceText::new(code.to_string());

    // 2. Initialize parser
    let parser = DsvParser::<MY_CSV>::new();
    let mut cache = ParseSession::<MyCsv>::default();

    // 3. Execute parsing
    let result = parser.parse(&source, &[], &mut cache);

    // 4. Handle results
    if result.result.is_ok() {
        println!("Parsing successful!");
    } else {
        eprintln!("Errors found during parsing.");
    }
}
```

## 🔍 Core API Usage

### 1. Syntax Tree Traversal
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract DSV constructs like headers, rows, and individual fields.

### 2. Incremental Parsing
No need to re-parse massive DSV files when small changes occur:
```rust,ignore
// Assuming you have an old parse result 'old_result' and new source text 'new_source'
let new_result = parser.parse(&new_source, &edits, &mut cache);
```

### 3. Diagnostics
`oak-dsv` provides rich error contexts specifically tailored for DSV data:
```rust,ignore
for diag in result.diagnostics {
    println!("{:?}", diag);
}
```

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes DSV source text into a stream of tokens, handling field delimiters, row separators, and complex quoting logic.
- **Parser**: Syntax analyzer to handle the structural layout of records and fields.
- **AST**: A strongly-typed syntax abstraction layer designed for building high-performance data processing tools and editors.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/](tests/) for handling of various DSV dialects and edge cases.
