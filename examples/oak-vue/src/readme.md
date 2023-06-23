# 🛠️ Vue Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-vue`.

## 🚦 Quick Start

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
oak-vue = { path = "..." }
```

### Basic Parsing Example

The following is a standard workflow for parsing Vue Single File Components (SFCs), supporting template, script, and style blocks:

```rust
use oak_vue::{VueParser, SourceText, VueLanguage};

fn main() {
    // 1. Prepare source code
    let code = r#"
        <template>
          <div>{{ greeting }}</div>
        </template>

        <script setup>
        const greeting = 'Hello from Oak!'
        </script>
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser
    let config = VueLanguage::new();
    let parser = VueParser::new(&config);

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
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract Vue specific constructs like top-level blocks, template directives, script setup variables, and scoped styles.

### 2. Incremental Parsing
Vue SFCs are often edited in IDEs. `oak-vue` supports sub-millisecond incremental updates for a fluid editing experience:
```rust
// Re-parse only the modified section
let new_result = parser.reparse(&new_source, &old_result);
```

### 3. Error Recovery
The parser is designed for industrial-grade fault tolerance, recovering gracefully from unclosed tags or malformed script blocks to provide continuous feedback in IDEs.

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes Vue SFC source text, handling the transitions between HTML-like template syntax, JavaScript/TypeScript in script blocks, and CSS/SCSS in style blocks.
- **Parser**: A high-performance parser that manages the coordination between different languages within a single SFC.
- **AST**: A strongly-typed, lossless syntax tree that preserves all trivia (comments/whitespace) for refactoring and formatting tools.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/readme.md](tests/readme.md) for details on our snapshot-based testing.
