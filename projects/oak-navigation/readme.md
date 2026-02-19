# 🚀 oak-navigation

[![Crates.io](https://img.shields.io/crates/v/oak-navigation.svg)](https://crates.io/crates/oak-navigation)
[![Documentation](https://docs.rs/oak-navigation/badge.svg)](https://docs.rs/oak-navigation)

**Code Navigation Support for Oak Languages** — Traits and utilities for implementing "Go to Definition" and "Find All References" features.

## 🎯 Why oak-navigation?

Code navigation is fundamental to developer productivity. `oak-navigation` provides the building blocks for implementing navigation features in any Oak-based language tool.

## ✨ Key Features

- **🎯 Definition Provider** — `DefinitionProvider` trait for "Go to Definition"
- **🔍 References Provider** — `ReferencesProvider` trait for "Find All References"
- **📍 Location Type** — Unified `Location` for source positions across files
- **🔎 Simple Reference Finder** — Built-in name-based reference search
- **🔄 Serde Support** — Optional serialization for LSP integration

## 🏗️ Architecture

- `Location` — Represents a position in source code (URI + byte range)
- `DefinitionProvider<L>` — Trait for finding symbol definitions
- `ReferencesProvider<L>` — Trait for finding symbol references
- `SimpleReferenceFinder` — Helper for basic name-based search

## 🔗 Ecosystem Integration

Used by `oak-lsp` for LSP navigation features, `oak-mcp` for AI-assisted navigation, and custom IDE extensions.

## 📖 Documentation

For usage examples and API details, see the [API documentation](https://docs.rs/oak-navigation).

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
