# 🚀 Oak HTML Parser

[![Crates.io](https://img.shields.io/crates/v/oak-html.svg)](https://crates.io/crates/oak-html)
[![Documentation](https://docs.rs/oak-html/badge.svg)](https://docs.rs/oak-html)

**Structuring the Web with Precision** — A high-performance, incremental HTML parser built on the Oak framework. Optimized for web scraping, static analysis, and modern IDE support for web development.

## 🎯 Project Vision

HTML is the backbone of the web, and its complexity often arises from its flexibility and real-world "tag soup." `oak-html` aims to provide a robust, high-performance parsing solution that can handle modern HTML5 standards with industrial-grade reliability. By utilizing Oak's incremental parsing capabilities, it enables the creation of highly responsive tools for web development—from real-time preview engines to intelligent code refactoring tools.

## ✨ Core Features

- **⚡ Blazing Fast**: Leverages Rust's performance to deliver sub-millisecond parsing, essential for real-time web development tools and large-scale web analysis.
- **🔄 Incremental Parsing**: Built-in support for partial updates—re-parse only the sections of the HTML that changed, significantly improving performance for complex web pages.
- **🌳 High-Fidelity AST**: Generates a detailed and easy-to-traverse Abstract Syntax Tree capturing:
    - Elements, Attributes, and nested structures
    - Comments, Doctype declarations, and Text nodes
    - Support for modern HTML5 features
- **🛡️ Industrial-Grade Error Recovery**: Engineered to handle malformed or "tag soup" HTML gracefully, providing precise diagnostics while maintaining a valid tree structure.
- **🧩 Ecosystem Integration**: Part of the Oak family—easily integrate with `oak-lsp` for full LSP support or other Oak-based web analysis utilities.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn), which allows for:
1. **Efficient Immutability**: Share nodes across different versions of the tree without copying.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful code formatting and refactoring.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal and analysis.


## 🤝 Contributing

We welcome contributions of all kinds! If you find a bug, have a feature request, or want to contribute code, please check our [issues](https://github.com/ygg-lang/oaks/issues) or submit a pull request.
