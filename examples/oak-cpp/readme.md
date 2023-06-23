# 🚀 Oak C++ Parser

[![Crates.io](https://img.shields.io/crates/v/oak-cpp.svg)](https://crates.io/crates/oak-cpp)
[![Documentation](https://docs.rs/oak-cpp/badge.svg)](https://docs.rs/oak-cpp)

**Mastering Complexity with High Performance** — A high-performance, incremental C++ parser built on the Oak framework. Designed to navigate the vast and complex landscape of modern C++, from high-frequency trading systems to large-scale game engines.

## 🎯 Project Vision

C++ is notorious for its parsing complexity, yet it remains the language of choice for performance-critical systems. `oak-cpp` aims to provide a robust, modern, Rust-powered infrastructure for parsing C++ that is both accurate and incredibly fast. By utilizing Oak's incremental parsing architecture, we enable the creation of highly responsive tools that can handle modern C++ standards (up to C++23) in real-time, allowing developers to build sophisticated IDEs, refactoring tools, and static analyzers that keep pace with the language's evolution.

## ✨ Core Features

- **⚡ Blazing Performance**: Leverages Rust's memory safety and zero-cost abstractions to parse complex C++ templates and headers with sub-millisecond latency.
- **🔄 Incremental by Design**: Only re-parse what changed. Essential for maintaining high responsiveness in massive C++ projects with deep header dependencies.
- **🌳 High-Fidelity AST**: Generates a comprehensive Abstract Syntax Tree that captures the intricacies of C++:
    - **Templates**: Deep support for template declarations, specializations, and instantiations.
    - **Object-Oriented**: Precise mapping of classes, multiple inheritance, access specifiers, and virtual functions.
    - **Namespaces**: Robust handling of nested namespaces, aliases, and `using` directives.
    - **Modern C++**: Full support for `auto`, lambdas, `constexpr`, and other features from C++11 through C++23.
- **🛡️ Industrial-Grade Fault Tolerance**: Engineered to recover from syntax errors gracefully, providing precise diagnostics—crucial for analyzing code during active development in complex C++ environments.
- **🧩 Deep Ecosystem Integration**: Seamlessly works with `oak-lsp` for full LSP support and `oak-mcp` for intelligent, AI-driven code discovery.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn), which allows for:
1. **Efficient Immutability**: Share nodes across different versions of the tree without copying.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful code formatting and refactoring.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal and analysis.


## 🤝 Contributing

We welcome contributions of all kinds! If you find a bug, have a feature request, or want to contribute code, please check our [issues](https://github.com/ygg-lang/oaks/issues) or submit a pull request.
