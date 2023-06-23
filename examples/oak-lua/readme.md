# 🚀 oak-lua

[![Crates.io](https://img.shields.io/crates/v/oak-lua.svg)](https://crates.io/crates/oak-lua)
[![Documentation](https://docs.rs/oak-lua/badge.svg)](https://docs.rs/oak-lua)

**Lightweight Power for Scripting** — A high-performance, incremental Lua parser built on the Oak framework. Specially optimized for game development, embedded scripting, and modern IDE integration for the Lua programming language.

## 🎯 Project Vision

Lua is the language of choice for extensibility in games and embedded systems. `oak-lua` aims to provide a modern, Rust-based parsing solution that matches Lua's own philosophy of being lightweight and fast. By utilizing Oak's incremental parsing capabilities, it enables developers to build highly responsive tools for large Lua scripts, ensuring that game developers and scripters have the best possible tooling support.

## ✨ Core Features

- **⚡ Blazing Fast**: Leverages Rust's zero-cost abstractions to deliver sub-millisecond parsing performance, mirroring Lua's own speed.
- **🔄 Incremental Parsing**: Built-in support for partial updates—re-parse only what changed, making it ideal for real-time IDE feedback and large-scale script analysis.
- **🌳 High-Fidelity AST**: Generates a clean and easy-to-traverse syntax tree capturing:
    - Global and local variable declarations
    - Function definitions (both global and table-based)
    - Table constructors and complex indexing
    - Control flow structures (If, While, Repeat, For)
    - Full support for Lua 5.x syntax
- **🛡️ Robust Error Recovery**: Engineered to handle incomplete or malformed scripts gracefully, providing precise diagnostics while maintaining parser state.
- **🧩 Ecosystem Integration**: Part of the Oak family—easily integrate with `oak-lsp` for full LSP support or other Oak-based code analysis tools.

## 🏗️ Architecture

`oak-lua` follows the modern Green/Red Tree architecture (inspired by Roslyn):

- **Green Tree**: Immutable, lossless, and syntax-only tree. It captures the full fidelity of the source code, including trivia (comments, whitespace).
- **Red Tree**: A facade over the Green Tree that provides a convenient, type-safe API for tree traversal and analysis, including parent pointers and absolute offsets.

This design enables efficient incremental parsing and powerful refactoring capabilities.


## 🛠️ Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.
