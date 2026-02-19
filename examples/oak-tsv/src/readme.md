# 🛠️ TSV Parser Developer Guide

Tsv support for the Oak language framework.

This guide is designed to help you quickly get started with developing and integrating `oak-tsv`.

## 🚦 Quick Start

### Basic Parsing Example

The following is a standard workflow for parsing a simple TSV string:

```rust
use oak_tsv::{TsvParser, TsvLanguage};
use oak_core::{Parser, source::SourceText, parser::session::ParseSession};

fn main() {
    let source = SourceText::new("header1\theader2\nvalue1\tvalue2");
    let mut session = ParseSession::<TsvLanguage>::default();
    let parser = TsvParser::new();
    let result = parser.parse(&source, &[], &mut session);
    let ast = result.result.unwrap();
    println!("{:#?}", ast);
}
```

## Advanced Usage

For more advanced scenarios, such as customized parsing or handling larger files, you can use the `TsvBuilder` to construct TSV AST manually.

```rust
use oak_tsv::TsvBuilder;
// your code here
```

## 🔍 Core API Usage

### 1. Syntax Tree Traversal
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract TSV constructs like headers, rows, and individual fields.

### 2. Incremental Parsing
No need to re-parse massive TSV files when small changes occur:
```rust
# use oak_tsv::{TsvParser, TsvLanguage};
# use oak_core::{Parser, source::SourceText, parser::session::ParseSession};
# let parser = TsvParser::new();
# let mut session = ParseSession::<TsvLanguage>::default();
# let old_source = SourceText::new("id\tname\n1\tJohn");
# let old_result = parser.parse(&old_source, &[], &mut session);
# let new_source = SourceText::new("id\tname\n1\tJohn Doe");
// Assuming you have an old parse result 'old_result' and new source text 'new_source'
let new_result = parser.parse(&new_source, &[], &mut session);
```

### 3. Diagnostics
`oak-tsv` provides rich error contexts specifically tailored for TSV data:
```rust
# use oak_tsv::{TsvParser, TsvLanguage};
# use oak_core::{Parser, source::SourceText, parser::session::ParseSession};
# let parser = TsvParser::new();
# let mut session = ParseSession::<TsvLanguage>::default();
# let source = SourceText::new("id\tname\n1");
# let result = parser.parse(&source, &[], &mut session);
for diag in result.diagnostics {
    println!("{:?}", diag);
}
```

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes TSV source text into a stream of tokens, handling tab delimiters, row separators, and complex quoting logic.
- **Parser**: Syntax analyzer based on the structural layout of records and fields.
- **AST**: A strongly-typed syntax abstraction layer designed for building high-performance data processing tools and editors.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/](tests/) for handling of various TSV dialects and edge cases.
