# 🚀 Oak APL Parser

[![Crates.io](https://img.shields.io/crates/v/oak-apl.svg)](https://crates.io/crates/oak-apl)
[![Documentation](https://docs.rs/oak-apl/badge.svg)](https://docs.rs/oak-apl)

**Concise and Powerful Array Programming** — A high-performance, incremental APL parser built on the Oak framework. Optimized for array-oriented languages, mathematical notation, and modern IDE support for the APL programming language.

## 🎯 Project Vision

APL was designed for mathematical expression and array manipulation, emphasizing conciseness, power, and symbolic representation. `oak-apl` brings these same principles to parsing, providing a modern, Rust-powered infrastructure for analyzing APL code. By utilizing Oak's incremental parsing capabilities, it enables developers to build highly responsive tools for complex APL projects. Whether you are building static analyzers, mathematical tools, or sophisticated IDE extensions, `oak-apl` provides the robust, efficient foundation you need.

## ✨ Core Features

- **⚡ Blazing Fast**: Leverages Rust's performance and memory safety to provide sub-millisecond parsing, essential for real-time analysis.
- **🔄 Incremental by Nature**: Built-in support for partial updates—re-parse only what has changed. Ideal for interactive APL environments.
- **🌳 High-Fidelity AST**: Generates a clean and easy-to-traverse Abstract Syntax Tree capturing:
    - **Arrays & Vectors**: Comprehensive management of APL's core array structures.
    - **Functions & Operators**: Precise mapping of primitive and user-defined functions and operators.
    - **Symbolic Notation**: Robust handling of APL's unique character set and syntax.
- **🛡️ Robust Error Recovery**: Engineered to handle incomplete or malformed code gracefully, providing precise diagnostics.
- **🧩 Deep Ecosystem Integration**: Seamlessly works with `oak-lsp` for full LSP support and `oak-mcp` for Model Context Protocol capabilities.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn), which allows for:
1. **Efficient Immutability**: Share nodes across different versions of the tree without copying.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful code formatting and refactoring.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal and analysis.

## 🤝 Contributing

We welcome contributions of all kinds! If you find a bug, have a feature request, or want to contribute code, please check our [issues](https://github.com/ygg-lang/oaks/issues) or submit a pull request.
