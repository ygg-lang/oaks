# 🛠️ Zig Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-zig`.

## 🚦 Quick Start

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
oak-zig = { path = "..." }
```

### Basic Parsing Example

The following is a standard workflow for parsing a Zig source file:

```rust
use oak_zig::{ZigParser, SourceText, ZigLanguage};

fn main() {
    // 1. Prepare Zig source code
    let code = r#"
        const std = @import("std");

        pub fn main() !void {
            const stdout = std.io.getStdOut().writer();
            try stdout.print("Hello, {s}!\n", .{"Oak"});
        }
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser
    let config = ZigLanguage::new();
    let parser = ZigParser::new(&config);

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
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract Zig constructs like function definitions, comptime blocks, or struct declarations.

### 2. Incremental Parsing
Zig projects often involve many small files or frequent edits. `oak-zig` supports sub-millisecond incremental updates:
```rust
// Re-parse only the modified section
let new_result = parser.reparse(&new_source, &old_result);
```

### 3. Error Recovery
The parser is designed for industrial-grade fault tolerance, recovering gracefully from missing semicolons or malformed `try/catch` blocks to provide continuous feedback in IDEs.

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes Zig source text, supporting Zig-specific features like multi-line strings, character literals, and preprocessor-like `@import`.
- **Parser**: A high-performance recursive descent parser with Pratt parsing for expressions, handling Zig's operator precedence and complex compile-time logic.
- **AST**: A strongly-typed, lossless syntax tree that preserves all trivia (comments/whitespace) for refactoring and formatting tools.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/readme.md](tests/readme.md) for details on our snapshot-based testing.
