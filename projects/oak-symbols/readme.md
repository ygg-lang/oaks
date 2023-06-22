# Oak Symbols

[![Crates.io](https://img.shields.io/crates/v/oak-symbols.svg)](https://crates.io/crates/oak-symbols)
[![Documentation](https://docs.rs/oak-symbols/badge.svg)](https://docs.rs/oak-symbols)

Universal symbol indexing and search engine for the Oak ecosystem.

## 🎯 Overview

Oak Symbols provides a unified way to identify, collect, and search for symbols (functions, classes, variables, etc.) across source files. It features a `UniversalSymbolProvider` that can automatically extract symbols from any language that follows Oak's semantic role conventions.

## ✨ Features

- **Universal Provider**: Automatically extracts symbols from any Oak-compatible language without custom code.
- **Hierarchical Context**: Tracks container relationships (e.g., which class a method belongs to).
- **LSP Integration**: Designed to power `workspace/symbol` and `textDocument/documentSymbol`.
- **Role-Based Classification**: Uses `UniversalElementRole` for consistent symbol categorization across different languages.

## 🚀 Quick Start

Using the `UniversalSymbolProvider`:

```rust
use oak_symbols::{UniversalSymbolProvider, SymbolProvider};
use oak_core::tree::RedNode;
use my_language::MyLanguage;

let provider = UniversalSymbolProvider::new();
// let symbols = provider.document_symbols::<MyLanguage>(&root);
```

## 📋 Examples

### Symbol Information Structure

```rust
use oak_symbols::SymbolInformation;
use oak_core::language::UniversalElementRole;
use oak_lsp::Location;

let info = SymbolInformation {
    name: "calculate_total".to_string(),
    role: UniversalElementRole::Definition,
    location: Location { uri: "...".to_string(), range: 10..30 },
    container_name: Some("OrderProcessor".to_string()),
};
```

## 🔧 Advanced Features

### Semantic Role Extraction

The `UniversalSymbolProvider` works by inspecting the `UniversalElementRole` of AST nodes. Any node marked as `Definition` is automatically indexed as a symbol, and its children are searched for identifiers to use as the symbol name.

## 🏗️ Integration

- **Oak LSP**: Powers global workspace symbol search.
- **Oak Navigation**: Used to resolve symbol names to their definition locations.
- **Documentation Generators**: Automatically builds indexes of all functions/types in a project.

## 📊 Performance

- **Fast Collection**: Optimized tree traversal for symbol extraction.
- **Search-Ready**: Symbol structures are designed for efficient indexing in search engines.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

---

**Oak Symbols** - Finding the needle in the code haystack 🚀
