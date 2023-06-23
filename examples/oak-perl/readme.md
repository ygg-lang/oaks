# 🚀 Oak Perl Parser

[![Crates.io](https://img.shields.io/crates/v/oak-perl.svg)](https://crates.io/crates/oak-perl)
[![Documentation](https://docs.rs/oak-perl/badge.svg)](https://docs.rs/oak-perl)

**Legacy Power, Modern Performance** — A high-performance, incremental Perl parser built on the Oak framework. Optimized for complex legacy codebase maintenance, large-scale text processing, and real-time developer tools.

## 🎯 Project Vision

Perl has a long history as the "Swiss Army knife" of programming, known for its powerful text processing and flexible syntax. `oak-perl` aims to provide a robust, modern, Rust-powered infrastructure for parsing Perl that is both accurate and incredibly fast. By utilizing Oak's incremental parsing architecture, we enable the creation of highly responsive IDEs, code analysis tools, and automated refactoring utilities that can handle complex Perl projects in real-time. Whether you are building custom linters, automated migration tools, or sophisticated IDE extensions, `oak-perl` provides the high-fidelity AST and efficiency needed to support the Perl ecosystem.

## ✨ Core Features

- **⚡ Blazing Fast**: Leverages Rust's performance and memory safety to provide sub-millisecond parsing, essential for high-frequency developer tools and real-time analysis in large Perl projects.
- **🔄 Incremental by Nature**: Built-in support for partial updates—re-parse only what has changed. Ideal for massive Perl scripts where maintainability and tool responsiveness are critical.
- **🌳 High-Fidelity AST**: Generates a comprehensive and precise Abstract Syntax Tree capturing the full depth of Perl:
    - **Regex Support**: Precise mapping of Perl's powerful regular expression literals and operators.
    - **Variable Sigils**: Detailed tracking of scalar (`$`), array (`@`), and hash (`%`) variables.
    - **Control Flow**: Robust parsing of complex loops, conditionals, and statement modifiers.
    - **Subroutines & Packages**: Support for modular programming constructs and package declarations.
    - **Comments & Whitespace**: Retains all trivia, enabling faithful round-trip processing and refactoring.
- **🛡️ Industrial-Grade Fault Tolerance**: Engineered to recover from syntax errors gracefully, providing precise diagnostics—crucial for maintaining a smooth developer experience during active coding.
- **🧩 Deep Ecosystem Integration**: Seamlessly works with `oak-lsp` for full LSP support and `oak-mcp` for intelligent code discovery and analysis.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn), which allows for:
1. **Efficient Immutability**: Share nodes across different versions of the tree without copying.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful code formatting and refactoring of Perl files.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal and analysis.


## 🛠️ Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.
