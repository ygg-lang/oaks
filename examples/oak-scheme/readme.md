# 🚀 oak-scheme

[![Crates.io](https://img.shields.io/crates/v/oak-scheme.svg)](https://crates.io/crates/oak-scheme)
[![Documentation](https://docs.rs/oak-scheme/badge.svg)](https://docs.rs/oak-scheme)

**The Elegance of Lisp with the Power of Rust** — A high-performance, incremental Scheme parser built on the Oak framework. Optimized for various R*RS standards, complex macro systems, and responsive developer tools.

## 🎯 Project Vision

Scheme is celebrated for its minimalistic design and powerful macro system. `oak-scheme` aims to provide a robust, modern, Rust-powered infrastructure for parsing Scheme that is both accurate and incredibly fast. By utilizing Oak's incremental parsing architecture, we enable the creation of highly responsive IDEs, static analyzers, and refactoring tools that can handle complex Scheme projects in real-time. Whether you are building custom linters, automated code transformation tools, or sophisticated IDE extensions, `oak-scheme` provides the high-fidelity AST and efficiency needed to keep pace with the Scheme ecosystem.

## ✨ Core Features

- **⚡ Blazing Fast**: Leverages Rust's performance and memory safety to provide sub-millisecond parsing, essential for high-frequency developer tools and real-time analysis in large Scheme projects.
- **🔄 Incremental by Nature**: Built-in support for partial updates—re-parse only what has changed. Ideal for interactive development environments where tool responsiveness is critical.
- **🌳 High-Fidelity AST**: Generates a comprehensive and precise syntax tree capturing the full depth of Scheme:
    - **Standards Support**: Robust parsing of R5RS, R6RS, and R7RS syntax patterns.
    - **S-Expressions**: Detailed mapping of atoms, pairs, and lists.
    - **Macro Systems**: Precise handling of `syntax-rules` and other macro-related constructs.
    - **Indentation & Formatting**: Precise capture of indentation and whitespace for faithful code refactoring.
- **🛡️ Industrial-Grade Fault Tolerance**: Engineered to recover from syntax errors gracefully, providing precise diagnostics—crucial for maintaining a smooth developer experience during active coding.
- **🧩 Deep Ecosystem Integration**: Seamlessly works with `oak-lsp` for full LSP support and `oak-mcp` for intelligent code discovery and analysis.

## 🏗️ Architecture

`oak-scheme` follows the modern Green/Red Tree architecture (inspired by Roslyn):

- **Green Tree**: Immutable, lossless, and syntax-only tree. It captures the full fidelity of the source code, including trivia (comments, whitespace).
- **Red Tree**: A facade over the Green Tree that provides a convenient, type-safe API for tree traversal and analysis, including parent pointers and absolute offsets.

This design enables efficient incremental parsing and powerful refactoring capabilities.


## 🛠️ Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.
