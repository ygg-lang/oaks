# 🚀 oak-ruby

[![Crates.io](https://img.shields.io/crates/v/oak-ruby.svg)](https://crates.io/crates/oak-ruby)
[![Documentation](https://docs.rs/oak-ruby/badge.svg)](https://docs.rs/oak-ruby)

**Elegance Meets Performance** — A high-performance, incremental Ruby parser built on the Oak framework. Optimized for static analysis, automated refactoring, and modern IDE integration for the Ruby programming language.

## 🎯 Project Vision

Ruby is celebrated for its developer happiness and expressive syntax. `oak-ruby` aims to bring that same level of joy to tool developers by providing a fast, modern, Rust-powered parsing foundation. By leveraging Oak's incremental parsing architecture, it enables the creation of highly responsive tools that can handle large Ruby codebases—from Rails applications to complex DSLs. We provide the high-fidelity AST and efficiency needed to build the next generation of Ruby developer tools, ensuring that developers can focus on writing elegant code while their tools keep up with them in real-time.

## ✨ Core Features

- **⚡ Blazing Fast**: Combines Rust's efficiency with advanced parsing techniques to deliver sub-millisecond response times, even for complex Ruby scripts with deep nesting.
- **🔄 Incremental Parsing**: Only re-parse what changed. Perfect for IDEs where rapid feedback is essential for maintaining developer flow in large projects.
- **🌳 High-Fidelity AST**: Generates a detailed and easy-to-traverse syntax tree capturing:
    - **OOP Structures**: Full support for Classes, Modules, Inheritance, and Mixins.
    - **Flexible Syntax**: Precise handling of Ruby's unique features like heredocs, percent literals (`%q`, `%w`, `%x`), and command-style method calls.
    - **Functional Blocks**: Robust parsing of Blocks (`do...end` and `{...}`), Procs, and Lambdas.
    - **DSL Support**: Optimized for parsing domain-specific languages common in the Ruby ecosystem (e.g., Rails routes, RSpec tests).
- **🛡️ Robust Error Recovery**: Engineered to handle malformed or incomplete Ruby code gracefully, providing precise diagnostics while maintaining a valid syntax tree for continuous analysis.
- **🧩 Ecosystem Integration**: Seamlessly works with `oak-lsp` for full LSP support and other Oak-based code analysis utilities.

## 🏗️ Architecture

`oak-ruby` follows the modern Green/Red Tree architecture (inspired by Roslyn):

- **Green Tree**: Immutable, lossless, and syntax-only tree. It captures the full fidelity of the source code, including trivia (comments, whitespace).
- **Red Tree**: A facade over the Green Tree that provides a convenient, type-safe API for tree traversal and analysis, including parent pointers and absolute offsets.

This design enables efficient incremental parsing and powerful refactoring capabilities.


## 🛠️ Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.
