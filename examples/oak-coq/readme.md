# 🚀 Oak Coq Parser

[![Crates.io](https://img.shields.io/crates/v/oak-coq.svg)](https://crates.io/crates/oak-coq)
[![Documentation](https://docs.rs/oak-coq/badge.svg)](https://docs.rs/oak-coq)

**Formally Verified Development at Scale** — A high-performance, incremental Coq parser built on the Oak framework. Optimized for interactive theorem proving, formal verification workflows, and modern IDE support for the Coq proof assistant.

## 🎯 Project Vision

Coq is a powerful formal proof management system used to develop verified software and mathematical proofs. `oak-coq` brings modern parsing infrastructure to the Coq ecosystem, providing a high-performance, Rust-powered alternative for analyzing Coq scripts. By utilizing Oak's incremental parsing capabilities, it enables the creation of highly responsive IDEs and interactive tools that can handle large formal developments with sub-millisecond latency. Whether you are building automated proof search tools, documentation generators, or sophisticated IDE extensions, `oak-coq` provides the robust foundation needed for deep analysis of Gallina and Ltac constructs.

## ✨ Core Features

- **⚡ Blazing Fast**: Leverages Rust's performance and memory safety to parse complex Coq scripts with exceptional speed.
- **🔄 Incremental by Nature**: Built-in support for partial updates—re-parse only what has changed. Ideal for providing immediate feedback during interactive proof development.
- **🌳 High-Fidelity AST**: Generates a precise Abstract Syntax Tree capturing the full structure of Coq developments:
    - **Vernacular Commands**: Comprehensive mapping of definitions, theorems, lemmas, and module structures.
    - **Gallina Terms**: Precise representation of types, functions, and inductive definitions.
    - **Ltac & Tactics**: Robust support for proof scripts and custom tactic definitions.
    - **Notations**: Handles complex user-defined notations and syntax extensions.
- **🛡️ Robust Error Recovery**: Engineered to handle incomplete or malformed scripts gracefully, providing precise diagnostics—essential for maintaining a smooth proof engineering experience.
- **🧩 Deep Ecosystem Integration**: Seamlessly works with `oak-lsp` for full LSP support and `oak-mcp` for intelligent proof discovery and formal analysis.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn), which allows for:
1. **Efficient Immutability**: Share nodes across different versions of the tree without copying.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful code formatting and refactoring.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal and analysis.

## 🤝 Contributing

We welcome contributions of all kinds! If you find a bug, have a feature request, or want to contribute code, please check our [issues](https://github.com/ygg-lang/oaks/issues) or submit a pull request.
