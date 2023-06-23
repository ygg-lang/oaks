# 🛠️ TeX/LaTeX Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-tex`.

## 🚦 Quick Start

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
oak-tex = { path = "..." }
```

### Basic Parsing Example

The following is a standard workflow for parsing TeX/LaTeX documents, supporting commands, environments, and math mode:

```rust
use oak_tex::{TexParser, SourceText, TexLanguage};

fn main() {
    // 1. Prepare source code
    let code = r#"
        \documentclass{article}
        \begin{document}
        The value of $\pi$ is approximately 3.14159.
        \end{document}
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser
    let config = TexLanguage::new();
    let parser = TexParser::new(&config);

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
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract TeX specific constructs like commands, environments, math mode groups, and plain text content.

### 2. Incremental Parsing
TeX documents can be very large (e.g., theses, books). `oak-tex` supports sub-millisecond incremental updates:
```rust
// Re-parse only the modified section
let new_result = parser.reparse(&new_source, &old_result);
```

### 3. Error Recovery
The parser is designed for industrial-grade fault tolerance, recovering gracefully from unclosed environments or missing braces to provide continuous feedback in IDEs.

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes TeX source text, supporting control sequences, category codes (catcodes), and special characters.
- **Parser**: A high-performance recursive descent parser that handles TeX's macro-based expansion and environment nesting.
- **AST**: A strongly-typed, lossless syntax tree that preserves all trivia (comments/whitespace) for refactoring and formatting tools.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/readme.md](tests/readme.md) for details on our snapshot-based testing.
