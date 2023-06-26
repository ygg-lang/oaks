# 🚀 oak-pretty-print

[![Crates.io](https://img.shields.io/crates/v/oak-pretty-print.svg)](https://crates.io/crates/oak-pretty-print)
[![Documentation](https://docs.rs/oak-pretty-print/badge.svg)](https://docs.rs/oak-pretty-print)

**Code Formatting and Pretty Printing for Oak** — Lossless code formatting infrastructure for Oak language parsers.

## 🎯 Why oak-pretty-print?

Code formatting is essential for readability and consistency. `oak-pretty-print` provides the infrastructure for implementing formatters that preserve all source details while applying consistent formatting rules.

## ✨ Key Features

- **📝 Lossless Formatting** — Preserves all source details including comments and trivia
- **🌳 Tree-Based** — Works directly with Oak's syntax trees for accuracy
- **🔧 Configurable** — Support for different formatting styles and options
- **⚡ Incremental** — Efficient re-formatting of changed sections only
- **🧩 Language Agnostic** — Works with any Oak language parser

## 🏗️ Architecture

Operates on the Green/Red tree structure via trivia manipulation, rule-based formatting, and lossless round-trip processing.

## 🔗 Ecosystem Integration

Integrates with `oak-lsp` for `textDocument/formatting`, language-specific formatters, and code style enforcement tools.

## 📖 Documentation

For usage examples and API details, see the [API documentation](https://docs.rs/oak-pretty-print).

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
