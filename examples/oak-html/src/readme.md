# 🛠️ HTML Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-html`.

## 🚦 Quick Start

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
oak-html = { path = "..." }
```

### Basic Parsing Example

The following is a standard workflow for parsing a modern HTML5 document with attributes and nested elements:

```rust
use oak_html::{HtmlParser, SourceText, HtmlLanguage};

fn main() {
    // 1. Prepare source code
    let code = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <title>Oak HTML Example</title>
        </head>
        <body>
            <div id="app" class="container">
                <h1>Hello, Oak!</h1>
                <img src="logo.png" alt="Oak Logo" />
            </div>
            <script src="app.js"></script>
        </body>
        </html>
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser
    let config = HtmlLanguage::new();
    let parser = HtmlParser::new(&config);

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
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract HTML-specific constructs like element tags, attribute values, text content, or specific `script` and `style` blocks.

### 2. Incremental Parsing
No need to re-parse a massive HTML document when small changes occur:
```rust
// Assuming you have an old parse result 'old_result' and new source text 'new_source'
let new_result = parser.reparse(&new_source, &old_result);
```

### 3. Diagnostics
`oak-html` provides rich error contexts specifically tailored for web developers, handling complex scenarios like unclosed tags or malformed attribute syntax:
```rust
for diag in result.diagnostics() {
    println!("[{}:{}] {}", diag.line, diag.column, diag.message);
}
```

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes HTML source text into a stream of tokens, including support for tags, attributes, text nodes, and special handling for `script`/`style` content.
- **Parser**: Syntax analyzer based on the Pratt parsing algorithm to handle HTML's hierarchical structure, void elements, and self-closing tags.
- **AST**: A strongly-typed syntax abstraction layer designed for high-performance HTML analysis tools, scrapers, and IDEs.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/](tests/) for handling of various HTML5 edge cases and "tag soup."
