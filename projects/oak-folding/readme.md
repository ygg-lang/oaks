# 🚀 oak-folding

[![Crates.io](https://img.shields.io/crates/v/oak-folding.svg)](https://crates.io/crates/oak-folding)
[![Documentation](https://docs.rs/oak-folding/badge.svg)](https://docs.rs/oak-folding)

**Code Folding Support for Oak Languages** — Identify collapsible regions in source code for IDE outline views and editor folding.

## 🎯 Why oak-folding?

Code folding helps developers manage large files by collapsing sections of code. `oak-folding` provides a trait-based interface for identifying foldable regions in any Oak-supported language.

## ✨ Key Features

- **📁 Folding Provider Trait** — `FoldingProvider` for language-specific folding logic
- **🎯 Folding Range Types** — Comments, imports, and custom regions
- **📍 Precise Ranges** — Byte-offset based for accurate folding
- **🔄 Serde Support** — Optional serialization for LSP integration

## 🏗️ Architecture

- `FoldingRange` — Represents a foldable region with optional kind
- `FoldingRangeKind` — Comment, Imports, or Region
- `FoldingProvider<L>` — Trait for providing folding ranges

## 🔗 Ecosystem Integration

Used by `oak-lsp` for `textDocument/foldingRange` support, IDE extensions for code outline views, and code formatters.

## 📖 Documentation

For usage examples and API details, see the [API documentation](https://docs.rs/oak-folding).

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
