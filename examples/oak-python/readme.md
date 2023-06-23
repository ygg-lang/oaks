# 🚀 Oak Python Parser

[![Crates.io](https://img.shields.io/crates/v/oak-python.svg)](https://crates.io/crates/oak-python)
[![Documentation](https://docs.rs/oak-python/badge.svg)](https://docs.rs/oak-python)

**Elegance and Speed for the Python Ecosystem** — A high-performance, incremental Python parser built on the Oak framework. Optimized for modern Python (3.10+) features, type checking, and real-time developer tools.

## 🎯 Project Vision

Python's growth has led to massive codebases where analysis and tooling speed are paramount. `oak-python` aims to provide a robust, Rust-powered parsing infrastructure that can handle the unique challenges of Python's indentation-based syntax while delivering sub-millisecond performance. By leveraging Oak's incremental parsing architecture, we enable the creation of highly responsive IDEs, static analyzers, and refactoring tools that can process large Python projects in real-time. Whether you are building a custom type checker, a security scanner, or an advanced code completion engine, `oak-python` provides the high-fidelity AST and efficiency needed to keep pace with Python's rapid evolution.

## ✨ Core Features

- **⚡ Blazing Fast**: Leverages Rust's performance and memory safety to provide sub-millisecond parsing, essential for high-frequency developer tools and real-time analysis.
- **🔄 Incremental by Nature**: Built-in support for partial updates—re-parse only modified code blocks. Ideal for large Python projects and real-time feedback loops.
- **🌳 High-Fidelity AST**: Generates a comprehensive Abstract Syntax Tree capturing the full depth of modern Python:
    - **Indentation Awareness**: Precise tracking of indentation levels for correct block scope identification.
    - **Type Hints**: Full support for PEP 484 type annotations and modern type syntax (e.g., `|` for unions).
    - **Async/Await**: Deep integration of asynchronous programming constructs.
    - **Pattern Matching**: Robust support for PEP 634 structural pattern matching (`match` and `case`).
    - **Decorators & F-Strings**: Detailed mapping of function/class decorators and complex f-string expressions.
- **🛡️ Industrial-Grade Fault Tolerance**: Engineered to handle incomplete or malformed code gracefully, providing precise diagnostics—crucial for maintaining a smooth developer experience during active coding.
- **🧩 Deep Ecosystem Integration**: Seamlessly works with `oak-lsp` for full LSP support and `oak-mcp` for intelligent code discovery and analysis.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn), which allows for:
1. **Efficient Immutability**: Share nodes across different versions of the tree without copying.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful code formatting and refactoring.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal and analysis.


## 🤝 Contributing

We welcome contributions of all kinds! If you find a bug, have a feature request, or want to contribute code, please check our [issues](https://github.com/ygg-lang/oaks/issues) or submit a pull request.
