# 🛠️ CSV Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-csv`.

## 🚦 Quick Start

### Basic Parsing Example

The following is a standard workflow for parsing a simple CSV string:

```rust
use oak_csv::{CsvParser, CsvLanguage};
use oak_core::{Parser, source::SourceText, parser::session::ParseSession};

fn main() {
    // 1. Prepare source code
    let code = "id,name,email\n1,John Doe,john@example.com\n2,Jane Smith,jane@example.com";
    let source = SourceText::new(code.to_string());

    // 2. Initialize parser
    let parser = CsvParser::new();
    let mut session = ParseSession::<CsvLanguage>::default();

    // 3. Execute parsing
    let result = parser.parse(&source, &[], &mut session);

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
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract CSV constructs like headers, rows, and individual fields.

### 2. Incremental Parsing
No need to re-parse massive CSV files when small changes occur:
```rust
# use oak_csv::{CsvParser, CsvLanguage};
# use oak_core::{Parser, source::SourceText, parser::session::ParseSession};
# let parser = CsvParser::new();
# let mut session = ParseSession::<CsvLanguage>::default();
# let old_source = SourceText::new("id,name\n1,John".to_string());
# let old_result = parser.parse(&old_source, &[], &mut session);
# let new_source = SourceText::new("id,name\n1,John Doe".to_string());
// Assuming you have an old parse result 'old_result' and new source text 'new_source'
let new_result = parser.parse(&new_source, &[], &mut session);
```

### 3. Diagnostics
`oak-csv` provides rich error contexts specifically tailored for CSV data:
```rust
# use oak_csv::{CsvParser, CsvLanguage};
# use oak_core::{Parser, source::SourceText, parser::session::ParseSession};
# let parser = CsvParser::new();
# let mut session = ParseSession::<CsvLanguage>::default();
# let source = SourceText::new("id,name\n1".to_string());
# let result = parser.parse(&source, &[], &mut session);
for diag in result.diagnostics {
    println!("{:?}", diag);
}
```

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes CSV source text into a stream of tokens, handling field delimiters, row separators, and complex quoting logic.
- **Parser**: Syntax analyzer based on the structural layout of records and fields.
- **AST**: A strongly-typed syntax abstraction layer designed for building high-performance data processing tools and editors.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/](tests/) for handling of various CSV dialects and edge cases.
