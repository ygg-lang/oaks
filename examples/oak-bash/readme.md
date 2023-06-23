# 🚀 Oak Bash Parser

[![Crates.io](https://img.shields.io/crates/v/oak-bash.svg)](https://crates.io/crates/oak-bash)
[![Documentation](https://docs.rs/oak-bash/badge.svg)](https://docs.rs/oak-bash)

**Industrial-Grade Shell Script Analysis** — A high-performance, incremental Bash parser built on the Oak framework. Optimized for complex shell scripts, command-line tools, and modern IDE integration.

## 🎯 Project Vision

Shell scripts are the glue of the modern computing world, yet they are notoriously difficult to parse and analyze due to their flexible and often ambiguous syntax. `oak-bash` aims to provide a robust, modern, Rust-powered infrastructure for parsing Bash that is both accurate and incredibly fast. By utilizing Oak's incremental parsing architecture, we enable the creation of highly responsive IDEs, static analyzers (like a modern ShellCheck), and automated refactoring tools that can handle massive script libraries in real-time. Whether you are building custom linters, DevOps automation tools, or sophisticated IDE extensions, `oak-bash` provides the high-fidelity AST and efficiency needed to tame the complexity of Bash.

## ✨ Core Features

- **⚡ Blazing Fast**: Leverages Rust's performance and memory safety to provide sub-millisecond parsing, essential for high-frequency developer tools and real-time script analysis.
- **🔄 Incremental by Nature**: Built-in support for partial updates—re-parse only what has changed. Ideal for long scripts and continuous integration pipelines.
- **🌳 High-Fidelity AST**: Generates a comprehensive and precise Abstract Syntax Tree capturing the full depth of Bash:
    - **Complex Commands**: Full support for pipelines, loops (`for`, `while`, `until`), and conditional constructs (`if`, `case`).
    - **Variable Expansion**: Precise mapping of parameter expansion, command substitution, and arithmetic expansion.
    - **Redirections**: Detailed tracking of file descriptor redirections and here-documents.
    - **Functions**: Robust parsing of function definitions and call sites.
    - **Shell Options**: Awareness of common shell behaviors and extensions.
- **🛡️ Industrial-Grade Fault Tolerance**: Engineered to recover from syntax errors gracefully, providing precise diagnostics—crucial for maintaining a smooth developer experience during active script editing.
- **🧩 Deep Ecosystem Integration**: Seamlessly works with `oak-lsp` for full LSP support and `oak-mcp` for intelligent code discovery and analysis.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn), which allows for:
1. **Efficient Immutability**: Share nodes across different versions of the tree without copying.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful code formatting and refactoring of Bash scripts.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal and analysis.


## 🤝 Contributing

We welcome contributions of all kinds! If you find a bug, have a feature request, or want to contribute code, please check our [issues](https://github.com/ygg-lang/oaks/issues) or submit a pull request.
