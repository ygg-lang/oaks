# 🚀 Oak CSV Parser

[![Crates.io](https://img.shields.io/crates/v/oak-csv.svg)](https://crates.io/crates/oak-csv)
[![Documentation](https://docs.rs/oak-csv/badge.svg)](https://docs.rs/oak-csv)

**High-Performance Tabular Data Processing** — A high-performance, incremental CSV parser built on the Oak framework. Optimized for large-scale data analysis, real-time streaming, and robust handling of tabular data formats.

## 🎯 Project Vision

Comma-Separated Values (CSV) are the universal language of data exchange, used across every industry from finance to scientific research. `oak-csv` provides a modern, Rust-powered infrastructure for analyzing and processing tabular data with extreme efficiency. By utilizing Oak's incremental parsing capabilities, it enables the creation of highly responsive data tools that can handle gigabyte-scale files with sub-millisecond updates. Whether you are building data validation engines, automated ETL pipelines, or sophisticated spreadsheet-like editors, `oak-csv` provides the robust, efficient foundation you need for high-fidelity data extraction.

## ✨ Core Features

- **⚡ Blazing Fast**: Leverages Rust's zero-cost abstractions to parse massive CSV datasets with maximum throughput and minimal memory overhead.
- **🔄 Incremental by Nature**: Built-in support for partial updates—re-parse only modified rows or columns. Ideal for real-time data monitoring and large-scale dataset editing.
- **🌳 High-Fidelity AST**: Generates a clean and easy-to-traverse Abstract Syntax Tree capturing:
    - **Rows & Records**: Precise mapping of records, including support for complex quoting and escaping rules.
    - **Headers & Fields**: Automatic identification of header rows and field-level metadata.
    - **Delimiters**: Robust handling of various delimiters (comma, tab, semicolon) and line endings.
- **🛡️ Industrial-Grade Fault Tolerance**: Engineered to handle malformed data gracefully, providing precise diagnostics for unclosed quotes, mismatched column counts, or invalid encoding.
- **🧩 Deep Ecosystem Integration**: Seamlessly works with `oak-lsp` for full LSP support and `oak-mcp` for intelligent data discovery and structured analysis.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn), which allows for:
1. **Efficient Immutability**: Share nodes across different versions of the tree without copying.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful code formatting and refactoring.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal and analysis.

## 🤝 Contributing

We welcome contributions of all kinds! If you find a bug, have a feature request, or want to contribute code, please check our [issues](https://github.com/ygg-lang/oaks/issues) or submit a pull request.
