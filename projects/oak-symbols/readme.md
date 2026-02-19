# 🚀 oak-symbols

[![Crates.io](https://img.shields.io/crates/v/oak-symbols.svg)](https://crates.io/crates/oak-symbols)
[![Documentation](https://docs.rs/oak-symbols/badge.svg)](https://docs.rs/oak-symbols)

**Symbol Management for Oak Languages** — Extract and manage symbol information for document outlines, workspace search, and navigation.

## 🎯 Why oak-symbols?

Symbols are the building blocks of code — functions, classes, variables, and more. `oak-symbols` provides a unified interface for extracting symbol information from syntax trees.

## ✨ Key Features

- **📊 Symbol Provider Trait** — `SymbolProvider` for symbol extraction
- **🔍 Symbol Information** — Rich type with name, role, location, and container
- **🌐 Universal Provider** — Built-in `UniversalSymbolProvider` for any Oak language
- **📂 Document & Workspace** — Document-level and workspace-wide symbol queries
- **🔄 Serde Support** — Optional serialization for LSP integration

## 🏗️ Architecture

- `SymbolInformation` — Symbol name, role, URI, range, and container
- `SymbolProvider<L>` — Trait for document and workspace symbol queries
- `UniversalSymbolProvider` — Ready-to-use provider for any language

## 🔗 Ecosystem Integration

Used by `oak-lsp` for `textDocument/documentSymbol` and `workspace/symbol`, `oak-structural-view` for outline views, and `oak-mcp` for AI-assisted symbol discovery.

## 📖 Documentation

For usage examples and API details, see the [API documentation](https://docs.rs/oak-symbols).

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
