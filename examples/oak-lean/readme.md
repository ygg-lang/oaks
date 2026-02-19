# 🚀 Oak Lean Parser

[![Crates.io](https://img.shields.io/crates/v/oak-lean.svg)](https://crates.io/crates/oak-lean)
[![Documentation](https://docs.rs/oak-lean/badge.svg)](https://docs.rs/oak-lean)

**Formal Verification Meets Modern Tooling** — A high-performance, incremental Lean parser built on the Oak framework.

## 🎯 Why oak-lean?

Lean is a powerful theorem prover and programming language used for formal verification and mathematical proofs. `oak-lean` brings modern parsing infrastructure to the Lean ecosystem for building responsive IDEs and verification tools.

## ✨ Key Features

- **⚡ Blazing Fast** — Sub-millisecond parsing for interactive proof development
- **🔄 Incremental Parsing** — Essential for frequently-changing proof files
- **🌳 High-Fidelity AST** — Captures theorems, lemmas, proofs, type classes, unification
- **🛡️ Robust Error Recovery** — Handles incomplete proofs gracefully
- **🧩 Deep Ecosystem Integration** — Works with `oak-lsp` and `oak-mcp`

## 🏗️ Architecture

Follows the Green/Red Tree architecture with efficient immutability, lossless syntax trees, and type-safe traversal.

## 📖 Documentation

For usage examples and API details, see the [API documentation](https://docs.rs/oak-lean).

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
