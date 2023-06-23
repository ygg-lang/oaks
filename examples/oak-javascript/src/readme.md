# 🛠️ JavaScript Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-javascript`.

## 🚦 Quick Start

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
oak-javascript = { path = "..." }
```

### Basic Parsing Example

The following is a standard workflow for parsing modern JavaScript with async functions and JSX:

```rust
use oak_javascript::{Parser, SourceText};

fn main() {
    // 1. Prepare source code
    let code = r#"
        import React from 'react';

        const Greet = async ({ name }) => {
            const message = await fetchMessage(name);
            return (
                <div className="greeting">
                    <h1>{message}</h1>
                </div>
            );
        };
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser
    let parser = Parser::new();

    // 3. Execute parsing
    let result = parser.parse(&source);

    // 4. Handle results
    if result.is_success() {
        println!("Parsing successful! AST node count: {}", result.node_count());
    } else {
        eprintln!("Errors found during parsing.");
    }
}
```

## 🔍 Core API Usage

### 1. Syntax Tree Traversal
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract JavaScript-specific constructs like arrow functions, JSX elements, async/await blocks, or ESM imports/exports.

### 2. Incremental Parsing
No need to re-parse the entire file when small changes occur:
```rust
// Assuming you have an old parse result 'old_result' and new source text 'new_source'
let new_result = parser.reparse(&new_source, &old_result);
```

### 3. Diagnostics
`oak-javascript` provides rich error contexts specifically tailored for JS developers, handling complex scenarios like missing semicolons (ASI awareness) or malformed JSX:
```rust
for diag in result.diagnostics() {
    println!("[{}:{}] {}", diag.line, diag.column, diag.message);
}
```

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes JavaScript source text into a stream of tokens, including support for template literals, JSX tags, and automatic semicolon insertion (ASI) hints.
- **Parser**: Syntax analyzer based on the Pratt parsing algorithm to handle JavaScript's expression precedence, complex async/await flow, and JSX integration.
- **AST**: A strongly-typed syntax abstraction layer designed for high-performance JS analysis tools, bundlers, and IDEs.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/](tests/) for handling of various ECMAScript versions and JSX edge cases.
