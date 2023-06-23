# 🚀 Oak Kotlin Parser

[![Crates.io](https://img.shields.io/crates/v/oak-kotlin.svg)](https://crates.io/crates/oak-kotlin)
[![Documentation](https://docs.rs/oak-kotlin/badge.svg)](https://docs.rs/oak-kotlin)

**Modern, Concise, and Incremental Parsing for Kotlin** — A high-performance Kotlin parser built on the Oak framework. Optimized for Kotlin 1.9+ features, multiplatform projects, and highly responsive IDE integration.

## 🎯 Project Vision

Kotlin has redefined modern JVM and multiplatform development with its concise syntax and powerful safety features. `oak-kotlin` aims to provide a premium, Rust-powered parsing infrastructure that matches Kotlin's expressiveness with incredible speed. By leveraging Oak's incremental parsing architecture, we enable the creation of ultra-responsive developer tools—from IDEs and static analyzers to custom refactoring engines—that can handle complex Kotlin codebases in real-time. Whether you are building linting tools for Android, KMP-aware analyzers, or sophisticated code generators, `oak-kotlin` provides the high-fidelity AST and sub-millisecond efficiency required for the next generation of Kotlin tooling.

## ✨ Core Features

- **⚡ Blazing Fast**: Fully utilizes Rust's performance and memory safety to achieve sub-millisecond parsing response times, essential for high-frequency analysis in modern IDEs.
- **🔄 Incremental by Design**: Built-in support for partial updates—re-parse only modified code blocks. This is a game-changer for large Kotlin projects and complex multiplatform setups.
- **🌳 High-Fidelity AST**: Generates a comprehensive and precise Abstract Syntax Tree capturing the full depth of modern Kotlin:
    - **Modern Features**: Full support for Data Classes, Sealed Classes, Context Receivers, and Context Parameters.
    - **Functional Programming**: Precise parsing of Lambdas, Higher-order functions, and Inline functions.
    - **Concurrency**: Deep integration of `suspend` functions and Coroutine-related constructs.
    - **Multiplatform (KMP)**: Robust handling of `expect`/`actual` declarations and platform-specific syntax.
    - **Type System**: Detailed mapping of Generics, Reified type parameters, and Type aliases.
- **🛡️ Industrial-Grade Fault Tolerance**: Engineered to recover from syntax errors gracefully, providing precise diagnostics—crucial for maintaining a fluid developer experience during active coding.
- **🧩 Deep Ecosystem Integration**: Seamlessly works with `oak-lsp` for full LSP support and `oak-mcp` for intelligent code discovery and analysis.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn), which allows for:
1. **Efficient Immutability**: Share nodes across different versions of the tree without copying.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful code formatting and refactoring of Kotlin files.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal and analysis.


## 🤝 Contributing

We welcome contributions of all kinds! If you find a bug, have a feature request, or want to contribute code, please check our [issues](https://github.com/ygg-lang/oaks/issues) or submit a pull request.
