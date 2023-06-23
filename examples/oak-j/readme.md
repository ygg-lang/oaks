# 🚀 Oak J Parser

[![Crates.io](https://img.shields.io/crates/v/oak-j.svg)](https://crates.io/crates/oak-j)
[![Documentation](https://docs.rs/oak-j/badge.svg)](https://docs.rs/oak-j)

**Safety and Precision for Mission-Critical Systems** — A high-performance, incremental J parser built on the Oak framework. Optimized for safety-critical systems, large-scale engineering, and modern IDE support for the J programming language.

## 🎨 Project Vision

J was designed for high-level mathematical and data analysis, emphasizing reliability and maintainability. `oak-j` brings these same principles to parsing, providing a modern, Rust-powered infrastructure for analyzing J code. By utilizing Oak's incremental parsing capabilities, it enables developers to build highly responsive tools for complex J projects. Whether you are building static analyzers, automated refactoring tools, or sophisticated IDE extensions, `oak-j` provides the robust, efficient foundation you need.

## ✅ Core Features

- **⚡ Blazing Fast**: Leverages Rust's performance and memory safety to provide sub-millisecond parsing, essential for real-time analysis.
- **🔄 Incremental by Nature**: Built-in support for partial updates—re-parse only what has changed. Ideal for large J codebases where performance is critical.
- **🌳 High-Fidelity AST**: Generates a clean and easy-to-traverse Abstract Syntax Tree.
- **🛠️ Robust Error Recovery**: Engineered to handle incomplete or malformed code gracefully, providing precise diagnostics—crucial for maintaining a smooth developer experience during active coding.
- **🌐 Deep Ecosystem Integration**: Seamlessly works with `oak-lsp` for full LSP support and `oak-mcp` for Model Context Protocol capabilities.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn), which allows for:
1. **Efficient Immutability**: Share nodes across different versions of the tree without copying.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful code formatting and refactoring.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal and analysis.

## 🤝 Contributing

We welcome contributions of all kinds! If you find a bug, have a feature request, or want to contribute code, please check our [issues](https://github.com/ygg-lang/oaks/issues) or submit a pull request.
