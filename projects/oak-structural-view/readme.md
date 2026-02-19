# 🚀 oak-structural-view

[![Crates.io](https://img.shields.io/crates/v/oak-structural-view.svg)](https://crates.io/crates/oak-structural-view)
[![Documentation](https://docs.rs/oak-structural-view/badge.svg)](https://docs.rs/oak-structural-view)

**Document Structure View for Oak Languages** — Hierarchical representation of document structure for outline views and breadcrumb navigation.

## 🎯 Why oak-structural-view?

Structure views help developers navigate large files by showing a hierarchical outline of classes, functions, and other definitions. Similar to IntelliJ's Structure View and VS Code's Outline panel.

## ✨ Key Features

- **📊 Structure Provider Trait** — `StructureProvider` for language-specific extraction
- **🌳 Hierarchical Items** — `StructureItem` supports nested children
- **🎯 Selection Ranges** — Separate ranges for full element and clickable identifier
- **📝 Detail Support** — Optional detail text for signatures or type information
- **🔄 Serde Support** — Optional serialization for LSP and IDE integration

## 🏗️ Architecture

- `StructureItem` — Name, detail, role, range, selection range, deprecated, children
- `StructureProvider<L>` — Trait for providing document structure

### IDE Integration

Maps to IntelliJ Structure View, VS Code Outline panel and breadcrumbs, and LSP `textDocument/documentSymbol`.

## 🔗 Ecosystem Integration

Used by `oak-lsp` for `textDocument/documentSymbol`, IDE extensions for outline views, and code navigation tools.

## 📖 Documentation

For usage examples and API details, see the [API documentation](https://docs.rs/oak-structural-view).

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
