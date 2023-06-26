# 🚀 Oak Nim Parser

[![Crates.io](https://img.shields.io/crates/v/oak-nim.svg)](https://crates.io/crates/oak-nim)
[![Documentation](https://docs.rs/oak-nim/badge.svg)](https://docs.rs/oak-nim)

**Efficiency Meets Expressiveness** — A high-performance, incremental Nim parser built on the Oak framework.

## 🎯 Why oak-nim?

Nim combines Python-like expressiveness with C-like performance, featuring indentation-based syntax, powerful metaprogramming, and compile-time execution. `oak-nim` handles Nim's unique syntax challenges while delivering sub-millisecond performance.

## ✨ Key Features

- **⚡ Blazing Fast** — Sub-millisecond parsing for real-time IDE feedback
- **🔄 Incremental Parsing** — Re-parse only what changed
- **🌳 High-Fidelity AST** — Captures indentation blocks, metaprogramming, type system, pragmas
- **🛡️ Robust Error Recovery** — Graceful handling with precise diagnostics
- **🧩 Deep Ecosystem Integration** — Works with `oak-lsp` and `oak-mcp`

## 🏗️ Architecture

Follows the Green/Red Tree architecture with efficient immutability, lossless syntax trees, and type-safe traversal.

## 📖 Documentation

For usage examples and API details, see the [API documentation](https://docs.rs/oak-nim).

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
