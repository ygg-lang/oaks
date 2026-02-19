# 🚀 Oak CMD Parser

[![Crates.io](https://img.shields.io/crates/v/oak-cmd.svg)](https://crates.io/crates/oak-cmd)
[![Documentation](https://docs.rs/oak-cmd/badge.svg)](https://docs.rs/oak-cmd)

**Windows Batch Script Analysis** — A high-performance, incremental Windows batch script parser built on the Oak framework.

## 🎯 Why oak-cmd?

Windows batch scripts (.bat, .cmd) remain essential for Windows automation and system administration. `oak-cmd` provides modern parsing infrastructure for linting tools, IDE support, and analysis utilities.

## ✨ Key Features

- **⚡ Blazing Fast** — Sub-millisecond parsing for IDE integration
- **🔄 Incremental Parsing** — Re-parse only what changed
- **🌳 High-Fidelity AST** — Captures commands, control flow, variables, redirection
- **🛡️ Robust Error Recovery** — Precise diagnostics for common issues
- **🧩 Deep Ecosystem Integration** — Works with `oak-lsp` and `oak-mcp`

## 🏗️ Architecture

Follows the Green/Red Tree architecture with efficient immutability, lossless syntax trees, and type-safe traversal.

## 📖 Documentation

For usage examples and API details, see the [API documentation](https://docs.rs/oak-cmd).

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
