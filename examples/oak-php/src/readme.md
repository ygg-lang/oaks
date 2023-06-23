# 🛠️ PHP Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-php`.

## 🚦 Quick Start

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
oak-php = { path = "..." }
```

### Basic Parsing Example

The following is a standard workflow for parsing modern PHP with attributes, enums, and promoted properties:

```rust
use oak_php::{PhpParser, SourceText, PhpLanguage};

fn main() {
    // 1. Prepare source code
    let code = r#"
        <?php

        namespace App\Core;

        #[Attribute]
        enum Status: string {
            case Active = 'active';
            case Inactive = 'inactive';
        }

        class User {
            public function __construct(
                public readonly string $username,
                private Status $status = Status::Active,
            ) {}

            public function greet(): string {
                return "Hello, {$this->username}!";
            }
        }
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser
    let config = PhpLanguage::new();
    let parser = PhpParser::new(&config);

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
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract PHP specific constructs like class definitions, attributes, enums, promoted properties, and arrow functions.

### 2. Incremental Parsing
PHP projects (especially those using frameworks like Laravel or Symfony) can contain thousands of files. `oak-php` supports sub-millisecond incremental updates:
```rust
// Re-parse only the modified section
let new_result = parser.reparse(&new_source, &old_result);
```

### 3. Mixed Content Handling
The parser correctly handles mixed HTML and PHP files, identifying `<?php ... ?>` tags and maintaining the structural integrity of both the code and the surrounding template.

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes PHP source text, supporting complex string interpolations (`{$var}`), heredocs, nowdocs, and various numeric literal formats.
- **Parser**: A high-performance syntax analyzer that handles PHP's complex grammar, including modern 8.x features and legacy constructs.
- **AST**: A strongly-typed, lossless syntax tree that preserves all trivia (comments/whitespace) for refactoring and formatting tools.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/readme.md](tests/readme.md) for handling of various PHP versions and edge cases.
