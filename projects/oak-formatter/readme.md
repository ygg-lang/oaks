# 📝 oak-formatter

[![Crates.io](https://img.shields.io/crates/v/oak-formatter.svg)](https://crates.io/crates/oak-formatter)
[![Documentation](https://docs.rs/oak-formatter/badge.svg)](https://docs.rs/oak-formatter)

**High-Level Formatting Library for Oak** — A unified formatting framework for the Oak language ecosystem, providing configuration management, annotation processing, and language-specific formatting rules.

## 🎯 Project Overview

`oak-formatter` is designed to power consistent code formatting across all Oak-supported languages. It abstracts away common formatting concerns, allowing language authors to focus on language-specific rules.

## ✨ Key Features

- **⚙️ Flexible Configuration** — Customizable indentation, line endings, and other formatting options
- **📝 Annotation Support** — Special annotation parsing for code-level formatting directives
- **🌐 Generic Traits** — `Formatter` trait for easy integration with any language
- **🔌 Extensible Architecture** — Plugin-based formatters for different languages
- **🔄 Serde Support** — Optional serialization for configuration persistence

## 🏗️ Core Components

- `config` — Formatting configuration (indent style, line endings, etc.)
- `annotation` — Annotation parser and processor for code-level directives
- `formatters` — Traits and implementations for language-specific formatters
- `errors` — Error types for formatting operations

## 🔗 Ecosystem Integration

Built on top of `oak-core` and used by various Oak language parsers and tools.

## 📖 Documentation

For detailed usage examples and API documentation, visit [docs.rs/oak-formatter](https://docs.rs/oak-formatter).

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
