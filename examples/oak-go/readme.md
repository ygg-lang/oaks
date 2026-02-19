# 🚀 Oak Go Parser

[![Crates.io](https://img.shields.io/crates/v/oak-go.svg)](https://crates.io/crates/oak-go)
[![Documentation](https://docs.rs/oak-go/badge.svg)](https://docs.rs/oak-go)

**Simplicity at Scale.** `oak-go` is a high-performance, incremental Go parser built on the Oak framework. Engineered for cloud-native toolchains, microservices, and modern IDE support, it brings Rust's power to the Go ecosystem.

## ✨ Core Features

- **⚡ Blazing Fast**: Built in Rust for sub-millisecond parsing, essential for real-time feedback in Go codebases.
- **🔄 Truly Incremental**: Re-parse only what changed. Ideal for massive Go monorepos and responsive IDE feedback.
- **🛡️ Industrial-Grade Fault Tolerance**: Gracefully handles syntax errors, providing precise diagnostics for Go code.
- **🌳 High-Fidelity AST**: Captures every detail of Go, including concurrency constructs, defer/recover, and trivia.
- **Full Go Support**: Comprehensive coverage of packages, interfaces, and methods.
- **IDE Ready**: Designed from the ground up for Language Server Protocol (LSP) and MCP integration.
- **Lossless Syntax**: Retains all source details, making it perfect for automated refactoring and formatting.

## 🏗️ Modern Architecture

Leveraging the **Green/Red Tree** pattern (similar to Roslyn and rust-analyzer):
1. **Efficient Immutability**: Share nodes across tree versions without expensive copies.
2. **Type-Safe API**: Easy traversal with strongly-typed nodes.
3. **Zero-Copy Trivia**: Efficiently manage comments and whitespace.

## 🤝 Contributing

We welcome contributions! Whether it's bug reports, feature requests, or code, help us build the best Go parsing infrastructure. Check our [issues](https://github.com/ygg-lang/oaks/issues) or submit a PR.
