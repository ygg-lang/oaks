# 🛠️ Svelte Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-svelte`.

## 🚦 Quick Start

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
oak-svelte = { path = "..." }
```

### Basic Parsing Example

The following is a standard workflow for parsing Svelte components, supporting template, script, and style blocks:

```rust
use oak_svelte::{SvelteParser, SvelteLanguage};
use oak_core::{SourceText, parser::Parser, ParseSession};

fn main() {
    // 1. Prepare source code
    let code = r#"
        <script>
        let greeting = 'Hello from Oak!'
        </script>

        <h1>{greeting}</h1>
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser
    let config = SvelteLanguage::default();
    let parser = SvelteParser::new(&config);
    let mut session = ParseSession::default();

    // 3. Execute parsing
    let result = parser.parse(&source, &[], &mut session);

    // 4. Handle results
    if result.result.is_ok() {
        println!("Parsing successful!");
    } else {
        eprintln!("Errors found during parsing.");
    }
}
```

## 🔍 Core API Usage

### 1. Syntax Tree Traversal
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract Svelte specific constructs like logic blocks (`{#if}`, `{#each}`), directives (`on:`, `bind:`), and Svelte 5 snippets.

### 2. Incremental Parsing
Svelte components are often edited in IDEs. `oak-svelte` supports sub-millisecond incremental updates for a fluid editing experience:
```rust
# use oak_svelte::{SvelteParser, SvelteLanguage};
# use oak_core::{SourceText, parser::Parser, ParseSession};
# let config = SvelteLanguage::default();
# let parser = SvelteParser::new(&config);
# let mut session = ParseSession::default();
# let old_source = SourceText::new("<p>Hello</p>");
# let old_result = parser.parse(&old_source, &[], &mut session);
# let new_source = SourceText::new("<p>Hello World</p>");
// Re-parse only the modified section
let new_result = parser.parse(&new_source, &[], &mut session);
```

### 3. Error Recovery
The parser is designed for industrial-grade fault tolerance, recovering gracefully from unclosed tags or malformed logic blocks to provide continuous feedback in IDEs.

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes Svelte component source text, handling the transitions between HTML-like template syntax and logic blocks.
- **Parser**: A high-performance parser that manages the coordination between different sections within a single Svelte component.
- **AST**: A strongly-typed, lossless syntax tree that preserves all trivia (comments/whitespace) for refactoring and formatting tools.

## 🔗 Advanced Resources

- **Full Examples**: Check the `examples/` folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See `tests/` for details on our snapshot-based testing.
