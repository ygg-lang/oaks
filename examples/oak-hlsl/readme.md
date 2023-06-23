# 🚀 Oak HLSL Parser

[![Crates.io](https://img.shields.io/crates/v/oak-hlsl.svg)](https://crates.io/crates/oak-hlsl)
[![Documentation](https://docs.rs/oak-hlsl/badge.svg)](https://docs.rs/oak-hlsl)

**Graphics Power for the Modern Web & Beyond** — A high-performance, incremental HLSL (High-Level Shading Language) parser built on the Oak framework. Specially optimized for GPU shader development, cross-platform graphics toolchains, and modern IDE integration.

## 🎯 Project Vision

HLSL is the backbone of modern graphics programming. `oak-hlsl` aims to provide a robust, Rust-based parsing infrastructure that meets the demands of high-performance shader compilation and real-time analysis. By utilizing Oak's incremental parsing capabilities, it enables developers to build highly responsive shader editors, linters, and translation tools that can handle complex shader codebases with sub-millisecond feedback. Whether you are building a custom shader compiler, a visual shader graph tool, or an advanced IDE extension for DirectX/Vulkan development, `oak-hlsl` provides the efficiency and precision required.

## ✨ Core Features

- **⚡ Blazing Fast**: Leverages Rust's zero-cost abstractions to deliver sub-millisecond parsing performance, essential for real-time shader development and iteration.
- **🔄 Incremental Parsing**: Built-in support for partial updates—re-parse only modified shader stages or functions. Ideal for real-time preview and IDE feedback loops.
- **🌳 High-Fidelity AST**: Generates a clean and easy-to-traverse Abstract Syntax Tree capturing:
    - **Shader Stages**: Explicit support for Vertex, Pixel, Compute, and other shader stages.
    - **Buffer & Resource Declarations**: Precise mapping of Constant Buffers, Structured Buffers, and Texture resources.
    - **Semantics & Annotations**: Detailed tracking of input/output semantics and shader-specific attributes.
    - **Complex Expressions**: Robust support for vector/matrix operations and HLSL intrinsic functions.
- **🛡️ Industrial-Grade Fault Tolerance**: Engineered to handle incomplete or malformed shader code gracefully, providing precise diagnostics—crucial for maintaining a smooth developer experience during active shader authoring.
- **🧩 Deep Ecosystem Integration**: Seamlessly works with `oak-lsp` for full LSP support and other Oak-based graphics analysis tools.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn), which allows for:
1. **Efficient Immutability**: Share nodes across different versions of the tree without copying.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful code formatting and refactoring.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal and analysis.


## 🤝 Contributing

We welcome contributions of all kinds! If you find a bug, have a feature request, or want to contribute code, please check our [issues](https://github.com/ygg-lang/oaks/issues) or submit a pull request.
