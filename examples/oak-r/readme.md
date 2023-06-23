# 🚀 Oak R Parser

[![Crates.io](https://img.shields.io/crates/v/oak-r.svg)](https://crates.io/crates/oak-r)
[![Documentation](https://docs.rs/oak-r/badge.svg)](https://docs.rs/oak-r)

**Statistical Power, Modern Engineering** — A high-performance, incremental R parser built on the Oak framework. Optimized for data science workflows, large-scale statistical analysis, and real-time developer tools.

## 🎯 Project Vision

R is the language of choice for statistical computing and graphics, but its flexible and sometimes idiosyncratic syntax can be challenging for robust tool development. `oak-r` provides a modern, Rust-powered infrastructure for parsing R that is both accurate and incredibly fast. By utilizing Oak's incremental parsing architecture, we enable the creation of highly responsive IDEs, code analyzers, and automated reporting tools that can handle massive R scripts and complex data processing pipelines in real-time.

## ✨ Core Features

- **⚡ Blazing Fast**: Leverages Rust's zero-cost abstractions to deliver sub-millisecond parsing, essential for high-frequency feedback in data-intensive environments.
- **🔄 Incremental by Design**: Built-in support for partial updates—re-parse only the sections of the R script that changed. Ideal for interactive data analysis and large-scale script processing.
- **🌳 High-Fidelity AST**: Generates a comprehensive Abstract Syntax Tree capturing the full depth of R:
    - **Functional Programming**: Precise mapping of functions, closures, and lazy evaluation constructs.
    - **Vectorized Operations**: Full support for R's unique array and matrix manipulation syntax.
    - **Statistical Formulas**: Robust handling of R's formula notation (`~`).
    - **Comments & Formatting**: Retains all trivia, enabling faithful round-trip processing and refactoring.
- **🛡️ Industrial-Grade Fault Tolerance**: Engineered to recover from syntax errors gracefully, providing precise diagnostics—crucial for maintaining a smooth developer experience during iterative analysis.
- **🧩 Deep Ecosystem Integration**: Seamlessly works with `oak-lsp` for full LSP support and `oak-mcp` for intelligent data structure discovery.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn), which allows for:
1. **Efficient Immutability**: Share nodes across different versions of the tree without copying.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful code formatting and refactoring.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal and analysis.


## 🤝 Contributing

We welcome contributions of all kinds! If you find a bug, have a feature request, or want to contribute code, please check our [issues](https://github.com/ygg-lang/oaks/issues) or submit a pull request.
