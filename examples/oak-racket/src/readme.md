# 🛠️ Scheme Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-scheme`.

## 🚦 Quick Start

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
oak-scheme = { path = "..." }
```

### Basic Parsing Example

The following is a standard workflow for parsing Scheme scripts, supporting various R7RS and traditional Lisp constructs:

```rust
use oak_scheme::{SchemeParser, SourceText, SchemeLanguage};

fn main() {
    // 1. Prepare source code
    let code = r#"
        (define (square x)
          (* x x))
        
        (display (square 10))
        (newline)
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser
    let config = SchemeLanguage::new();
    let parser = SchemeParser::new(&config);

    // 3. Execute parsing
    let result = parser.parse(&source);

    // 4. Handle results
    if result.is_success() {
        println!("Parsing successful! AST node count: {}", result.node_count());
    } else {
        eprintln!("Errors found during parsing.");
        for diag in result.diagnostics() {
            println!("[{}:{}] {}", diag.line, diag.column, diag.message);
        }
    }
}
```

## 🔍 Core API Usage

### 1. Syntax Tree Traversal
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract Scheme specific constructs like lists, vectors, symbols, and procedure definitions.

### 2. Incremental Parsing
Scheme scripts can be part of larger systems. `oak-scheme` supports sub-millisecond incremental updates:
```rust
// Re-parse only the modified section
let new_result = parser.reparse(&new_source, &old_result);
```

### 3. Error Recovery
The parser is designed for industrial-grade fault tolerance, recovering gracefully from mismatched parentheses or malformed expressions to provide continuous feedback in IDEs.

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes Scheme source text, supporting standard S-expressions, symbols, strings, and numeric literals.
- **Parser**: A high-performance recursive descent parser optimized for the nested nature of Lisp-like languages.
- **AST**: A strongly-typed, lossless syntax tree that preserves all trivia (comments/whitespace) for refactoring and formatting tools.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/readme.md](tests/readme.md) for details on our snapshot-based testing.
