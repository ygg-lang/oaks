# 🚀 Oak Ada Parser

[![Crates.io](https://img.shields.io/crates/v/oak-ada.svg)](https://crates.io/crates/oak-ada)
[![Documentation](https://docs.rs/oak-ada/badge.svg)](https://docs.rs/oak-ada)

**Safety and Precision for Mission-Critical Systems** — A high-performance, incremental Ada parser built on the Oak framework. Optimized for safety-critical systems, large-scale engineering, and modern IDE support for the Ada programming language.

## 🎯 Project Vision

Ada was designed for safety-critical and large-scale software engineering, emphasizing reliability, readability, and maintainability. `oak-ada` brings these same principles to parsing, providing a modern, Rust-powered infrastructure for analyzing Ada code. By utilizing Oak's incremental parsing capabilities, it enables developers to build highly responsive tools for complex Ada projects, ensuring that safety and productivity go hand-in-hand. Whether you are building static analyzers, automated refactoring tools, or sophisticated IDE extensions, `oak-ada` provides the robust, efficient foundation you need.

## ✨ Core Features

- **⚡ Blazing Fast**: Leverages Rust's performance and memory safety to provide sub-millisecond parsing, essential for real-time analysis in large-scale systems.
- **🔄 Incremental by Nature**: Built-in support for partial updates—re-parse only what has changed. Ideal for large Ada codebases where performance is critical.
- **🌳 High-Fidelity AST**: Generates a clean and easy-to-traverse Abstract Syntax Tree capturing:
    - **Packages & Subprograms**: Comprehensive management of package specifications, bodies, and nested subprograms.
    - **Tasking & Concurrency**: Precise mapping of tasks, entries, and protected objects.
    - **Strong Typing**: Detailed tracking of Ada's unique type system, including range constraints and private types.
    - **Generics**: Robust handling of Ada generic units and formal parameters.
- **🛡️ Robust Error Recovery**: Engineered to handle incomplete or malformed code gracefully, providing precise diagnostics—crucial for maintaining a smooth developer experience during active coding.
- **🧩 Deep Ecosystem Integration**: Seamlessly works with `oak-lsp` for full LSP support and `oak-mcp` for Model Context Protocol capabilities.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn), which allows for:
1. **Efficient Immutability**: Share nodes across different versions of the tree without copying.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful code formatting and refactoring.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal and analysis.

## 🤝 Contributing

We welcome contributions of all kinds! If you find a bug, have a feature request, or want to contribute code, please check our [issues](https://github.com/ygg-lang/oaks/issues) or submit a pull request.
