# Oak VFS

[![Crates.io](https://img.shields.io/crates/v/oak-vfs.svg)](https://crates.io/crates/oak-vfs)
[![Documentation](https://docs.rs/oak-vfs/badge.svg)](https://docs.rs/oak-vfs)

A high-performance Virtual File System (VFS) abstraction for the Oak ecosystem, supporting both disk and memory-based storage.

## 🎯 Overview

Oak VFS provides a unified interface for file operations, allowing the Oak framework to work seamlessly with physical files on disk or virtual files in memory. This is critical for IDEs and Language Servers where files may be modified in memory before being saved to disk.

## ✨ Features

- **Unified Interface**: Single `Vfs` trait for all file operations.
- **Multiple Backends**:
    - `DiskVfs`: Real-time access to the physical filesystem.
    - `MemoryVfs`: Ultra-fast, in-memory file storage for unsaved buffers or tests.
- **Position Mapping**: Built-in support for converting between byte offsets and line/column positions.
- **Metadata Support**: Track file types, sizes, and modification timestamps.
- **Async Ready**: Designed to work in asynchronous environments like `tokio`.

## 🚀 Quick Start

Using the `MemoryVfs`:

```rust
use oak_vfs::{MemoryVfs, Vfs, WritableVfs};

let vfs = MemoryVfs::new();
vfs.write_file("file:///hello.rs", "fn main() {}".to_string());

if vfs.exists("file:///hello.rs") {
    let source = vfs.get_source("file:///hello.rs").unwrap();
    println!("Content: {}", source.text());
}
```

## 📋 Examples

### Working with Disk VFS

```rust
use oak_vfs::{DiskVfs, Vfs};

let vfs = DiskVfs::new();
if let Some(metadata) = vfs.metadata("file:///C:/projects/main.rs") {
    println!("File size: {} bytes", metadata.len);
}
```

### Position Mapping

```rust
use oak_vfs::{Vfs, MemoryVfs};
use oak_core::source::Position;

let vfs = MemoryVfs::new();
vfs.write_file("test.txt", "Line 1\nLine 2".to_string());

let pos = vfs.offset_to_position("test.txt", 8).unwrap();
println!("Offset 8 is at Line: {}, Col: {}", pos.line, pos.character);
```

## 🏗️ Integration

Oak VFS is the backbone of:

- **Oak LSP**: Manages open file buffers and provides source content to parsers.
- **Oak Core**: Uses VFS-provided sources for building red-green trees.
- **Test Suites**: Uses `MemoryVfs` to simulate file structures without disk I/O.

## 📊 Performance

- **Zero-Copy Reads**: Efficiently handles large file contents.
- **Fast Lookups**: Optimized URI-to-source mapping.
- **Minimal Locking**: Designed for high-concurrency access.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

---

**Oak VFS** - A solid foundation for file management 🚀
