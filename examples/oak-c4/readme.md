# 🚀 Oak C4 Model

[![Crates.io](https://img.shields.io/crates/v/oak-c4.svg)](https://crates.io/crates/oak-c4)
[![Documentation](https://docs.rs/oak-c4/badge.svg)](https://docs.rs/oak-c4)

**Semantic Modeling for Modern Architecture** — A high-performance, incremental C4 model semantic model built on the Oak framework. Optimized for architectural analysis, documentation generators, and intelligent software modeling tools.

## 🎯 Project Vision

The C4 model is a lean graphical notation technique for modelling the architecture of software systems. `oak-c4` provides the semantic backbone for representing C4 models within the Oak ecosystem. Unlike simple parsers, `oak-c4` focuses on the semantic relationships and structural integrity of the architecture model itself. By utilizing Oak's incremental capabilities, it enables real-time architectural validation and intelligent documentation updates. Whether you are building automated architecture compliance tools or sophisticated design-time analyzers, `oak-c4` provides the robust semantic foundation you need.

## ✨ Core Features

- **⚡ Blazing Fast**: Leverages Rust's performance for sub-millisecond semantic analysis.
- **🔄 Incremental by Nature**: Built-in support for partial model updates—re-validate only what has changed.
- **🌳 Semantic Tree**: Generates a high-fidelity representation of the C4 hierarchy:
    - **Software Systems & People**: First-class representation of top-level architectural elements.
    - **Containers & Components**: Precise mapping of the internal structure of systems.
    - **Relationships**: Deep tracking of semantic dependencies and interactions.
- **🛡️ Robust Validation**: Engineered to identify architectural inconsistencies and provide clear, actionable feedback.
- **🧩 Deep Ecosystem Integration**: Seamlessly works with `oak-structurizr` and other Oak modeling projects.

## 🏗️ Architecture

The model follows the **Green/Red Tree** architecture (inspired by Roslyn), which allows for:
1. **Efficient Immutability**: Share nodes across different versions of the model without copying.
2. **Lossless Representation**: Retains all semantic metadata and relationships.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for model traversal and analysis.

## 🤝 Contributing

We welcome contributions of all kinds! If you find a bug, have a feature request, or want to contribute code, please check our [issues](https://github.com/ygg-lang/oaks/issues) or submit a pull request.
