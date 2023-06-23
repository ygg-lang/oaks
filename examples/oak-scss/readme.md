# 🚀 Oak SCSS Parser

[![Crates.io](https://img.shields.io/crates/v/oak-scss.svg)](https://crates.io/crates/oak-scss)
[![Documentation](https://docs.rs/oak-scss/badge.svg)](https://docs.rs/oak-scss)

**Modern Styling, Industrial Performance** — A high-performance, incremental SCSS parser built on the Oak framework. Designed for building next-generation CSS tools, IDEs, and static analyzers for the modern web.

## 🎯 Project Vision

SCSS is the cornerstone of modern web styling, enabling complex design systems through its powerful syntax. `oak-scss` aims to provide a robust, Rust-powered infrastructure for parsing SCSS that is both accurate and incredibly fast. By leveraging Oak's incremental parsing architecture, we enable the creation of highly responsive tools that can handle massive stylesheets with ease. Whether you are building a custom linter, a CSS-in-JS generator, or an advanced IDE extension, `oak-scss` provides the high-fidelity AST and sub-millisecond performance required to keep up with the evolving CSS ecosystem.

## ✨ Core Features

- **⚡ Blazing Fast**: Engineered in Rust to deliver sub-millisecond parsing response times, even for complex SCSS files with deep nesting and multiple imports.
- **🔄 Incremental by Design**: Built-in support for partial updates—re-parse only the sections of the stylesheet that changed. Essential for providing real-time feedback in large-scale styling projects.
- **🌳 High-Fidelity AST**: Generates a comprehensive and precise Abstract Syntax Tree capturing the full depth of SCSS:
    - **Variables & Interpolation**: Accurate mapping of variable declarations and their usage in selectors or values.
    - **Mixins & Functions**: Deep support for mixin definitions, includes, and custom function logic.
    - **Control Directives**: Precise tracking of `@if`, `@for`, `@each`, and `@while` blocks.
    - **Nesting & Parent Selectors**: Robust handling of CSS nesting and the `&` operator.
    - **Comments & Trivia**: Retains all trivia (whitespace and comments), enabling faithful round-trip processing and refactoring.
- **🛡️ Industrial-Grade Fault Tolerance**: Gracefully recovers from syntax errors during active development, providing clear and actionable diagnostics.
- **🧩 Deep Ecosystem Integration**: Seamlessly works with `oak-lsp` for full LSP support and `oak-mcp` for intelligent style discovery.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn), which allows for:
1. **Efficient Immutability**: Share nodes across different versions of the tree without copying.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful code formatting and refactoring.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal and analysis.


## 🛠️ Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.
