# 🛠️ Swift Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-swift`.

## 🚦 Quick Start

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
oak-swift = { path = "..." }
```

### Basic Parsing Example

The following is a standard workflow for parsing modern Swift with property wrappers, generics, and concurrency:

```rust
use oak_swift::{SwiftParser, SourceText, SwiftLanguage};

fn main() {
    // 1. Prepare source code
    let code = r#"
        import Foundation

        @propertyWrapper
        struct Clamped<T: Comparable> {
            var wrappedValue: T
            let range: ClosedRange<T>
        }

        actor UserManager {
            @Clamped(range: 0...100)
            var userScore: Int = 50

            func updateScore(to newValue: Int) async {
                userScore = newValue
            }
        }
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser
    let config = SwiftLanguage::new();
    let parser = SwiftParser::new(&config);

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
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract Swift specific constructs like property wrappers, actors, async functions, and result builders.

### 2. Incremental Parsing
Swift projects (especially those using SwiftUI) can be very complex. `oak-swift` supports sub-millisecond incremental updates:
```rust
// Re-parse only the modified section
let new_result = parser.reparse(&new_source, &old_result);
```

### 3. SwiftUI and DSL Handling
The parser is optimized to handle SwiftUI's declarative syntax and other result-builder-based DSLs, providing accurate trees for UI definitions and complex logic.

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes Swift source text, including support for complex string literals (interpolations, multiline), custom operators, and various numeric formats.
- **Parser**: A high-performance syntax analyzer that handles Swift's sophisticated grammar, including generics, property wrappers, and modern concurrency features.
- **AST**: A strongly-typed, lossless syntax tree that preserves all trivia (comments/whitespace) for refactoring and formatting tools.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/readme.md](tests/readme.md) for handling of various Swift versions and edge cases.
