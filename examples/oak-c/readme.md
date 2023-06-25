# 🚀 Oak C Parser

[![Crates.io](https://img.shields.io/crates/v/oak-c.svg)](https://crates.io/crates/oak-c)
[![Documentation](https://docs.rs/oak-c/badge.svg)](https://docs.rs/oak-c)

**Legacy Performance, Modern Tooling** — A high-performance, incremental C parser built on the Oak framework. Optimized for systems programming, legacy codebase analysis, and real-time developer tooling.

## 🎯 Project Vision

C is the foundation of modern computing, but its aging syntax and complex preprocessor make it a challenge for modern developer tools. `oak-c` provides a robust, high-performance, Rust-powered infrastructure for parsing C that is both accurate and incredibly fast. By utilizing Oak's incremental parsing architecture, we enable the creation of highly responsive IDEs, refactoring tools, and static analysis infrastructure that can handle massive C codebases and complex header chains in real-time.

## ✨ Core Features

- **⚡ Blazing Fast**: Leverages Rust's zero-cost abstractions to deliver sub-millisecond parsing, essential for real-time feedback in systems-level development.
- **🔄 Incremental by Design**: Built-in support for partial updates—re-parse only the sections of the C file that changed. Ideal for large-scale source files and complex build systems.
- **🌳 High-Fidelity AST**: Generates a comprehensive Abstract Syntax Tree capturing the full depth of C:
    - **Declarations & Definitions**: Precise mapping of variables, functions, structs, unions, and enums.
    - **Expressions & Statements**: Full support for C's complex operator precedence and control flow.
    - **Preprocessor Awareness**: Robust handling of directives and macro definitions.
    - **Comments & Formatting**: Retains all trivia, enabling faithful round-trip processing and refactoring.
- **🛡️ Industrial-Grade Fault Tolerance**: Engineered to recover from syntax errors gracefully, providing precise diagnostics—crucial for maintaining a smooth developer experience when editing complex C code.
- **🧩 Deep Ecosystem Integration**: Seamlessly works with `oak-lsp` for full LSP support and `oak-mcp` for intelligent code structure discovery.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn), which allows for:
1. **Efficient Immutability**: Share nodes across different versions of the tree without copying.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful code formatting and refactoring.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal and analysis.


## 🤝 Contributing

We welcome contributions of all kinds! If you find a bug, have a feature request, or want to contribute code, please check our [issues](https://github.com/ygg-lang/oaks/issues) or submit a pull request.
