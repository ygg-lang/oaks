# 🚀 oak-vfs

[![Crates.io](https://img.shields.io/crates/v/oak-vfs.svg)](https://crates.io/crates/oak-vfs)
[![Documentation](https://docs.rs/oak-vfs/badge.svg)](https://docs.rs/oak-vfs)

**Virtual File System for Oak Language Tools** — A unified abstraction for file system operations supporting both in-memory and disk-based storage.

## 🎯 Why oak-vfs?

Language tools need to access files from various sources — local disk, in-memory buffers, remote storage, or virtual projects. `oak-vfs` provides a unified abstraction layer that decouples language analysis from storage implementation.

## ✨ Key Features

- **📁 Unified Abstraction** — `Vfs` trait for any file source
- **💾 Memory VFS** — In-memory projects, testing, sandboxed environments
- **💿 Disk VFS** — Real file system with watching support (feature-gated)
- **📊 Line Mapping** — `LineMap` for byte offset to line/column translation
- **🔄 Serde Support** — Optional serialization for metadata and line maps

## 🏗️ Architecture

- `Vfs` trait — Core abstraction for file system access
- `MemoryVfs` — In-memory file system for testing
- `DiskVfs` — Real file system with watching (feature: `disk`)
- `LineMap` — Efficient line/column mapping

## 🔗 Ecosystem Integration

Used by `oak-lsp` for workspace file management, `oak-mcp` for project analysis, and language servers for cross-file navigation.

## 📖 Documentation

For usage examples and API details, see the [API documentation](https://docs.rs/oak-vfs).

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
