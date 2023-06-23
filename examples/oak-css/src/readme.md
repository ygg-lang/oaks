# 🛠️ CSS Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-css`.

## 🚦 Quick Start

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
oak-css = { path = "..." }
```

### Basic Parsing Example

The following is a standard workflow for parsing a modern CSS file with variables, nesting, and media queries:

```rust
use oak_css::{CssParser, SourceText, CssLanguage};

fn main() {
    // 1. Prepare source code
    let code = r#"
        :root {
            --primary-color: #3498db;
        }

        .container {
            display: grid;
            grid-template-columns: repeat(3, 1fr);
            gap: 20px;

            & .item {
                background-color: var(--primary-color);
                padding: 1rem;
            }
        }

        @media (max-width: 768px) {
            .container {
                grid-template-columns: 1fr;
            }
        }
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser
    let config = CssLanguage::new();
    let parser = CssParser::new(&config);

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
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract CSS-specific constructs like rule sets, selectors, declarations, media queries, or custom property definitions.

### 2. Incremental Parsing
No need to re-parse a massive CSS file when small changes occur:
```rust
// Assuming you have an old parse result 'old_result' and new source text 'new_source'
let new_result = parser.reparse(&new_source, &old_result);
```

### 3. Diagnostics
`oak-css` provides rich error contexts specifically tailored for designers and developers, handling complex scenarios like malformed selectors or invalid property values:
```rust
for diag in result.diagnostics() {
    println!("[{}:{}] {}", diag.line, diag.column, diag.message);
}
```

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes CSS source text into a stream of tokens, including support for identifiers, strings, numbers with units, and special characters used in selectors.
- **Parser**: Syntax analyzer based on the Pratt parsing algorithm to handle CSS's rule-based structure, complex selector precedence, and nested rules.
- **AST**: A strongly-typed syntax abstraction layer designed for high-performance CSS analysis tools, post-processors, and IDEs.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/](tests/) for handling of various CSS3/CSS4 edge cases and browser-specific hacks.
