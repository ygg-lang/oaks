# 💎 Oak Core

[![Crates.io](https://img.shields.io/crates/v/oak-core.svg)](https://crates.io/crates/oak-core)
[![Documentation](https://docs.rs/oak-core/badge.svg)](https://docs.rs/oak-core)

**The Engine of Language Intelligence** — `oak-core` is the heartbeat of the Oak ecosystem, providing the fundamental data structures and algorithms required for high-performance, incremental language processing.

## 🎯 Project Vision

Modern developer tools demand more than just a static parser; they require a dynamic system that can react to every keystroke. `oak-core` is built to provide this infrastructure. It abstracts away the complexities of incremental parsing, syntax tree management, and error recovery, allowing language authors to focus on the unique semantics of their language.

## ✨ Core Pillars

- **🌳 Persistent Syntax Trees**: Implements the "Green/Red Tree" architecture, enabling efficient immutability, lossless code representation, and high-performance tree transformations.
- **🔄 Universal Incrementalism**: Provides a generic framework for incremental updates that can be applied to any language, ensuring that only the necessary parts of the AST are re-evaluated.
- **🛡️ Advanced Error Recovery**: Features a sophisticated suite of recovery strategies that allow parsers to produce meaningful syntax trees even from highly fragmented or incorrect source code.
- **🚀 Zero-Cost Abstractions**: Written in pure Rust, the core ensures that the flexibility of the Oak framework comes with minimal runtime overhead.
- **🧩 Extensible Framework**: Designed as a modular library, it serves as the base for all `oak-*` parsers and high-level tools like `oak-lsp`.

## 🏗️ Architecture at a Glance

1. **Green Tree**: An immutable, simplified, and address-agnostic representation of the syntax tree, optimized for sharing and caching.
2. **Red Tree**: A thin, type-safe layer over the Green Tree that provides parent pointers and absolute positioning, optimized for developer ergonomics.
3. **Lexer/Parser Traits**: Standardized interfaces that ensure consistency across all language implementations in the Oak ecosystem.
