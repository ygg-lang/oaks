# Rust Language Definition

This module contains the metadata and configuration options for the Rust language within the Oak framework.

## ⚙️ Configuration

The `RustLanguage` struct defines how the parser and lexer should behave to accommodate various Rust editions and compiler features:

```rust
pub struct RustLanguage {}
```

Currently, `RustLanguage` serves as a marker struct for Rust support. Future versions may include configuration for:
- **`edition`**: Specifies the Rust edition to target (e.g., 2015, 2018, 2021, 2024).
- **`unstable`**: Enables parsing support for unstable or nightly-only Rust syntax.
- **`macro_expansion`**: Configures the maximum depth for recursive macro expansion during parsing.
- **`strict`**: Configures how strictly the parser should enforce Rust syntax rules.

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering keywords, operators, and literals.
- **`ElementType`**: Defined in the `parser` module, representing traits, structs, enums, and macros.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the Rust source file.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of Rust's unique syntax and ownership-based semantics.
