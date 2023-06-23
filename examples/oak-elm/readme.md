# 🚀 Oak Elixir Parser

[![Crates.io](https://img.shields.io/crates/v/oak-elixir.svg)](https://crates.io/crates/oak-elixir)
[![Documentation](https://docs.rs/oak-elixir/badge.svg)](https://docs.rs/oak-elixir)

**Scalable and Fault-Tolerant Parsing for the Beam** — A high-performance, incremental Elixir parser built on the Oak framework. Optimized for distributed systems, high-concurrency environments, and modern developer tooling.

## 🎯 Project Vision

Elixir is designed for building scalable and maintainable applications, leveraging the Erlang VM (BEAM). `oak-elixir` aims to provide a robust, modern, Rust-powered infrastructure for parsing Elixir that is both accurate and incredibly fast. By utilizing Oak's incremental parsing architecture, we enable the creation of highly responsive IDEs, code analysis tools, and automated refactoring utilities that can handle complex Elixir projects in real-time. Whether you are building custom linters, automated code generators, or sophisticated IDE extensions, `oak-elixir` provides the high-fidelity AST and efficiency needed to keep pace with Elixir's productivity-focused ecosystem.

## ✨ Core Features

- **⚡ Blazing Fast**: Leverages Rust's performance and memory safety to provide sub-millisecond parsing, essential for high-frequency developer tools and real-time analysis in large Elixir projects.
- **🔄 Incremental by Nature**: Built-in support for partial updates—re-parse only what has changed. Ideal for large-scale Elixir projects where maintainability and tool responsiveness are critical.
- **🌳 High-Fidelity AST**: Generates a comprehensive and precise Abstract Syntax Tree capturing the full depth of Elixir:
    - **Metaprogramming**: Deep support for macros, quotes, and unquotes.
    - **Modules & Functions**: Precise mapping of module definitions, function clauses, and guards.
    - **Pattern Matching**: Robust handling of complex pattern matching constructs.
    - **Protocols & Behaviors**: Detailed tracking of protocol implementations and behavior definitions.
    - **Comments & Whitespace**: Retains all trivia, enabling faithful round-trip processing and refactoring.
- **🛡️ Industrial-Grade Fault Tolerance**: Engineered to recover from syntax errors gracefully, providing precise diagnostics—crucial for maintaining a smooth developer experience during active coding.
- **🧩 Deep Ecosystem Integration**: Seamlessly works with `oak-lsp` for full LSP support and `oak-mcp` for intelligent code discovery and analysis.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn), which allows for:
1. **Efficient Immutability**: Share nodes across different versions of the tree without copying.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful code formatting and refactoring of Elixir scripts.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal and analysis.


## 🛠️ Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.
