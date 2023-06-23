# 🛠️ Bash Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-bash`.

## 🚦 Quick Start

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
oak-bash = { path = "..." }
```

### Basic Parsing Example

The following is a standard workflow for parsing a Bash script with functions, loops, and redirections:

```rust
use oak_bash::{BashParser, SourceText, BashLanguage};

fn main() {
    // 1. Prepare source code
    let code = r#"
        #!/bin/bash

        function greet() {
            local name=$1
            echo "Hello, ${name}!" >&2
        }

        for i in {1..5}; do
            greet "User $i"
        done
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser
    let config = BashLanguage::new();
    let parser = BashParser::new(&config);

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
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract Bash specific constructs like function definitions, command pipelines, variable expansions, and redirections.

### 2. Incremental Parsing
Bash scripts can sometimes be long and complex. `oak-bash` supports sub-millisecond incremental updates:
```rust
// Re-parse only the modified section
let new_result = parser.reparse(&new_source, &old_result);
```

### 3. Expansion Awareness
The parser identifies different types of shell expansions (parameter, command, arithmetic), providing a solid foundation for building advanced static analysis tools.

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes Bash source text, handling complex rules for word splitting, quoting (single, double, ANSI-C), and comment identification.
- **Parser**: A high-performance syntax analyzer that handles Bash's flexible grammar, including control structures and redirections.
- **AST**: A strongly-typed, lossless syntax tree that preserves all trivia (comments/whitespace) for refactoring and formatting tools.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/readme.md](tests/readme.md) for handling of various Bash syntax edge cases.
