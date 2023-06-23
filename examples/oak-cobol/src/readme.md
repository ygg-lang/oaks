# 🛠️ COBOL Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-cobol`.

## 🚦 Quick Start

### Basic Parsing Example

The following is a standard workflow for parsing a simple COBOL program:

```rust
use oak_cobol::{CobolParser, SourceText, CobolLanguage};

fn main() {
    // 1. Prepare source code
    let code = r#"
       IDENTIFICATION DIVISION.
       PROGRAM-ID. HELLO-WORLD.
       PROCEDURE DIVISION.
           DISPLAY 'Hello, Oak COBOL!'.
           STOP RUN.
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser
    let config = CobolLanguage::new();
    let parser = CobolParser::new(&config);

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
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract COBOL constructs like division headers, data items, or procedure statements.

### 2. Incremental Parsing
No need to re-parse massive COBOL files when small changes occur:
```rust
// Assuming you have an old parse result 'old_result' and new source text 'new_source'
let new_result = parser.reparse(&new_source, &old_result);
```

### 3. Diagnostics
`oak-cobol` provides rich error contexts specifically tailored for COBOL developers:
```rust
for diag in result.diagnostics() {
    println!("[{}:{}] {}", diag.line, diag.column, diag.message);
}
```

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes COBOL source text into a stream of tokens, handling keywords (case-insensitive), picture clauses, and fixed/free format rules.
- **Parser**: Syntax analyzer based on the Pratt parsing algorithm to handle COBOL's unique verb-based syntax and division structure.
- **AST**: A strongly-typed syntax abstraction layer designed for building high-performance COBOL analysis tools and modernization engines.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/](tests/) for handling of various COBOL dialects and legacy edge cases.
