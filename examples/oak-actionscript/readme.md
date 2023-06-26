# 🚀 Oak ActionScript Parser

[![Crates.io](https://img.shields.io/crates/v/oak-actionscript.svg)](https://crates.io/crates/oak-actionscript)
[![Documentation](https://docs.rs/oak-actionscript/badge.svg)](https://docs.rs/oak-actionscript)

**Modern Tooling for Legacy Code.** `oak-actionscript` is a high-performance, incremental ActionScript 3.0 parser built on the Oak framework. It's engineered for building modern IDEs, static analysis tools, and automated migration paths for Flash and Flex applications.

## ✨ Core Features

- **⚡ Blazing Fast**: Built in Rust for sub-millisecond parsing performance and zero-cost abstractions.
- **🔄 Truly Incremental**: Re-parse only what changed. Ideal for real-time IDE feedback and massive codebases.
- **🛡️ Industrial-Grade Fault Tolerance**: Gracefully handles malformed code with intelligent error recovery.
- **🌳 High-Fidelity AST**: Captures every detail, including metadata (`[Bindable]`), E4X, and even whitespace/comments (trivia).
- **Full AS3 Support**: Comprehensive coverage of packages, classes, interfaces, and namespaces.
- **E4X Integration**: First-class support for ECMAScript for XML extensions.
- **Metadata Aware**: Easily extract and analyze annotations like `[Event]`, `[Inject]`, etc.
- **IDE Ready**: Designed from the ground up for Language Server Protocol (LSP) integration.

## 🏗️ Modern Architecture

Leveraging the **Green/Red Tree** pattern (similar to Roslyn and rust-analyzer):
1. **Efficient Immutability**: Share nodes across tree versions without expensive copies.
2. **Lossless Syntax**: Perfect for automated refactoring and code formatting.
3. **Type-Safe API**: Easy traversal with strongly-typed nodes.

## 🤝 Contributing

We welcome contributions! Whether it's bug reports, feature requests, or code, help us bring modern tooling to the ActionScript ecosystem. Check our [issues](https://github.com/ygg-lang/oaks/issues) or submit a PR.
