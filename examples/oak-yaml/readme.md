# 🚀 Oak YAML Parser

[![Crates.io](https://img.shields.io/crates/v/oak-yaml.svg)](https://crates.io/crates/oak-yaml)
[![Documentation](https://docs.rs/oak-yaml/badge.svg)](https://docs.rs/oak-yaml)

**The Power of Clarity and Flexibility** — A high-performance, incremental YAML parser built on the Oak framework. Optimized for configuration management, CI/CD pipeline analysis, and high-fidelity document processing.

## 🎯 Project Vision

YAML is the backbone of modern configuration and orchestration, but its whitespace-sensitive nature and complex features (like anchors and aliases) make it difficult to parse reliably. `oak-yaml` provides a robust, modern, Rust-powered infrastructure for parsing YAML that is both accurate and incredibly fast. By utilizing Oak's incremental parsing architecture, we enable the creation of highly responsive IDEs, configuration validators, and automated deployment tools that can handle massive YAML files and complex document streams in real-time.

## ✨ Core Features

- **⚡ Blazing Fast**: Leverages Rust's zero-cost abstractions to deliver sub-millisecond parsing, essential for real-time validation and large-scale configuration analysis.
- **🔄 Incremental by Design**: Built-in support for partial updates—re-parse only the sections of the YAML file that changed. Ideal for real-time editing of large Kubernetes manifests or CI/CD pipelines.
- **🌳 High-Fidelity AST**: Generates a comprehensive Abstract Syntax Tree capturing the full depth of YAML:
    - **Scalars, Sequences, and Mappings**: Precise mapping of all YAML core structures.
    - **Anchors & Aliases**: Full support for references and data reuse within documents.
    - **Comments & Formatting**: Retains all trivia, enabling faithful round-trip processing and refactoring.
- **🛡️ Industrial-Grade Fault Tolerance**: Engineered to recover from syntax errors gracefully, providing precise diagnostics—crucial for maintaining a smooth developer experience when editing complex configurations.
- **🧩 Deep Ecosystem Integration**: Seamlessly works with `oak-lsp` for full LSP support and `oak-mcp` for intelligent configuration discovery.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn), which allows for:
1. **Efficient Immutability**: Share nodes across different versions of the tree without copying.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful code formatting and refactoring.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal and analysis.


## 🤝 Contributing

We welcome contributions of all kinds! If you find a bug, have a feature request, or want to contribute code, please check our [issues](https://github.com/ygg-lang/oaks/issues) or submit a pull request.
