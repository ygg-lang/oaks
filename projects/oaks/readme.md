# Oaks Ecosystem

[![Crates.io](https://img.shields.io/crates/v/oaks.svg)](https://crates.io/crates/oaks)
[![Documentation](https://docs.rs/oaks/badge.svg)](https://docs.rs/oaks)

A unified, language-agnostic framework for building modern IDE features, compilers, and language tools in Rust.

## 🎯 Overview

`oaks` is a high-level facade and orchestration layer for the entire Oak ecosystem. It provides a decoupled, trait-based architecture for building common editor features like hovers, folding, navigation, and semantic highlighting. Inspired by IntelliJ's PSI and the Language Server Protocol (LSP), Oaks makes it easy to add advanced IDE support to any programming language.

## 🏗️ Ecosystem Components

The Oak ecosystem consists of several specialized libraries:

- **[oak-core](./projects/oak-core)**: Foundational primitives, green/red trees, and incremental parsing.
- **[oak-lsp](./projects/oak-lsp)**: Lightweight LSP-compatible types and `LanguageService` traits.
- **[oak-vfs](./projects/oak-vfs)**: Virtual File System abstraction for memory and disk-based storage.
- **[oak-highlight](./projects/oak-highlight)**: Rule-based syntax highlighting engine.
- **[oak-pretty-print](./projects/oak-pretty-print)**: Advanced code formatting and document construction.
- **[oak-hover](./projects/oak-hover)**: Documentation hovers and tooltips.
- **[oak-folding](./projects/oak-folding)**: Code folding range provider.
- **[oak-navigation](./projects/oak-navigation)**: Go-to-definition and find-references support.
- **[oak-structure](./projects/oak-structure)**: Hierarchical document outline and breadcrumbs.
- **[oak-symbols](./projects/oak-symbols)**: Workspace and document symbol indexing.
- **[oak-semantic-tokens](./projects/oak-semantic-tokens)**: Precise semantic syntax highlighting.
- **[oak-semantic-search](./projects/oak-semantic-search)**: Vector-based semantic code search.
- **[oak-visualize](./projects/oak-visualize)**: AST and graph visualization algorithms.
- **[oak-mcp](./projects/oak-mcp)**: Model Context Protocol (MCP) integration for AI agents.

## 🚀 Quick Start

To build a language service using Oaks:

```rust
use oaks::{MemoryVfs, LanguageServiceBuilder};
use oak_rust::RustService; // Example language implementation

#[tokio::main]
async fn main() {
    // 1. Initialize Virtual File System
    let vfs = MemoryVfs::new();
    vfs.write_file("file:///main.rs", "fn main() { }".to_string());

    // 2. Build the Language Service
    let service = LanguageServiceBuilder::new(Arc::new(vfs))
        .with_hover(MyHoverProvider::new())
        .with_folding(MyFoldingProvider::new())
        .build();

    // 3. Use the service
    let hover = service.hover("file:///main.rs", Position::new(0, 5)).await;
}
```

## ✨ Key Concepts

### Language-Agnostic Design
Oaks is designed to be completely independent of any specific programming language. By implementing the `Language` trait from `oak-core`, you can plug any language into the Oaks ecosystem.

### Virtual File System (VFS)
The `Vfs` trait provides a unified way to access source code, whether it's stored on disk, in memory, or provided by a remote client. This allows Oaks features to work consistently across different environments.

### Decoupled Features
Each IDE feature (hover, folding, etc.) is defined as a separate trait. This allows you to implement features incrementally and mix-and-match them as needed.

## 🔧 Integration with LSP

Oaks provides first-class support for the Language Server Protocol. You can easily convert an Oaks `LanguageService` into a full-featured LSP server using `oak-lsp`.

```rust
use oak_lsp::LspServer;

let lsp_server = LspServer::new(my_oaks_service);
lsp_server.run().await?;
```

## 📊 Performance

- **Incremental Processing**: Built-in support for incremental updates ensures low latency even for large projects.
- **Parallel Analysis**: Designed for multi-threaded execution to leverage modern hardware.
- **Memory Efficient**: Shared green trees and compact data structures minimize memory overhead.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

---

**Oaks** - Empowering the next generation of language tools 🚀
