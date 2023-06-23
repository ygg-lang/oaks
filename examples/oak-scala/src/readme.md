# 🛠️ Scala Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-scala`.

## 🚦 Quick Start

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
oak-scala = { path = "..." }
```

### Basic Parsing Example

The following is a standard workflow for parsing Scala source files, supporting Scala 2 and Scala 3 syntax:

```rust
use oak_scala::{ScalaParser, SourceText, ScalaLanguage};

fn main() {
    // 1. Prepare source code
    let code = r#"
        package com.example

        import scala.util.Random

        object Dice {
            def roll(): Int = {
                Random.nextInt(6) + 1
            }
        }
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser
    let config = ScalaLanguage::new();
    let parser = ScalaParser::new(&config);

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
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract Scala specific constructs like package clauses, imports, object/class/trait definitions, and method declarations.

### 2. Incremental Parsing
Scala projects can be large. `oak-scala` supports sub-millisecond incremental updates for efficient IDE feedback:
```rust
// Re-parse only the modified section
let new_result = parser.reparse(&new_source, &old_result);
```

### 3. Error Recovery
The parser is designed for industrial-grade fault tolerance, recovering gracefully from missing braces or malformed declarations to provide continuous feedback in IDEs.

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes Scala source text, supporting Scala 3's indentation-based syntax and Scala 2's traditional syntax.
- **Parser**: A high-performance recursive descent parser handling complex Scala features like implicits, generics, and pattern matching.
- **AST**: A strongly-typed, lossless syntax tree that preserves all trivia (comments/whitespace) for refactoring and formatting tools.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/readme.md](tests/readme.md) for details on our snapshot-based testing.
