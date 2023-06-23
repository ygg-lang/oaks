# 🛠️ TypeScript Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-typescript`.

## 🚦 Quick Start

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
oak-typescript = { path = "..." }
```

### Basic Parsing Example

The following is a standard workflow for parsing modern TypeScript with interfaces, generics, and TSX:

```rust
use oak_typescript::{Parser, SourceText};

fn main() {
    // 1. Prepare source code
    let code = r#"
        interface User<T> {
            id: T;
            name: string;
            greet(): void;
        }

        const Greet = ({ name }: User<number>) => {
            return (
                <div className="welcome">
                    <h1>Hello, {name}!</h1>
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
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract TypeScript-specific constructs like interface declarations, type aliases, generic parameters, or TSX elements.

### 2. Incremental Parsing
No need to re-parse the entire file when small changes occur:
```rust
// Assuming you have an old parse result 'old_result' and new source text 'new_source'
let new_result = parser.reparse(&new_source, &old_result);
```

### 3. Diagnostics
`oak-typescript` provides rich error contexts specifically tailored for TS developers, handling complex scenarios like missing type annotations or malformed TSX:
```rust
for diag in result.diagnostics() {
    println!("[{}:{}] {}", diag.line, diag.column, diag.message);
}
```

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes TypeScript source text into a stream of tokens, including support for type-only tokens, decorators, and TSX-specific tags.
- **Parser**: Syntax analyzer based on the Pratt parsing algorithm to handle TypeScript's complex type-level syntax, expression precedence, and TSX integration.
- **AST**: A strongly-typed syntax abstraction layer designed for high-performance TS analysis tools, bundlers, and IDEs.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/](tests/) for handling of various TypeScript versions and TSX edge cases.
