# Oak Navigation

[![Crates.io](https://img.shields.io/crates/v/oak-navigation.svg)](https://crates.io/crates/oak-navigation)
[![Documentation](https://docs.rs/oak-navigation/badge.svg)](https://docs.rs/oak-navigation)

Core navigation traits and structures for the Oak ecosystem, providing "Go to Definition" and "Find References" capabilities.

## 🎯 Overview

Oak Navigation defines the standard interfaces for navigating through source code. It provides the abstractions needed for cross-referencing symbols across files and projects, supporting the primary navigation features expected in modern IDEs.

## ✨ Features

- **Standardized Traits**: `DefinitionProvider` and `ReferencesProvider` for consistent implementation across languages.
- **LSP Compatibility**: Directly maps to LSP `textDocument/definition` and `textDocument/references`.
- **Location Mapping**: Uses standard `Location` and `Position` types for precise navigation.
- **Language Agnostic**: Works with any language that implements the Oak `Language` trait.

## 🚀 Quick Start

Basic implementation of a definition provider:

```rust
use oak_navigation::{DefinitionProvider, Location, Position};
use oak_core::tree::RedNode;
use my_language::MyLanguage;

struct MyNavProvider;

impl DefinitionProvider<MyLanguage> for MyNavProvider {
    fn definition(&self, root: &RedNode<MyLanguage::ElementType>, position: Position) -> Vec<Location> {
        // Resolve symbol at position and return its definition location
        vec![]
    }
}
```

## 📋 Examples

### Finding References

```rust
use oak_navigation::{ReferencesProvider, Location, Position};
use oak_core::tree::RedNode;

impl ReferencesProvider<MyLanguage> for MyNavProvider {
    fn references(
        &self, 
        root: &RedNode<MyLanguage::ElementType>, 
        position: Position, 
        include_declaration: bool
    ) -> Vec<Location> {
        // Search for all usages of the symbol at the given position
        vec![]
    }
}
```

## 🏗️ Integration

Oak Navigation is a key part of:

- **Oak LSP**: Powers the navigation requests in language servers.
- **Oak Symbols**: Often works in tandem with symbol providers to build a complete index.
- **IDE Plugins**: Enables "Jump to Definition" (F12) and "Find All References" (Shift+F12).

## 📊 Performance

- **Fast Indexing**: Designed to work with global symbol indexes for rapid lookup.
- **Lazy Resolution**: Navigation targets are resolved only when requested.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

---

**Oak Navigation** - Seamless code navigation for every language 🚀
