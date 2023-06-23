# 🚀 Oak JSON Parser

[![Crates.io](https://img.shields.io/crates/v/oak-json.svg)](https://crates.io/crates/oak-json)
[![Documentation](https://docs.rs/oak-json/badge.svg)](https://docs.rs/oak-json)

**The Foundation of Data Exchange** — A high-performance, incremental JSON parser built on the Oak framework. Optimized for large-scale data processing, configuration management, and real-time validation.

## 🎯 Project Vision

JSON is the universal language of data exchange in modern computing. `oak-json` provides a high-performance, Rust-powered infrastructure for parsing and analyzing JSON data. By leveraging Oak's incremental parsing architecture, it enables developers to build tools that can handle massive JSON files and streams with sub-millisecond latency, making it ideal for everything from configuration editors to large-scale data analysis pipelines.

## ✨ Core Features

- **⚡ Blazing Fast**: Fully utilizes Rust's performance advantages to deliver sub-millisecond parsing response times, even for deeply nested JSON structures.
- **🔄 Incremental Parsing**: Built-in support for partial updates—re-parse only the parts of the JSON that changed, significantly improving performance for large configuration files.
- **🌳 High-Fidelity AST**: Generates a clear and easy-to-traverse Abstract Syntax Tree capturing:
    - Objects, Arrays, and nested structures
    - Precise tracking of keys and values
    - Full support for all JSON data types (Strings, Numbers, Booleans, Null)
- **🛡️ Industrial-Grade Error Recovery**: Engineered to handle malformed or incomplete JSON gracefully, providing precise diagnostics to help developers and users fix data issues quickly.
- **🧩 Ecosystem Integration**: Seamlessly works with other Oak tools for validation, formatting, and semantic analysis.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn), which allows for:
1. **Efficient Immutability**: Share nodes across different versions of the tree without copying.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments, if applicable in extended JSON formats), enabling faithful code formatting and refactoring.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal and analysis.


## 🤝 Contributing

We welcome contributions of all kinds! If you find a bug, have a feature request, or want to contribute code, please check our [issues](https://github.com/ygg-lang/oaks/issues) or submit a pull request.
