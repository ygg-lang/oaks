# 🚀 oak-resolver

[![Crates.io](https://img.shields.io/crates/v/oak-resolver.svg)](https://crates.io/crates/oak-resolver)
[![Documentation](https://docs.rs/oak-resolver/badge.svg)](https://docs.rs/oak-resolver)

**Symbol Resolution for Oak Languages** — Cross-file symbol resolution and import handling for language analysis.

## 🎯 Why oak-resolver?

Modern codebases span multiple files and modules. `oak-resolver` provides the infrastructure for resolving symbol references across file boundaries, handling imports, and building symbol indexes.

## ✨ Key Features

- **🔍 Cross-File Resolution** — Resolve symbols across multiple source files
- **📦 Import Handling** — Process import statements and module dependencies
- **📊 Symbol Indexing** — Build and query symbol indexes for fast lookups
- **🌐 Workspace Support** — Scale from single files to large projects
- **🔄 Incremental Updates** — Efficiently update resolution on file changes

## 🏗️ Architecture

Resolution process: Parse → Collect → Index → Resolve

## 🔗 Ecosystem Integration

Integrates with `oak-vfs` for file access, `oak-navigation` for definition/reference providers, and `oak-lsp` for workspace symbol support.

## 📖 Documentation

For usage examples and API details, see the [API documentation](https://docs.rs/oak-resolver).

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
