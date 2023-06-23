# 🚀 oak-svelte

[![Crates.io](https://img.shields.io/crates/v/oak-svelte.svg)](https://crates.io/crates/oak-svelte)
[![Documentation](https://docs.rs/oak-svelte/badge.svg)](https://docs.rs/oak-svelte)

**Modern Web Development with Incremental Precision** — A high-performance, incremental Svelte component parser built on the Oak framework. Optimized for Svelte 3/4/5 components, complex control blocks, and responsive developer tools for the Svelte ecosystem.

## 🎯 Project Vision

Svelte is a revolutionary framework that shifts much of its work to a compile-time step, and its component architecture requires a sophisticated parser that can handle HTML, logic, and styles with unique control flow syntax. `oak-svelte` aims to provide a robust, modern, Rust-powered infrastructure for parsing Svelte components that is both accurate and incredibly fast. By utilizing Oak's incremental parsing architecture, we enable the creation of highly responsive IDEs, static analyzers, and refactoring tools that can handle massive Svelte projects in real-time. Whether you are building custom linters for Svelte components, automated migration tools, or sophisticated IDE extensions for Svelte Language Server-like experiences, `oak-svelte` provides the high-fidelity AST and efficiency needed to support the modern web developer.

## ✨ Core Features

- **⚡ Blazing Fast**: Leverages Rust's performance and memory safety to provide sub-millisecond parsing, essential for high-frequency developer tools and real-time analysis in large Svelte projects.
- **🔄 Incremental by Nature**: Built-in support for partial updates—re-parse only the specific section (logic, template, or style) that changed. Ideal for real-time IDE feedback and large-scale component analysis.
- **🌳 High-Fidelity AST**: Generates a comprehensive and precise syntax tree capturing the full depth of Svelte components:
    - **Component Structure**: Precise mapping of `<script>`, `<script context="module">`, and `<style>` blocks alongside the template.
    - **Control Blocks**: Detailed tracking of `{#if}`, `{#each}`, `{#await}`, `{#key}`, and Svelte 5 `{#snippet}`.
    - **Expressions**: Robust parsing of mustache interpolations `{ ... }` and directive expressions.
    - **Special Tags**: Support for `{@html}`, `{@const}`, `{@debug}`, etc.
    - **Directives**: Precise parsing of `on:`, `bind:`, `class:`, `use:`, `transition:`, `animate:`, etc.
    - **Indentation & Formatting**: Precise capture of indentation and whitespace for faithful code refactoring.
- **🛡️ Industrial-Grade Fault Tolerance**: Engineered to recover from syntax errors gracefully, providing precise diagnostics—crucial for maintaining a smooth developer experience during active coding.
- **🧩 Deep Ecosystem Integration**: Seamlessly works with `oak-lsp` for full LSP support and `oak-mcp` for intelligent component discovery and analysis.

## 🏗️ Architecture

`oak-svelte` follows the modern Green/Red Tree architecture (inspired by Roslyn):

- **Green Tree**: Immutable, lossless, and syntax-only tree. It captures the full fidelity of the source code, including trivia (comments, whitespace).
- **Red Tree**: A facade over the Green Tree that provides a convenient, type-safe API for tree traversal and analysis, including parent pointers and absolute offsets.

This design enables efficient incremental parsing and powerful refactoring capabilities.


## 🛠️ Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.
