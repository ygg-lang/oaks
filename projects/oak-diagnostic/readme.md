# 🚀 oak-diagnostic

[![Crates.io](https://img.shields.io/crates/v/oak-diagnostic.svg)](https://crates.io/crates/oak-diagnostic)
[![Documentation](https://docs.rs/oak-diagnostic/badge.svg)](https://docs.rs/oak-diagnostic)

**Diagnostic Reporting for Oak Languages** — Structured error and warning reporting for language analysis tools.

## 🎯 Why oak-diagnostic?

Diagnostics are the feedback mechanism that helps developers fix issues in their code. `oak-diagnostic` provides a unified infrastructure for reporting errors, warnings, and hints with precise source locations.

## ✨ Key Features

- **📊 Structured Diagnostics** — Rich types with severity, message, and location
- **📍 Precise Locations** — Byte-offset based ranges for accurate highlighting
- **🔧 Related Information** — Support for related locations and additional context
- **🎯 Severity Levels** — Error, warning, information, and hint
- **🔄 Serde Support** — Optional serialization for LSP integration

## 🏗️ Architecture

- `Diagnostic` — Range, severity, message, and related information
- `DiagnosticSeverity` — Error, Warning, Information, Hint

## 🔗 Ecosystem Integration

Used by `oak-lsp` for `textDocument/publishDiagnostics`, language parsers for syntax error reporting, and static analysis tools for linting.

## 📖 Documentation

For usage examples and API details, see the [API documentation](https://docs.rs/oak-diagnostic).

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
