# Oak LSP

[![Crates.io](https://img.shields.io/crates/v/oak-lsp.svg)](https://crates.io/crates/oak-lsp)
[![Documentation](https://docs.rs/oak-lsp/badge.svg)](https://docs.rs/oak-lsp)

A high-performance Language Server Protocol (LSP) framework built specifically for Oak-based languages.

## 🎯 Overview

Oak LSP provides the glue between Oak's powerful parsing/analysis tools and the Language Server Protocol. It offers a structured way to build language servers, handling common tasks like VFS management, request dispatching, and type conversion between Oak and LSP standards.

## ✨ Features

- **Service-Oriented Architecture**: Clean `LanguageService` trait for implementing language features.
- **VFS Integration**: Built-in support for Virtual File Systems (Memory and Disk).
- **Asynchronous**: Built on `tokio` for high-concurrency request handling.
- **Comprehensive Support**: Ready-to-use handlers for Hover, Folding, Symbols, Diagnostics, and more.
- **LSP Type Safety**: Strong typing for LSP structures, minimizing manual JSON handling.

## 🚀 Quick Start

Defining a simple language service:

```rust
use oak_lsp::service::LanguageService;
use oak_vfs::MemoryVfs;
use my_language::MyLanguage;

pub struct MyService {
    vfs: MemoryVfs,
}

impl LanguageService for MyService {
    type Lang = MyLanguage;
    type Vfs = MemoryVfs;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }
    
    // Override methods like hover, folding_ranges, etc.
}
```

## 📋 Examples

### Running an LSP Server

```rust
use oak_lsp::LanguageService;
use std::sync::Arc;

async fn start_server<S: LanguageService>(service: Arc<S>) {
    // Oak LSP provides handlers that can be plugged into web frameworks or stdio
    // e.g., using axum
}
```

## 🏗️ Supported Features

Oak LSP provides a unified interface for:

- **Navigation**: Definition, References, Implementation.
- **Editing**: Completion, Formatting, Rename.
- **Analysis**: Diagnostics, Semantic Tokens, Document Symbols.
- **UI**: Hover, Folding, Breadcrumbs.

## 📊 Performance

- **Non-blocking**: All LSP requests are handled asynchronously.
- **Incremental**: Leverages Oak's incremental parsing for fast re-analysis on every keystroke.
- **Resource Efficient**: Shared state management via `Arc` and efficient VFS caching.

## 🔗 Integration

Oak LSP is the foundation for:

- **VS Code Extension**: The official Oak extension uses this framework.
- **CLI Tools**: For running static analysis and linting in CI/CD.
- **Web-based Editors**: Integration with Monaco or CodeMirror via LSP.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

---

**Oak LSP** - Powering the next generation of language tools 🚀
