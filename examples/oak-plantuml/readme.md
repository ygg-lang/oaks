# 🚀 Oak PlantUML Parser

[![Crates.io](https://img.shields.io/crates/v/oak-plantuml.svg)](https://crates.io/crates/oak-plantuml)
[![Documentation](https://docs.rs/oak-plantuml/badge.svg)](https://docs.rs/oak-plantuml)

**The Power of PlantUML, Reimagined for Performance** — A high-performance, incremental PlantUML parser built on the Oak framework. Optimized for large-scale system modeling, architectural documentation, and modern developer tooling.

## 🎯 Project Vision

PlantUML is a versatile tool for creating UML diagrams from a simple and intuitive text description. `oak-plantuml` brings this versatility to the Rust ecosystem, providing a high-performance, incremental parser that can handle even the most massive modeling projects. By utilizing Oak's advanced parsing capabilities, it enables the creation of highly responsive IDEs and automated modeling tools that can analyze and visualize complex systems in real-time. Whether you are managing large enterprise architectures or building custom modeling solutions, `oak-plantuml` provides the robust foundation you need.

## ✨ Core Features

- **⚡ Blazing Fast**: Leverages Rust's performance to provide sub-millisecond parsing, even for large and complex model files.
- **🔄 Incremental by Nature**: Built-in support for partial updates—re-parse only what has changed. Essential for large-scale modeling projects.
- **🌳 High-Fidelity AST**: Generates a clean and easy-to-traverse Abstract Syntax Tree capturing:
    - **UML Elements**: Comprehensive mapping of classes, interfaces, components, and use cases.
    - **Relationships**: Precise tracking of associations, generalizations, and dependencies.
    - **Notes & Annotations**: Detailed capture of comments, notes, and metadata.
    - **Styling & Themes**: Support for PlantUML's extensive styling and skinparam directives.
- **🛡️ Robust Error Recovery**: Engineered to handle incomplete or malformed code gracefully, providing precise diagnostics—crucial for a smooth developer experience.
- **🧩 Deep Ecosystem Integration**: Seamlessly works with `oak-lsp` for full LSP support and `oak-mcp` for intelligent model discovery.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn), which allows for:
1. **Efficient Immutability**: Share nodes across different versions of the tree without copying.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful code formatting and refactoring.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal and analysis.

## 🤝 Contributing

We welcome contributions of all kinds! If you find a bug, have a feature request, or want to contribute code, please check our [issues](https://github.com/ygg-lang/oaks/issues) or submit a pull request.
