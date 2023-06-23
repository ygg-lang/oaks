# TOML Language Definition

This module contains the metadata and configuration options for the TOML language within the Oak framework.

## ⚙️ Configuration

The `TomlLanguage` struct defines how the parser and lexer should behave to accommodate the TOML specification and its various versions:

```rust
pub struct TomlLanguage {
    pub allow_multiline_strings: bool,
    pub allow_hex_numbers: bool,
    pub datetime_format: DateTimeFormat,
}

pub enum DateTimeFormat {
    Rfc3339,
}
```

### Predefined Configurations

`TomlLanguage` provides several helper methods to create configurations for common TOML variants:

- **`standard()`**: Standard TOML configuration (Multiline strings enabled, hex numbers disabled, RFC3339 date-time).

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering TOML keys, strings, numbers, dates, and structural characters.
- **`ElementType`**: Defined in the `parser` module, representing TOML tables, arrays, and key-value pairs.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the TOML document.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of TOML's unique syntax and hierarchical configuration model.
