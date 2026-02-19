# 🚀 Oak C Parser

[![Crates.io](https://img.shields.io/crates/v/oak-c.svg)](https://crates.io/crates/oak-c)
[![Documentation](https://docs.rs/oak-c/badge.svg)](https://docs.rs/oak-c)

**Legacy Performance, Modern Tooling.** `oak-c` is a high-performance, incremental C parser built on the Oak framework. Optimized for systems programming, legacy codebase analysis, and real-time developer tooling, it brings modern infrastructure to the C ecosystem.

## ✨ Core Features

- **⚡ Blazing Fast**: Built in Rust for sub-millisecond parsing, essential for real-time feedback in massive systems-level codebases.
- **🔄 Truly Incremental**: Re-parse only what changed. Ideal for massive translation units and responsive IDE feedback.
- **🛡️ Industrial-Grade Fault Tolerance**: Gracefully handles syntax errors, providing precise diagnostics for complex C code.
- **🌳 High-Fidelity AST**: Captures every detail of C, including declarations, expressions, preprocessor awareness, and trivia.
- **Full C Support**: Comprehensive coverage of functions, structs, unions, and enums.
- **Preprocessor Aware**: Robust handling of directives and macro definitions.
- **IDE Ready**: Designed from the ground up for Language Server Protocol (LSP) and MCP integration.
- **Lossless Syntax**: Retains all source details, making it perfect for automated refactoring and formatting.

## 🏗️ Modern Architecture

Leveraging the **Green/Red Tree** pattern (similar to Roslyn and rust-analyzer):
1. **Efficient Immutability**: Share nodes across tree versions without expensive copies.
2. **Type-Safe API**: Easy traversal with strongly-typed nodes.
3. **Zero-Copy Trivia**: Efficiently manage comments and whitespace.

## 🤝 Contributing

We welcome contributions! Whether it's bug reports, feature requests, or code, help us build the best C parsing infrastructure. Check our [issues](https://github.com/ygg-lang/oaks/issues) or submit a PR.
