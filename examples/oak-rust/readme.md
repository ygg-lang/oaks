# 🚀 Oak Rust Parser

[![Crates.io](https://img.shields.io/crates/v/oak-rust.svg)](https://crates.io/crates/oak-rust)
[![Documentation](https://docs.rs/oak-rust/badge.svg)](https://docs.rs/oak-rust)

**Safety and Speed for the System** — A high-performance, incremental Rust parser built on the Oak framework. Designed for building the next generation of Rust developer tools, IDEs, and static analyzers.

## 🎯 Project Vision

Rust has redefined systems programming with its focus on memory safety and performance. `oak-rust` aims to provide a parsing infrastructure that is as robust and efficient as the language itself. By leveraging Oak's incremental parsing architecture, we enable the creation of highly responsive tools that can handle large Rust projects with ease. Whether you're building a custom linter, a code refactoring engine, or an advanced IDE extension, `oak-rust` provides the high-fidelity AST and sub-millisecond performance required to keep up with Rust's rapid evolution and complex syntax.

## ✨ Core Features

- **⚡ Blazing Fast**: Engineered in Rust to parse Rust—achieving sub-millisecond latency through zero-cost abstractions and efficient memory management.
- **🔄 Incremental by Design**: Only re-parse what changed. Essential for providing real-time feedback in large-scale Rust crates and workspaces.
- **🌳 High-Fidelity AST**: Generates a comprehensive and precise Abstract Syntax Tree capturing the full depth of Rust:
    - **Ownership & Lifetimes**: Accurate representation of borrowing, ownership transitions, and explicit lifetime annotations.
    - **Generics & Traits**: Deep support for trait bounds, associated types, and complex generic constraints.
    - **Macros**: Robust handling of both declarative (`macro_rules!`) and procedural macros.
    - **Pattern Matching**: Precise mapping of complex `match` arms, destructuring, and guard expressions.
    - **Async/Await**: First-class support for modern asynchronous programming constructs.
- **🛡️ Industrial-Grade Fault Tolerance**: Gracefully recovers from syntax errors during active development, providing clear and actionable diagnostics.
- **🧩 Deep Ecosystem Integration**: Seamlessly works with `oak-lsp` for full LSP support and `oak-mcp` for intelligent code discovery and analysis.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn), which allows for:
1. **Efficient Immutability**: Share nodes across different versions of the tree without copying.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful code formatting and refactoring.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal and analysis.


## 🤝 Contributing

We welcome contributions of all kinds! If you find a bug, have a feature request, or want to contribute code, please check our [issues](https://github.com/ygg-lang/oaks/issues) or submit a pull request.
