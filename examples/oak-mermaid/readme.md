# 🚀 Oak Mermaid Parser

[![Crates.io](https://img.shields.io/crates/v/oak-mermaid.svg)](https://crates.io/crates/oak-mermaid)
[![Documentation](https://docs.rs/oak-mermaid/badge.svg)](https://docs.rs/oak-mermaid)

**Flowcharts and Beyond, Powered by Oak** — A high-performance, incremental Mermaid diagramming language parser built on the Oak framework. Optimized for documentation systems, developer tools, and real-time diagram rendering.

## 🎯 Project Vision

Mermaid has become the de facto standard for embedding diagrams in Markdown and documentation. `oak-mermaid` provides a modern, Rust-powered infrastructure for parsing Mermaid diagrams with high fidelity and performance. By leveraging Oak's incremental parsing architecture, it enables the creation of highly responsive documentation editors and previewers that update complex flowcharts, sequence diagrams, and Gantt charts instantly. Our goal is to provide the most robust and efficient foundation for the next generation of Mermaid-powered developer tools.

## ✨ Core Features

- **⚡ Blazing Fast**: Leverages Rust's zero-cost abstractions to deliver sub-millisecond parsing for even the most complex diagrams.
- **🔄 Incremental by Design**: Built-in support for partial updates—re-parse only the sections of the diagram that changed.
- **🌳 High-Fidelity AST**: Generates a comprehensive Abstract Syntax Tree capturing the full variety of Mermaid diagrams:
    - **Flowcharts**: Precise mapping of nodes, shapes, and directed/undirected edges.
    - **Sequence Diagrams**: Detailed tracking of participants, messages, and activations.
    - **Class & State Diagrams**: Comprehensive management of entities and transitions.
    - **Gantt & Pie Charts**: Accurate parsing of data-driven diagram types.
- **🛡️ Industrial-Grade Fault Tolerance**: Engineered to recover from syntax errors gracefully, providing precise diagnostics—essential for live editing environments.
- **🧩 Deep Ecosystem Integration**: Seamlessly works with `oak-lsp` for full LSP support and `oak-mcp` for intelligent diagram analysis.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn), which allows for:
1. **Efficient Immutability**: Share nodes across different versions of the tree without copying.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful code formatting and refactoring.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal and analysis.

## 🤝 Contributing

We welcome contributions of all kinds! If you find a bug, have a feature request, or want to contribute code, please check our [issues](https://github.com/ygg-lang/oaks/issues) or submit a pull request.
