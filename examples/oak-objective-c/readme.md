# 🚀 Oak Objective-C Parser

[![Crates.io](https://img.shields.io/crates/v/oak-objective-c.svg)](https://crates.io/crates/oak-objective-c)
[![Documentation](https://docs.rs/oak-objective-c/badge.svg)](https://docs.rs/oak-objective-c)

**Stability and Precision for Legacy Apple Ecosystems** — A high-performance, incremental Objective-C parser built on the Oak framework. Optimized for modern Objective-C 2.0 features, seamless C/C++ integration, and enterprise-grade code analysis tools.

## 🎯 Project Vision

Objective-C remains a critical part of the Apple developer ecosystem, powering billions of lines of code in macOS and iOS. `oak-objective-c` aims to provide a robust, modern, Rust-powered infrastructure for parsing Objective-C that is both accurate and incredibly fast. By utilizing Oak's incremental parsing architecture, we enable the creation of highly responsive IDEs, static analyzers, and refactoring tools that can handle massive Objective-C codebases in real-time. Whether you are building custom linters, automated migration tools to Swift, or sophisticated IDE extensions, `oak-objective-c` provides the high-fidelity AST and efficiency needed to manage the complexities of modern Objective-C development.

## ✨ Core Features

- **⚡ Blazing Fast**: Leverages Rust's performance and memory safety to provide sub-millisecond parsing, essential for high-frequency developer tools and real-time analysis in large Objective-C projects.
- **🔄 Incremental by Nature**: Built-in support for partial updates—re-parse only what has changed. Ideal for large-scale legacy apps where maintainability and tool responsiveness are critical.
- **🌳 High-Fidelity AST**: Generates a comprehensive and precise Abstract Syntax Tree capturing the full depth of modern Objective-C:
    - **Modern Features**: Full support for Properties, Categories, Protocols, and Blocks.
    - **C/C++ Integration**: Robust parsing of Objective-C code mixed with C and C++ constructs.
    - **Message Passing**: Precise mapping of Objective-C's unique message sending syntax (`[receiver message:arg]`).
    - **Memory Management**: Detailed tracking of ARC (Automatic Reference Counting) and manual retain/release patterns.
    - **Attributes**: Support for `__attribute__`, `nullable`, `nonnull`, and other modern compiler directives.
- **🛡️ Industrial-Grade Fault Tolerance**: Engineered to recover from syntax errors gracefully, providing precise diagnostics—crucial for maintaining a smooth developer experience during active coding.
- **🧩 Deep Ecosystem Integration**: Seamlessly works with `oak-lsp` for full LSP support and `oak-mcp` for intelligent code discovery and analysis.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn), which allows for:
1. **Efficient Immutability**: Share nodes across different versions of the tree without copying.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful code formatting and refactoring of Objective-C files.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal and analysis.


## 🤝 Contributing

We welcome contributions of all kinds! If you find a bug, have a feature request, or want to contribute code, please check our [issues](https://github.com/ygg-lang/oaks/issues) or submit a pull request.
