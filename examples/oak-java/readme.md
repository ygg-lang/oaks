# 🚀 Oak Java Parser

[![Crates.io](https://img.shields.io/crates/v/oak-java.svg)](https://crates.io/crates/oak-java)
[![Documentation](https://docs.rs/oak-java/badge.svg)](https://docs.rs/oak-java)

**Enterprise-Grade Performance for the JVM Ecosystem** — A high-performance, incremental Java parser built on the Oak framework. Optimized for modern Java features, large-scale enterprise systems, and real-time developer tools.

## 🎯 Project Vision

Java powers some of the world's most complex and massive software systems. `oak-java` aims to provide a robust, modern, Rust-powered infrastructure for parsing Java that is both accurate and incredibly fast. By utilizing Oak's incremental parsing architecture, we enable the creation of highly responsive IDEs, static analyzers, and refactoring tools that can handle massive Java projects in real-time. Whether you are building custom linters, automated migration tools, or sophisticated IDE extensions, `oak-java` provides the high-fidelity AST and efficiency needed to keep pace with Java's rapid six-month release cycle and evolving feature set.

## ✨ Core Features

- **⚡ Blazing Fast**: Leverages Rust's performance and memory safety to provide sub-millisecond parsing, essential for high-frequency developer tools and real-time analysis in large enterprise codebases.
- **🔄 Incremental by Nature**: Built-in support for partial updates—re-parse only what has changed. Ideal for large-scale Java projects where maintainability and tool responsiveness are critical.
- **🌳 High-Fidelity AST**: Generates a comprehensive and precise Abstract Syntax Tree capturing the full depth of modern Java:
    - **Object-Oriented**: Deep support for classes, interfaces, inheritance, and access modifiers.
    - **Modern Java Features**: Full support for Records, Sealed Classes, and Pattern Matching for `switch`.
    - **Generics & Annotations**: Precise mapping of complex generic types and both built-in and custom annotations.
    - **Modules**: Robust handling of Java Platform Module System (Project Jigsaw) declarations.
    - **Functional Programming**: Support for Lambda expressions and Method References.
- **🛡️ Industrial-Grade Fault Tolerance**: Engineered to recover from syntax errors gracefully, providing precise diagnostics—crucial for maintaining a smooth developer experience during active coding in complex environments.
- **🧩 Deep Ecosystem Integration**: Seamlessly works with `oak-lsp` for full LSP support and `oak-mcp` for intelligent code discovery and analysis.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn), which allows for:
1. **Efficient Immutability**: Share nodes across different versions of the tree without copying.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful code formatting and refactoring.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal and analysis.


## 🤝 Contributing

We welcome contributions of all kinds! If you find a bug, have a feature request, or want to contribute code, please check our [issues](https://github.com/ygg-lang/oaks/issues) or submit a pull request.
