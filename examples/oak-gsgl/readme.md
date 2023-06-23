# 🚀 Oak GSGL Parser

[![Crates.io](https://img.shields.io/crates/v/oak-gsgl.svg)](https://crates.io/crates/oak-gsgl)
[![Documentation](https://docs.rs/oak-gsgl/badge.svg)](https://docs.rs/oak-gsgl)

**Advanced Graphics Shading with Precision** — A high-performance, incremental GSGL parser built on the Oak framework. Optimized for next-generation graphics pipelines, shader optimization, and modern developer tooling.

## 🎯 Project Vision

GSGL is a powerful language for graphics shading and computation. `oak-gsgl` aims to provide a robust, modern, Rust-powered infrastructure for parsing GSGL that is both accurate and incredibly fast. By utilizing Oak's incremental parsing architecture, we enable the creation of highly responsive shader editors, optimization tools, and automated translation utilities that can handle complex graphics codebases in real-time. Whether you are building custom shader compilers, performance analyzers, or sophisticated IDE extensions, `oak-gsgl` provides the high-fidelity AST and efficiency needed to push the boundaries of graphics programming.

## ✨ Core Features

- **⚡ Blazing Fast**: Leverages Rust's performance and memory safety to provide sub-millisecond parsing, essential for high-frequency developer tools and real-time shader analysis.
- **🔄 Incremental by Nature**: Built-in support for partial updates—re-parse only modified shader stages or functions. Ideal for real-time preview and rapid iteration in graphics development.
- **🌳 High-Fidelity AST**: Generates a comprehensive and precise Abstract Syntax Tree capturing the full depth of GSGL:
    - **Shader Stages**: Native support for vertex, fragment, compute, and geometry shader constructs.
    - **Buffer & Resource Mapping**: Precise tracking of uniform buffers, storage buffers, and texture resources.
    - **Vector & Matrix Operations**: Robust handling of graphics-specific math and intrinsic functions.
    - **Comments & Whitespace**: Retains all trivia, enabling faithful round-trip processing and refactoring.
- **🛡️ Industrial-Grade Fault Tolerance**: Engineered to recover from syntax errors gracefully, providing precise diagnostics—crucial for maintaining a smooth developer experience during active shader authoring.
- **🧩 Deep Ecosystem Integration**: Seamlessly works with `oak-lsp` for full LSP support and other Oak-based graphics tools.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn), which allows for:
1. **Efficient Immutability**: Share nodes across different versions of the tree without copying.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful code formatting and refactoring of GSGL source files.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal and analysis.


## 🤝 Contributing

We welcome contributions of all kinds! If you find a bug, have a feature request, or want to contribute code, please check our [issues](https://github.com/ygg-lang/oaks/issues) or submit a pull request.
