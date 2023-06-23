# 🚀 Oak Django Template Parser

[![Crates.io](https://img.shields.io/crates/v/oak-django.svg)](https://crates.io/crates/oak-django)
[![Documentation](https://docs.rs/oak-django/badge.svg)](https://docs.rs/oak-django)

**Dynamic Web Templating with Speed and Safety** — A high-performance, incremental Django template parser built on the Oak framework. Optimized for web development, server-side rendering, and modern developer tooling.

## 🎯 Project Vision

Django templates are a cornerstone of the Python web ecosystem, providing a powerful and extensible way to generate dynamic HTML. `oak-django` aims to provide a robust, modern, Rust-powered infrastructure for parsing Django templates that is both accurate and incredibly fast. By utilizing Oak's incremental parsing architecture, we enable the creation of highly responsive IDEs, linting tools, and automated refactoring utilities that can handle complex template hierarchies in real-time. Whether you are building custom template validators, automated theme generators, or sophisticated IDE extensions, `oak-django` provides the high-fidelity AST and efficiency needed to enhance the Django developer experience.

## ✨ Core Features

- **⚡ Blazing Fast**: Leverages Rust's performance and memory safety to provide sub-millisecond parsing, essential for high-frequency developer tools and real-time template analysis.
- **🔄 Incremental by Nature**: Built-in support for partial updates—re-parse only modified sections of large template files. Ideal for complex web projects where maintainability and tool responsiveness are critical.
- **🌳 High-Fidelity AST**: Generates a comprehensive and precise Abstract Syntax Tree capturing the full depth of Django templates:
    - **Variables & Tags**: Precise mapping of `{{ variable }}` and `{% tag %}` constructs.
    - **Filters**: Full support for variable filters like `{{ value|filter }}`.
    - **Inheritance**: Robust handling of `{% extends %}` and `{% block %}` for complex template hierarchies.
    - **Control Flow**: Detailed tracking of `{% if %}`, `{% for %}`, and other control flow tags.
    - **Comments & Whitespace**: Retains all trivia, including `{# comment #}`, enabling faithful round-trip processing and refactoring.
- **🛡️ Industrial-Grade Fault Tolerance**: Engineered to recover from syntax errors gracefully, providing precise diagnostics—crucial for maintaining a smooth developer experience during active template development.
- **🧩 Deep Ecosystem Integration**: Seamlessly works with `oak-lsp` for full LSP support and `oak-mcp` for intelligent template discovery and analysis.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn), which allows for:
1. **Efficient Immutability**: Share nodes across different versions of the tree without copying.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful code formatting and refactoring of Django template files.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal and analysis.


## 🤝 Contributing

We welcome contributions of all kinds! If you find a bug, have a feature request, or want to contribute code, please check our [issues](https://github.com/ygg-lang/oaks/issues) or submit a pull request.
