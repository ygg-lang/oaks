# 🚀 Oak Markdown Parser

[![Crates.io](https://img.shields.io/crates/v/oak-markdown.svg)](https://crates.io/crates/oak-markdown)
[![Documentation](https://docs.rs/oak-markdown/badge.svg)](https://docs.rs/oak-markdown)

**The Standard for Modern Documentation** — A high-performance, incremental Markdown parser built on the Oak framework. Optimized for CommonMark, GFM features, and real-time editing experiences.

## 🎯 Project Vision

Markdown is the ubiquitous language for documentation, notes, and collaborative writing. `oak-markdown` aims to provide a robust, modern, Rust-powered infrastructure for parsing Markdown that is both accurate and incredibly fast. By utilizing Oak's incremental parsing architecture, we enable the creation of highly responsive editors, live previewers, and static site generators that can handle large documents with sub-millisecond latency. Whether you are building a custom documentation platform, a personal knowledge base tool, or a sophisticated IDE extension, `oak-markdown` provides the high-fidelity AST and efficiency needed to deliver a superior writing experience.

## ✨ Core Features

- **⚡ Blazing Fast**: Leverages Rust's performance and memory safety to provide sub-millisecond parsing, essential for real-time feedback in modern Markdown editors.
- **🔄 Incremental by Nature**: Built-in support for partial updates—re-parse only the sections of the document that changed. Ideal for large documentation projects and live-preview environments.
- **🌳 High-Fidelity AST**: Generates a comprehensive and precise Abstract Syntax Tree capturing the full structure of Markdown:
    - **Block Elements**: Precise mapping of Headings, Lists, Blockquotes, Code Blocks, and Tables.
    - **Inline Elements**: Robust handling of Emphasis, Links, Images, Inline Code, and Strikethrough.
    - **GFM Support**: Built-in support for GitHub Flavored Markdown extensions like Task Lists and Tables.
    - **Comments & Trivia**: Retains all whitespace and metadata, enabling faithful round-trip processing.
- **🛡️ Industrial-Grade Fault Tolerance**: Engineered to handle malformed Markdown gracefully, ensuring a continuous and stable parsing experience even during active editing.
- **🧩 Deep Ecosystem Integration**: Seamlessly works with `oak-lsp` for full LSP support and `oak-mcp` for intelligent documentation discovery and analysis.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn), which allows for:
1. **Efficient Immutability**: Share nodes across different versions of the tree without copying.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful code formatting and refactoring of Markdown files.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal and analysis.


## 🛠️ Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.
