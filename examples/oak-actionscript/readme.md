# 🚀 Oak ActionScript Parser

[![Crates.io](https://img.shields.io/crates/v/oak-actionscript.svg)](https://crates.io/crates/oak-actionscript)
[![Documentation](https://docs.rs/oak-actionscript/badge.svg)](https://docs.rs/oak-actionscript)

**Bringing Legacy Code Back to Life** — A high-performance, incremental ActionScript 3.0 parser built on the Oak framework. Specially optimized for Adobe Flash, Apache Flex development, and automated code migration.

## 🎯 Project Vision

In today's development landscape, legacy systems written in ActionScript 3.0 often lack modern toolchain support. `oak-actionscript` aims to bridge this gap by providing enterprise-grade parsing capabilities using modern Rust infrastructure. Whether you are building IDE plugins, static analysis tools, or complex code refactoring systems, this parser provides the robust, efficient foundation you need.

## ✨ Core Features

- **⚡ Blazing Fast**: Leverages Rust's memory safety and zero-cost abstractions to deliver sub-millisecond parsing performance.
- **🔄 Incremental Parsing**: Built-in support for partial updates—only re-parse what changed, making it ideal for real-time IDE integration and large-scale codebases.
- **🌳 Comprehensive AST**: Generates a high-fidelity Abstract Syntax Tree that captures every detail:
    - Package and Namespace declarations
    - Class and Interface inheritance hierarchies
    - Metadata/Annotations (e.g., `[Bindable]`, `[Event]`)
    - E4X (ECMAScript for XML) extensions
- **🛡️ Industrial-Grade Fault Tolerance**: Engineered to handle malformed input gracefully. It can recover from syntax errors and continue parsing, providing precise diagnostics to help developers fix issues quickly.
- **🧩 Seamless Integration**: Part of the Oak ecosystem—easily combine it with `oak-lsp` for full Language Server Protocol support or `oak-semantic-search` for intelligent code discovery.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn), which allows for:
1. **Efficient Immutability**: Share nodes across different versions of the tree without copying.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful code formatting and refactoring.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal and analysis.

## 🤝 Contributing

We welcome contributions of all kinds! If you find a bug, have a feature request, or want to contribute code, please check our [issues](https://github.com/ygg-lang/oaks/issues) or submit a pull request.
