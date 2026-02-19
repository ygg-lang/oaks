# 🚀 Oak Java Parser

[![Crates.io](https://img.shields.io/crates/v/oak-java.svg)](https://crates.io/crates/oak-java)
[![Documentation](https://docs.rs/oak-java/badge.svg)](https://docs.rs/oak-java)

**Enterprise-Grade Performance for the JVM Ecosystem.** `oak-java` is a high-performance, incremental Java parser built on the Oak framework. Optimized for modern Java features and large-scale enterprise systems, it delivers the speed and accuracy needed for real-time developer tools.

## ✨ Core Features

- **⚡ Blazing Fast**: Built in Rust for sub-millisecond parsing, essential for real-time feedback in massive enterprise codebases.
- **🔄 Truly Incremental**: Re-parse only what changed. Ideal for massive Java projects and responsive IDE feedback.
- **🛡️ Industrial-Grade Fault Tolerance**: Gracefully handles syntax errors, providing precise diagnostics for Java code.
- **🌳 High-Fidelity AST**: Captures every detail of Java, including records, annotations, and trivia.
- **Full Java Support**: Comprehensive coverage of records, sealed classes, and modules.
- **IDE Ready**: Designed from the ground up for Language Server Protocol (LSP) and MCP integration.
- **Lossless Syntax**: Retains all source details, making it perfect for automated refactoring and formatting.

## 🏗️ Modern Architecture

Leveraging the **Green/Red Tree** pattern (similar to Roslyn and rust-analyzer):
1. **Efficient Immutability**: Share nodes across tree versions without expensive copies.
2. **Type-Safe API**: Easy traversal with strongly-typed nodes.
3. **Zero-Copy Trivia**: Efficiently manage comments and whitespace.

## 🤝 Contributing

We welcome contributions! Whether it's bug reports, feature requests, or code, help us build the best Java parsing infrastructure. Check our [issues](https://github.com/ygg-lang/oaks/issues) or submit a PR.
