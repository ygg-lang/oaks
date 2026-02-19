# 🚀 oak-semantic-search

[![Crates.io](https://img.shields.io/crates/v/oak-semantic-search.svg)](https://crates.io/crates/oak-semantic-search)
[![Documentation](https://docs.rs/oak-semantic-search/badge.svg)](https://docs.rs/oak-semantic-search)

**Semantic Search for Oak Languages** — Search code by meaning, not just text.

## 🎯 Why oak-semantic-search?

Traditional text search finds literal matches, but semantic search understands code meaning. Search for concepts like "all functions that return Result" or "all implementations of this trait."

## ✨ Key Features

- **🔍 Semantic Queries** — Search by code structure and meaning
- **📊 Pattern Matching** — Match code patterns across the codebase
- **🌐 Workspace-Wide** — Search across multiple files and projects
- **⚡ Fast Indexing** — Efficient indexing for quick searches
- **🧩 Language Agnostic** — Works with any Oak language parser

## 🏗️ Architecture

- `SemanticQuery` — Query types for semantic searches
- `SearchPattern` — Pattern-based code matching
- `SearchIndex` — Workspace-wide symbol indexing

## 🔗 Ecosystem Integration

Integrates with `oak-vfs` for file access, `oak-symbols` for symbol extraction, and IDE extensions for advanced search.

## 📖 Documentation

For usage examples and API details, see the [API documentation](https://docs.rs/oak-semantic-search).

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
