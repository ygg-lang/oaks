# 🚀 Oak VB.NET Parser

[![Crates.io](https://img.shields.io/crates/v/oak-vbnet.svg)](https://crates.io/crates/oak-vbnet)
[![Documentation](https://docs.rs/oak-vbnet/badge.svg)](https://docs.rs/oak-vbnet)

**Modern, Concise, and Incremental Parsing for VB.NET** — A high-performance VB.NET parser built on the Oak framework. Optimized for VB.NET 10+ features and highly responsive IDE integration.

## 🎯 Project Vision

VB.NET remains a widely used language in enterprise environments, especially for Windows applications and services. `oak-vbnet` aims to provide a premium, Rust-powered parsing infrastructure that enables the creation of responsive developer tools for VB.NET codebases. By leveraging Oak's incremental parsing architecture, we enable the creation of ultra-responsive developer tools—from IDEs and static analyzers to custom refactoring engines—that can handle complex VB.NET codebases in real-time.

## ✨ Core Features

- **⚡ Blazing Fast**: Fully utilizes Rust's performance and memory safety to achieve sub-millisecond parsing response times, essential for high-frequency analysis in modern IDEs.
- **🔄 Incremental by Design**: Built-in support for partial updates—re-parse only modified code blocks. This is a game-changer for large VB.NET projects.
- **🌳 High-Fidelity AST**: Generates a comprehensive and precise Abstract Syntax Tree capturing the full depth of modern VB.NET:
    - **Modern Features**: Full support for LINQ, async/await, and other modern VB.NET features.
    - **Object-Oriented Programming**: Precise parsing of classes, interfaces, and inheritance hierarchies.
    - **Type System**: Detailed mapping of generics, delegates types, and extension methods.
- **🛡️ Industrial-Grade Fault Tolerance**: Engineered to recover from syntax errors gracefully, providing precise diagnostics—crucial for maintaining a fluid developer experience during active coding.
- **🧩 Deep Ecosystem Integration**: Seamlessly works with `oak-lsp` for full LSP support and `oak-mcp` for intelligent code discovery and analysis.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn), which allows for:
1. **Efficient Immutability**: Share nodes across different versions of the tree without copying.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful code formatting and refactoring of VB.NET files.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal and analysis.


## 🤝 Contributing

We welcome contributions of all kinds! If you find a bug, have a feature request, or want to contribute code, please check our [issues](https://github.com/ygg-lang/oaks/issues) or submit a pull request.