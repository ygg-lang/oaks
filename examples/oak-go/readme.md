# 🚀 Oak Go Parser

[![Crates.io](https://img.shields.io/crates/v/oak-go.svg)](https://crates.io/crates/oak-go)
[![Documentation](https://docs.rs/oak-go/badge.svg)](https://docs.rs/oak-go)

**Simplicity at Scale** — A high-performance, incremental Go parser built on the Oak framework. Optimized for cloud-native toolchains, microservices, and modern IDE support for the Go programming language.

## 🎯 Project Vision

Go was designed for engineering at scale, emphasizing simplicity and fast compilation. `oak-go` brings these same principles to parsing, providing a modern, Rust-powered infrastructure for analyzing Go code. By utilizing Oak's incremental parsing capabilities, it enables developers to build highly responsive tools for massive Go monorepos, ensuring that developer productivity remains high even as projects grow. Whether you are building custom linters, automated refactoring tools, or sophisticated IDE extensions, `oak-go` provides the robust, efficient foundation you need.

## ✨ Core Features

- **⚡ Blazing Fast**: Leverages Rust's performance and memory safety to provide sub-millisecond parsing, essential for high-frequency developer tools and real-time analysis.
- **🔄 Incremental by Nature**: Built-in support for partial updates—re-parse only what has changed. Ideal for large-scale Go projects where performance is critical.
- **🌳 High-Fidelity AST**: Generates a clean and easy-to-traverse Abstract Syntax Tree capturing:
    - **Packages & Imports**: Comprehensive management of package declarations and complex import blocks.
    - **Types & Interfaces**: Precise mapping of structs, interfaces, and Go's unique structural typing system.
    - **Concurrency Constructs**: First-class support for `go` routines, `chan`nels, and `select` statements.
    - **Methods & Functions**: Detailed tracking of function signatures, receivers, and multiple return values.
- **🛡️ Robust Error Recovery**: Engineered to handle incomplete or malformed code gracefully, providing precise diagnostics—crucial for maintaining a smooth developer experience during active coding.
- **🧩 Deep Ecosystem Integration**: Seamlessly works with `oak-lsp` for full LSP support and `oak-mcp` for Model Context Protocol capabilities.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn), which allows for:
1. **Efficient Immutability**: Share nodes across different versions of the tree without copying.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful code formatting and refactoring.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal and analysis.


## 🤝 Contributing

We welcome contributions of all kinds! If you find a bug, have a feature request, or want to contribute code, please check our [issues](https://github.com/ygg-lang/oaks/issues) or submit a pull request.
