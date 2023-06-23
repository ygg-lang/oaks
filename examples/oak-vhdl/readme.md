# 🚀 oak-vhdl

[![Crates.io](https://img.shields.io/crates/v/oak-vhdl.svg)](https://crates.io/crates/oak-vhdl)
[![Documentation](https://docs.rs/oak-vhdl/badge.svg)](https://docs.rs/oak-vhdl)

**High-Fidelity Hardware Description Parsing** — A high-performance, incremental VHDL parser built on the Oak framework. Optimized for industrial-grade FPGA/ASIC design tools, formal verification, and real-time hardware development environments.

## 🎯 Project Vision

VHDL is a foundational language for hardware description, but its verbose syntax and complex scoping rules make it challenging for traditional parsers. `oak-vhdl` aims to provide a robust, modern, Rust-powered infrastructure for parsing VHDL that is both accurate and incredibly fast. By utilizing Oak's incremental parsing architecture, we enable the creation of highly responsive IDEs, linting tools, and synthesis front-ends that can handle massive hardware designs in real-time. Our goal is to empower hardware engineers with the same level of sophisticated tooling available in the software world.

## ✨ Core Features

- **⚡ Blazing Fast**: Leverages Rust's performance and memory safety to provide sub-millisecond parsing, essential for high-frequency developer tools and real-time analysis of large VHDL projects.
- **🔄 Incremental by Nature**: Built-in support for partial updates—re-parse only modified sections of large VHDL files. Ideal for large-scale FPGA designs and real-time error checking.
- **🌳 High-Fidelity AST**: Generates a comprehensive and precise Abstract Syntax Tree capturing the full depth of VHDL:
    - **Entity & Architecture**: Detailed mapping of hardware interfaces and their internal logic implementations.
    - **Processes & Concurrent Statements**: Precise tracking of sensitivity lists, signal assignments, and component instantiations.
    - **Packages & Libraries**: Robust support for VHDL's modularity and reusable component ecosystem.
    - **Types & Subtypes**: Detailed tracking of complex data types, records, and arrays.
- **🛡️ Industrial-Grade Fault Tolerance**: Engineered to recover from syntax errors gracefully, providing precise diagnostics—crucial for maintaining a smooth developer experience when writing complex hardware logic.
- **🧩 Deep Ecosystem Integration**: Seamlessly works with `oak-lsp` for full LSP support and `oak-mcp` for intelligent hardware design discovery and analysis.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn), which allows for:
1. **Efficient Immutability**: Share nodes across different versions of the tree without copying.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful code formatting and refactoring of VHDL source files.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal and analysis.


## 🤝 Contributing

We welcome contributions of all kinds! If you find a bug, have a feature request, or want to contribute code, please check our [issues](https://github.com/ygg-lang/oaks/issues) or submit a pull request.
