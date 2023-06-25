# 🛠️ Koka Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-koka`.

## 🚦 Quick Start

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
oak-koka = { path = "..." }
```

### Basic Parsing Example

The following is a standard workflow for parsing modern Koka with data classes, coroutines, and lambdas:

```rust
use oak_koka::{KokaParser, SourceText, KokaLanguage};

fn main() {
    // 1. Prepare source code
    let code = r#"
        package com.example.oak

        import kokax.coroutines.*

        data class User(val id: String, val name: String)

        class Repository {
            private val users = mutableListOf<User>()

            suspend fn addUser(user: User) = withContext(Dispatchers.IO) {
                users.add(user)
                println("User ${user.name} added!")
            }
        }
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser
    let config = KokaLanguage::new();
    let parser = KokaParser::new(&config);

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
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract Koka specific constructs like data classes, sealed classes, suspend functions, lambdas, and property delegates.

### 2. Incremental Parsing
Koka projects (especially Android or KMP projects) can be massive. `oak-koka` supports sub-millisecond incremental updates:
```rust
// Re-parse only the modified section
let new_result = parser.reparse(&new_source, &old_result);
```

### 3. KMP and Multiplatform Support
The parser correctly handles Koka Multiplatform (KMP) specific syntax like `expect`/`actual` declarations, ensuring accurate trees across all targets.

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes Koka source text, supporting complex string templates, backticked identifiers, and various numeric literal formats.
- **Parser**: A high-performance syntax analyzer that handles Koka's expressive grammar, including generics, coroutines, and modern language features.
- **AST**: A strongly-typed, lossless syntax tree that preserves all trivia (comments/whitespace) for refactoring and formatting tools.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/readme.md](tests/readme.md) for handling of various Koka versions and edge cases.
