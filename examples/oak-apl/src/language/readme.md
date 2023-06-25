# APL Language Definition

This module contains the metadata and configuration options for the APL language within the Oak framework.

## ⚙️ Configuration

The `AplLanguage` struct defines how the parser and lexer should behave:

```rust
pub struct AplLanguage {
    /// Whether to enable APL extension features
    pub allow_extensions: bool,
    /// Whether to enable strict mode
    pub strict_mode: bool,
}
```

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering APL symbols, literals, and identifiers.
- **`ElementType`**: Defined in the `parser` module, representing statements, expressions, and assignments.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the APL source.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of APL's unique symbolic syntax and array-oriented nature.
