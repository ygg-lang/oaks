# 🗺️ oak-source-map

[![Crates.io](https://img.shields.io/crates/v/oak-source-map.svg)](https://crates.io/crates/oak-source-map)
[![Documentation](https://docs.rs/oak-source-map/badge.svg)](https://docs.rs/oak-source-map)

**High-Performance Source Map v3 Implementation for Rust** — A complete, zero-copy Source Map v3 implementation with VLQ encoding/decoding, builder patterns, and map composition.

## 🎯 Why oak-source-map?

Source maps are essential for debugging bundled and minified code. `oak-source-map` provides a high-performance, feature-complete implementation of the Source Map v3 specification, designed for both parsing and generating source maps efficiently.

## ✨ Key Features

- **📋 Full v3 Specification Support** — Complete implementation of the Source Map v3 format
- **⚡ VLQ Base64 Encoding/Decoding** — Fast and efficient VLQ processing
- **🔄 Zero-Copy Parsing** — Minimal memory overhead when parsing existing maps
- **🏗️ Builder Pattern** — Incremental source map construction
- 🎭 Source Map Composition — Combine multiple source maps for chained transformations
- **🔍 Mapping Lookup** — Efficient line/column mapping queries
- **🔌 Serde Integration** — Seamless serialization and deserialization

## 🏗️ Architecture

- `SourceMap` — Core source map representation
- `SourceMapBuilder` — Incremental map construction
- `SourceMapComposer` — Map composition for chained transformations
- `SourceMapDecoder` — Parser for existing source maps
- `vlq` — VLQ Base64 encoding/decoding utilities
- `mapping` — Mapping and segment data structures

## 🔗 Ecosystem Integration

Used by Oak language tools for debugging support and by bundlers and minifiers in the Rust ecosystem.

## 📖 Documentation

For usage examples and API details, see the [API documentation](https://docs.rs/oak-source-map).

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
