# 🚀 Oak XML Parser

[![Crates.io](https://img.shields.io/crates/v/oak-xml.svg)](https://crates.io/crates/oak-xml)
[![Documentation](https://docs.rs/oak-xml/badge.svg)](https://docs.rs/oak-xml)

**Structured Markup with Unmatched Speed** — A high-performance, incremental XML parser built on the Oak framework. Optimized for large-scale data interchange, document processing, and real-time validation.

## 🎯 Project Vision

XML remains a foundational technology for data representation and document structure, but its complex rules for tags, attributes, and namespaces require a robust and efficient parser. `oak-xml` provides a high-performance, Rust-powered infrastructure for parsing XML that is both accurate and incredibly fast. By utilizing Oak's incremental parsing architecture, we enable the creation of highly responsive IDEs, document editors, and data processing pipelines that can handle massive XML files and complex document trees in real-time.

## ✨ Core Features

- **⚡ Blazing Fast**: Leverages Rust's zero-cost abstractions to deliver sub-millisecond parsing, essential for real-time validation and large-scale document analysis.
- **🔄 Incremental by Design**: Built-in support for partial updates—re-parse only the sections of the XML file that changed. Ideal for real-time editing of large SVG files or data exports.
- **🌳 High-Fidelity AST**: Generates a comprehensive Abstract Syntax Tree capturing the full depth of XML:
    - **Elements & Attributes**: Precise mapping of start tags, end tags, self-closing tags, and attribute-value pairs.
    - **Namespaces**: Robust handling of XML namespaces and prefixes.
    - **CDATAs & Entities**: Full support for character data sections and entity references.
    - **Comments & Processing Instructions**: Retains all trivia, enabling faithful round-trip processing and refactoring.
- **🛡️ Industrial-Grade Fault Tolerance**: Engineered to recover from syntax errors gracefully, providing precise diagnostics—crucial for maintaining a smooth developer experience when editing complex markup.
- **🧩 Deep Ecosystem Integration**: Seamlessly works with `oak-lsp` for full LSP support and `oak-mcp` for intelligent document structure discovery.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn), which allows for:
1. **Efficient Immutability**: Share nodes across different versions of the tree without copying.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful code formatting and refactoring.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal and analysis.


## 🤝 Contributing

We welcome contributions of all kinds! If you find a bug, have a feature request, or want to contribute code, please check our [issues](https://github.com/ygg-lang/oaks/issues) or submit a pull request.
