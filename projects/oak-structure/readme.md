# Oak Structure

[![Crates.io](https://img.shields.io/crates/v/oak-structure.svg)](https://crates.io/crates/oak-structure)
[![Documentation](https://docs.rs/oak-structure/badge.svg)](https://docs.rs/oak-structure)

Hierarchical document structure and outline provider for the Oak ecosystem.

## 🎯 Overview

Oak Structure provides the tools to build a hierarchical representation of a document's logical components. It is used to power "Outline" views, "Breadcrumbs", and "Document Symbols" in modern IDEs, allowing users to quickly see and navigate the structure of their code.

## ✨ Features

- **Hierarchical Representation**: Supports nested items (e.g., methods inside classes).
- **Universal Roles**: Uses `UniversalElementRole` for cross-language compatibility.
- **Selection Ranges**: Distinguishes between the full range of an item and its clickable name/identifier range.
- **LSP Ready**: Fully compatible with `textDocument/documentSymbol`.
- **Benchmarked**: Designed to match the quality and performance of IntelliJ's Structure View.

## 🚀 Quick Start

Creating a `StructureItem`:

```rust
use oak_structure::StructureItem;
use oak_core::language::UniversalElementRole;

let item = StructureItem {
    name: "main".to_string(),
    detail: Some("fn main()".to_string()),
    role: UniversalElementRole::Definition,
    range: 0..50,
    selection_range: 3..7,
    deprecated: false,
    children: vec![],
};
```

## 📋 Examples

### Implementing a Structure Provider

```rust
use oak_structure::{StructureProvider, StructureItem};
use oak_core::tree::RedNode;
use my_language::MyLanguage;

struct MyStructureProvider;

impl StructureProvider<MyLanguage> for MyStructureProvider {
    fn structure(&self, root: &RedNode<MyLanguage::ElementType>) -> Vec<StructureItem> {
        // Recursively build the structure tree from the AST
        vec![]
    }
}
```

## 🏗️ Integration

Oak Structure is a core component for:

- **Oak LSP**: Provides the data for document symbols and breadcrumbs.
- **Oak Navigation**: Often used as the entry point for navigating complex files.
- **IDE Outline Views**: Powers the sidebar navigation in various editors.

## 📊 Performance

- **Efficient Tree Mapping**: Optimized for converting deep ASTs into human-readable structure trees.
- **Memory Efficient**: Uses a flattened representation where possible to minimize allocations.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

---

**Oak Structure** - Visualizing the logical architecture of your code 🚀
