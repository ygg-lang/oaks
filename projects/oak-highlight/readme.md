# 🚀 oak-highlight

[![Crates.io](https://img.shields.io/crates/v/oak-highlight.svg)](https://crates.io/crates/oak-highlight)
[![Documentation](https://docs.rs/oak-highlight/badge.svg)](https://docs.rs/oak-highlight)

**Multi-Format Syntax Highlighting for Oak Languages** — A flexible highlighting library with support for ANSI, HTML, CSS, and JSON output formats.

## 🎯 Why oak-highlight?

Syntax highlighting transforms code from plain text into visually structured, readable content. `oak-highlight` provides a unified highlighting infrastructure that works across all Oak language parsers.

## ✨ Key Features

- **🎨 Multiple Export Formats** — ANSI, HTML, CSS classes, and JSON
- **🔤 Token-Based Highlighting** — Fast, accurate highlighting from token streams
- **🎭 Theme Support** — Pluggable theming with predefined and custom themes
- **⚡ Zero-Allocation Design** — Highlight results borrow from source text
- **🧩 Language Agnostic** — Works with any Oak language parser

## 🏗️ Architecture

- `OakHighlighter` — Main highlighter processing source into segments
- `HighlightResult` — Collection of highlighted segments with style
- `Exporter` trait — Convert results to various output formats

### Available Exporters

| Exporter | Output Format | Use Case |
|----------|---------------|----------|
| `AnsiExporter` | ANSI escape codes | Terminal, CLI tools |
| `HtmlExporter` | HTML with inline styles | Web applications |
| `CssExporter` | HTML with CSS classes | Custom styling |
| `JsonExporter` | JSON array | Tooling integration |

## 🔗 Ecosystem Integration

Used by `oak-repl` for terminal highlighting, `oak-lsp` for semantic tokens, and language-specific parsers for preview generation.

## 📖 Documentation

For usage examples and API details, see the [API documentation](https://docs.rs/oak-highlight).

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
