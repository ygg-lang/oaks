# 🚀 Oak D Parser

[![Crates.io](https://img.shields.io/crates/v/oak-d.svg)](https://crates.io/crates/oak-d)
[![Documentation](https://docs.rs/oak-d/badge.svg)](https://docs.rs/oak-d)

**Systems Programming with Modern Agility** — A high-performance, incremental D parser built on the Oak framework. Optimized for systems programming, performance-critical applications, and modern developer tooling.

## 🎯 Project Vision

D is a language that combines the power and performance of C++ with the programmer productivity of modern languages like Python and Ruby. `oak-d` aims to provide a robust, modern, Rust-powered infrastructure for parsing D that is both accurate and incredibly fast. By utilizing Oak's incremental parsing architecture, we enable the creation of highly responsive IDEs, refactoring tools, and static analyzers that can handle large D codebases in real-time.

## ✨ Core Features

- **⚡ Blazing Fast**: Leverages Rust's performance and memory safety to provide sub-millisecond parsing, essential for high-frequency developer tools and real-time analysis.
- **🔄 Incremental by Design**: Built-in support for partial updates—re-parse only what changed. Ideal for large-scale D projects where maintainability and tool responsiveness are critical.
- **🌳 High-Fidelity AST**: Generates a comprehensive Abstract Syntax Tree capturing the full depth of D:
    - **Metaprogramming**: Deep support for templates, mixins, and compile-time function execution (CTFE).
    - **Object-Oriented & Functional**: Precise mapping of classes, interfaces, and functional programming constructs.
    - **Memory Management**: Awareness of D's various memory management strategies (GC, manual, reference counting).
    - **Comments & Whitespace**: Retains all trivia, enabling faithful round-trip processing and refactoring.
- **🛡️ Industrial-Grade Fault Tolerance**: Engineered to recover from syntax errors gracefully, providing precise diagnostics—crucial for maintaining a smooth developer experience during active coding.
- **🧩 Deep Ecosystem Integration**: Seamlessly works with `oak-lsp` for full LSP support and `oak-mcp` for intelligent code discovery and analysis.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn), which allows for:
1. **Efficient Immutability**: Share nodes across different versions of the tree without copying.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful code formatting and refactoring.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal and analysis.


## 🤝 Contributing

We welcome contributions of all kinds! If you find a bug, have a feature request, or want to contribute code, please check our [issues](https://github.com/ygg-lang/oaks/issues) or submit a pull request.
