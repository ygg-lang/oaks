# 🚀 Oak EJS Template Parser

[![Crates.io](https://img.shields.io/crates/v/oak-ejs.svg)](https://crates.io/crates/oak-ejs)
[![Documentation](https://docs.rs/oak-ejs/badge.svg)](https://docs.rs/oak-ejs)

**Embedded JavaScript Templates for the Oak Ecosystem** — A high-performance, incremental EJS template parser built on the Oak framework. Optimized for server-side rendering, static site generation, and real-time template processing.

## 🎯 Project Vision

EJS (Embedded JavaScript) is one of the most popular templating engines for JavaScript, enabling developers to embed JavaScript code directly within HTML templates. `oak-ejs` provides a robust, Rust-powered infrastructure for parsing EJS templates that is both accurate and incredibly fast. By utilizing Oak's incremental parsing architecture, we enable the creation of highly responsive template engines, static site generators, and server-side rendering tools.

## ✨ Core Features

- **⚡ Blazing Fast**: Leverages Rust's performance and memory safety to provide sub-millisecond parsing, essential for high-throughput template rendering.
- **🔄 Incremental by Nature**: Built-in support for partial updates—re-parse only what has changed. Ideal for hot-reloading during development.
- **🌳 High-Fidelity AST**: Generates a comprehensive Abstract Syntax Tree capturing the full structure of EJS templates:
    - **Output Expressions**: `<%= ... %>` for HTML-escaped output, `<%- ... %>` for raw output.
    - **Code Blocks**: `<% ... %>` for arbitrary JavaScript code execution.
    - **Comments**: `<%# ... %>` for template comments that are not rendered.
    - **Escaped Tags**: `<%%` renders as literal `<%` in the output.
    - **Trim Mode**: `-%>` trims the following newline.
- **🛡️ Industrial-Grade Fault Tolerance**: Engineered to recover from syntax errors gracefully, providing precise diagnostics.
- **🧩 Deep Ecosystem Integration**: Seamlessly works with `oak-lsp` for full LSP support and template intelligence.

## 📖 EJS Syntax Reference

| Syntax | Description |
|--------|-------------|
| `<% code %>` | Execute JavaScript code (no output) |
| `<%= value %>` | Output HTML-escaped value |
| `<%- value %>` | Output raw value (no escaping) |
| `<%# comment %>` | Comment (not rendered) |
| `<%%` | Literal `<%` in output |
| `-%>` | Trim following newline |

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn), which allows for:
1. **Efficient Immutability**: Share nodes across different versions of the tree without copying.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful template formatting.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal and analysis.

## 🚦 Quick Start

```rust
use oak_ejs::{EjsLanguage, EjsLexer, EjsParser, EjsElementType};
use oak_core::SourceText;

fn main() {
    let template = r#"<h1>Hello, <%= name %>!</h1>"#;
    let source = SourceText::new(template);
    let config = EjsLanguage::default();
    
    let parser = EjsParser::new(&config);
    // Parse and process the template...
}
```

## 🤝 Contributing

We welcome contributions of all kinds! If you find a bug, have a feature request, or want to contribute code, please check our [issues](https://github.com/ygg-lang/oaks/issues) or submit a pull request.
