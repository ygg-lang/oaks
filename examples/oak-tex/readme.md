# 🚀 oak-tex

[![Crates.io](https://img.shields.io/crates/v/oak-tex.svg)](https://crates.io/crates/oak-tex)
[![Documentation](https://docs.rs/oak-tex/badge.svg)](https://docs.rs/oak-tex)

**Typesetting Excellence with Modern Performance** — A high-performance, incremental TeX/LaTeX parser built on the Oak framework. Optimized for complex document structures, math environments, and responsive developer tools for technical writing.

## 🎯 Project Vision

TeX and its derivatives like LaTeX are the gold standard for high-quality typesetting in academia and technical publishing. `oak-tex` aims to provide a robust, modern, Rust-powered infrastructure for parsing TeX that is both accurate and incredibly fast. By utilizing Oak's incremental parsing architecture, we enable the creation of highly responsive IDEs, real-time previewers, and automated document analysis tools that can handle massive TeX projects instantly. Whether you are building custom linters for academic papers, automated bibliography managers, or sophisticated IDE extensions for TeXstudio or VS Code, `oak-tex` provides the high-fidelity AST and efficiency needed to elevate the technical writing experience.

## ✨ Core Features

- **⚡ Blazing Fast**: Leverages Rust's performance and memory safety to provide sub-millisecond parsing, essential for real-time feedback in large TeX documents.
- **🔄 Incremental by Nature**: Built-in support for partial updates—re-parse only what has changed. Perfect for live-preview environments where document responsiveness is critical.
- **🌳 High-Fidelity AST**: Generates a comprehensive and precise syntax tree capturing the full depth of TeX/LaTeX:
    - **Commands & Macros**: Precise mapping of `\command{...}` and custom macro definitions.
    - **Environments**: Detailed tracking of `\begin{...}...\end{...}` blocks.
    - **Math Mode**: Robust parsing of inline `$ ... $` and display `\[ ... \]` math environments.
    - **Document Structure**: Clear identification of sections, subsections, and bibliography references.
    - **Indentation & Formatting**: Precise capture of indentation and whitespace for faithful code refactoring.
- **🛡️ Industrial-Grade Fault Tolerance**: Engineered to recover from syntax errors gracefully, providing precise diagnostics—crucial for maintaining a smooth writing experience during active document preparation.
- **🧩 Deep Ecosystem Integration**: Seamlessly works with `oak-lsp` for full LSP support and `oak-mcp` for intelligent document discovery and analysis.

## 🏗️ Architecture

`oak-tex` follows the modern Green/Red Tree architecture (inspired by Roslyn):

- **Green Tree**: Immutable, lossless, and syntax-only tree. It captures the full fidelity of the source code, including trivia (comments, whitespace).
- **Red Tree**: A facade over the Green Tree that provides a convenient, type-safe API for tree traversal and analysis, including parent pointers and absolute offsets.

This design enables efficient incremental parsing and powerful refactoring capabilities.


## 🛠️ Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.
