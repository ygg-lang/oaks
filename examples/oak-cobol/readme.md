# 🚀 Oak COBOL Parser

[![Crates.io](https://img.shields.io/crates/v/oak-cobol.svg)](https://crates.io/crates/oak-cobol)
[![Documentation](https://docs.rs/oak-cobol/badge.svg)](https://docs.rs/oak-cobol)

**Modernizing Legacy Systems** — A high-performance, incremental COBOL parser built on the Oak framework. Optimized for mainframe modernization, legacy code analysis, and modern IDE support for the COBOL programming language.

## 🎯 Project Vision

COBOL remains the backbone of global financial and administrative systems, powering mission-critical applications for decades. `oak-cobol` aims to bridge the gap between legacy code and modern development workflows by providing a high-performance, Rust-powered parsing infrastructure. By utilizing Oak's incremental parsing capabilities, it enables developers to build highly responsive tools for massive COBOL codebases, facilitating automated refactoring, documentation generation, and seamless integration with modern DevOps pipelines. Whether you are migrating legacy logic or maintaining existing systems, `oak-cobol` provides the robust foundation needed for deep code analysis.

## ✨ Core Features

- **⚡ Blazing Fast**: Leverages Rust's performance and memory safety to parse massive COBOL source files with sub-millisecond latency.
- **🔄 Incremental by Design**: Built-in support for partial updates—re-parse only what has changed. Essential for providing real-time feedback in large-scale legacy environments.
- **🌳 High-Fidelity AST**: Generates a precise Abstract Syntax Tree capturing the full structure of COBOL:
    - **Divisions & Sections**: Comprehensive mapping of Identification, Environment, Data, and Procedure divisions.
    - **Data Descriptions**: Detailed tracking of picture clauses, level numbers, and complex data hierarchies.
    - **Verbs & Statements**: Accurate representation of arithmetic, conditional, and I/O statements.
    - **Copybooks**: Robust handling of COPY statements for modular code analysis.
- **🛡️ Industrial-Grade Fault Tolerance**: Gracefully recovers from syntax errors, providing clear and actionable diagnostics—critical for analyzing incomplete or slightly malformed legacy code.
- **🧩 Deep Ecosystem Integration**: Seamlessly works with `oak-lsp` for full LSP support and `oak-mcp` for intelligent code discovery and legacy system analysis.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn), which allows for:
1. **Efficient Immutability**: Share nodes across different versions of the tree without copying.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful code formatting and refactoring.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal and analysis.

## 🤝 Contributing

We welcome contributions of all kinds! If you find a bug, have a feature request, or want to contribute code, please check our [issues](https://github.com/ygg-lang/oaks/issues) or submit a pull request.
