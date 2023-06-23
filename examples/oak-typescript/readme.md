# 🚀 Oak TypeScript Parser

[![Crates.io](https://img.shields.io/crates/v/oak-typescript.svg)](https://crates.io/crates/oak-typescript)
[![Documentation](https://docs.rs/oak-typescript/badge.svg)](https://docs.rs/oak-typescript)

**Type Safety and Speed for the Modern Web** — A high-performance, incremental TypeScript parser built on the Oak framework. Optimized for modern TypeScript (5.0+) features, TSX support, and enterprise-grade developer tools.

## 🎯 Project Vision

TypeScript has become the standard for large-scale web development, but its complex type system and evolving syntax demand high-performance tooling. `oak-typescript` aims to provide a robust, modern, Rust-powered infrastructure for parsing TypeScript that is both accurate and incredibly fast. By utilizing Oak's incremental parsing architecture, we enable the creation of highly responsive IDEs, static analyzers, and refactoring tools that can handle massive TypeScript projects in real-time. Whether you are building custom type checkers, automated code migration tools, or sophisticated IDE extensions, `oak-typescript` provides the high-fidelity AST and efficiency needed to keep pace with TypeScript's rapid evolution.

## ✨ Core Features

- **⚡ Blazing Fast**: Leverages Rust's performance and memory safety to provide sub-millisecond parsing, essential for high-frequency developer tools and real-time analysis in large TypeScript codebases.
- **🔄 Incremental by Nature**: Built-in support for partial updates—re-parse only what has changed. Ideal for large-scale TypeScript projects where maintainability and tool responsiveness are critical.
- **🌳 High-Fidelity AST**: Generates a comprehensive and precise Abstract Syntax Tree capturing the full depth of modern TypeScript:
    - **Advanced Type System**: Full support for generics, interfaces, enums, type aliases, and union/intersection types.
    - **TSX Support**: First-class support for parsing TSX syntax, essential for modern React and web development.
    - **Decorators**: Support for the latest TC39/TypeScript decorator syntax.
    - **Modern Features**: Support for `satisfies` operator, `const` type parameters, and other recent TypeScript additions.
    - **Modules**: Robust handling of ESM (`import`/`export`) and legacy namespace/module systems.
- **🛡️ Industrial-Grade Fault Tolerance**: Engineered to recover from syntax errors gracefully, providing precise diagnostics—crucial for maintaining a smooth developer experience during active coding.
- **🧩 Deep Ecosystem Integration**: Seamlessly works with `oak-lsp` for full LSP support and `oak-mcp` for intelligent code discovery and analysis.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn), which allows for:
1. **Efficient Immutability**: Share nodes across different versions of the tree without copying.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful code formatting and refactoring.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal and analysis.

## 🤝 Contributing

We welcome contributions of all kinds! If you find a bug, have a feature request, or want to contribute code, please check our [issues](https://github.com/ygg-lang/oaks/issues) or submit a pull request.
