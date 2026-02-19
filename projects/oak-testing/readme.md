# 🚀 oak-testing

[![Crates.io](https://img.shields.io/crates/v/oak-testing.svg)](https://crates.io/crates/oak-testing)
[![Documentation](https://docs.rs/oak-testing/badge.svg)](https://docs.rs/oak-testing)

**Testing Utilities for Oak Parsers** — Helpers and macros for testing Oak language implementations.

## 🎯 Why oak-testing?

Testing parsers requires specialized utilities for comparing syntax trees, checking error recovery, and validating incremental parsing. `oak-testing` provides these tools for straightforward and maintainable test writing.

## ✨ Key Features

- **🧪 Parse Testing** — Helpers for parsing and validating syntax trees
- **📊 Snapshot Testing** — Compare parsed output against expected snapshots
- **🔍 Error Checking** — Validate error recovery and diagnostic output
- **⚡ Incremental Testing** — Test incremental parsing behavior
- **📝 Macro Support** — Convenient macros for common test patterns

## 🏗️ Architecture

- Parse assertion macros
- Snapshot testing utilities
- Error validation helpers

## 🔗 Ecosystem Integration

Used by all Oak language parsers for unit tests, integration tests for LSP features, and CI pipelines.

## 📖 Documentation

For usage examples and API details, see the [API documentation](https://docs.rs/oak-testing).

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
