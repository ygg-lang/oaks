# 🚀 Oak Valkyrie Parser

[![Crates.io](https://img.shields.io/crates/v/oak-valkyrie.svg)](https://crates.io/crates/oak-valkyrie)
[![Documentation](https://docs.rs/oak-valkyrie/badge.svg)](https://docs.rs/oak-valkyrie)

**Making VALKYRIE processing simple** — A high-performance, incremental VALKYRIE parser built on the Oak framework. Specially optimized for large-scale codebases and modern IDE integration for the Valkyrie programming language.

## 🎯 Project Vision

`oak-valkyrie` is dedicated to providing industrial-grade parsing support for the VALKYRIE language. By leveraging Rust's high-performance characteristics and Oak's incremental parsing architecture, it can easily handle a variety of application scenarios, from simple script analysis to complex IDE language servers. Our goal is to empower developers with a robust, efficient, and high-fidelity parsing infrastructure that serves as the foundation for the next generation of Valkyrie developer tools.

## ✨ Core Features

- **⚡ Blazing Fast**: Fully utilizes Rust's performance advantages to achieve sub-millisecond parsing response times, even for complex Valkyrie constructs.
- **🔄 Incremental Parsing**: Built-in support for partial updates—re-parse only what has changed, demonstrating extremely high efficiency when processing large files.
- **🌳 High-Fidelity AST**: Generates a clean and easy-to-traverse Abstract Syntax Tree capturing:
    - **Namespaces & Scopes**: Precise tracking of namespace declarations and hierarchical scopes.
    - **Micro Functions**: First-class support for Valkyrie's micro function definitions and call sites.
    - **Strongly Typed Expressions**: Detailed mapping of complex expressions, including field access and indexing.
    - **Fault-Tolerant Statements**: Robust handling of variable bindings and control flow structures.
- **🛡️ Robust Error Recovery**: Engineered to handle incomplete or malformed code gracefully, providing precise diagnostics while maintaining parser state.
- **🧩 Easy Integration**: Designed with high cohesion and low coupling, allowing for quick integration into existing Rust projects and Oak-based tools.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn), which allows for:
1. **Efficient Immutability**: Share nodes across different versions of the tree without copying.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful code formatting and refactoring.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal and analysis.

## 🤝 Contributing

We welcome contributions of all kinds! If you find a bug, have a feature request, or want to contribute code, please check our [issues](https://github.com/ygg-lang/oaks/issues) or submit a pull request.
