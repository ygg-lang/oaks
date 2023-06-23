# 🚀 oak-fsharp

[![Crates.io](https://img.shields.io/crates/v/oak-fsharp.svg)](https://crates.io/crates/oak-fsharp)
[![Documentation](https://docs.rs/oak-fsharp/badge.svg)](https://docs.rs/oak-fsharp)

**Making F# processing simple** — A high-performance, incremental F# parser built on the Oak framework.

## 🎯 Project Vision

`oak-fsharp` is dedicated to providing industrial-grade parsing support for the F# language. By leveraging Rust's high-performance characteristics and Oak's incremental parsing architecture, it can easily handle a variety of application scenarios, from simple script analysis to complex IDE language servers.

## ✨ Core Features

- **⚡ Blazing Fast**: Fully utilizes Rust's performance advantages to achieve sub-millisecond parsing response times.
- **🔄 Incremental Parsing**: Built-in support for partial updates, demonstrating extremely high efficiency when processing large files.
- **🌳 High-Fidelity AST**: Captures language-specific constructs and trivia (comments/whitespace) for refactoring and analysis.
- **🛡️ Robustness**: Features a comprehensive error recovery mechanism, ensuring normal operation even when input is incomplete.
- **🧩 Easy Integration**: Designed with high cohesion and low coupling, allowing for quick integration into existing Rust projects.

## 🏗️ Architecture

`oak-fsharp` follows the modern Green/Red Tree architecture (inspired by Roslyn):

- **Green Tree**: Immutable, lossless, and syntax-only tree. It captures the full fidelity of the source code, including trivia (comments, whitespace).
- **Red Tree**: A facade over the Green Tree that provides a convenient, type-safe API for tree traversal and analysis, including parent pointers and absolute offsets.

This design enables efficient incremental parsing and powerful refactoring capabilities.


## 🛠️ Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.
