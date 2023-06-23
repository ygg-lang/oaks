# 🛠️ C# Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-csharp`.

## 🚦 Quick Start

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
oak-csharp = { path = "..." }
```

### Basic Parsing Example

The following is a standard workflow for parsing modern C# with records, LINQ, and primary constructors:

```rust
use oak_csharp::{CsharpParser, SourceText, CsharpLanguage};

fn main() {
    // 1. Prepare source code
    let code = r#"
        using System;
        using System.Linq;

        namespace Oak.Core;

        public record User(string Id, string Name);

        public class UserService(ILogger logger) {
            private readonly List<User> _users = [];

            public async Task<User?> GetUserAsync(string id) {
                logger.LogInformation("Getting user {Id}", id);
                return _users.FirstOrDefault(u => u.Id == id);
            }
        }
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser
    let config = CsharpLanguage::new();
    let parser = CsharpParser::new(&config);

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
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract C# specific constructs like records, primary constructors, async methods, LINQ expressions, and attributes.

### 2. Incremental Parsing
C# solutions can contain thousands of files. `oak-csharp` supports sub-millisecond incremental updates:
```rust
// Re-parse only the modified section
let new_result = parser.reparse(&new_source, &old_result);
```

### 3. Roslyn Compatibility
While built in Rust, the architecture is inspired by Roslyn (Green/Red Trees), making it familiar for developers coming from the .NET ecosystem and enabling high-performance analysis tools.

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes C# source text, supporting verbatim strings, interpolated strings, raw string literals, and various numeric formats.
- **Parser**: A high-performance syntax analyzer that handles C#'s complex grammar, including modern 12.0+ features and legacy constructs.
- **AST**: A strongly-typed, lossless syntax tree that preserves all trivia (comments/whitespace) for refactoring and formatting tools.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/readme.md](tests/readme.md) for handling of various C# versions and edge cases.
