# 🚀 oak-macros

[![Crates.io](https://img.shields.io/crates/v/oak-macros.svg)](https://crates.io/crates/oak-macros)
[![Documentation](https://docs.rs/oak-macros/badge.svg)](https://docs.rs/oak-macros)

**Procedural Macros for Oak** — Derive macros and helper attributes for Oak language implementations.

## 🎯 Why oak-macros?

Procedural macros reduce boilerplate in language implementations. `oak-macros` provides derive macros and helper attributes that simplify implementing Oak traits.

## ✨ Key Features

- **🔧 Derive Macros** — Automatically implement common Oak traits
- **📝 Code Generation** — Generate repetitive code patterns
- **🎯 Type-Safe** — Compile-time verification of generated code
- **⚡ Zero Runtime Cost** — Macros expand at compile time

## 🏗️ Architecture

### Available Macros

| Macro | Purpose |
|-------|---------|
| `#[derive(Language)]` | Auto-implement the `Language` trait |
| `json!` | JSON literal construction |

## 🔗 Ecosystem Integration

Used by language parser implementations, test code generation, and AST construction utilities.

## 📖 Documentation

For usage examples and API details, see the [API documentation](https://docs.rs/oak-macros).

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
