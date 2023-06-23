# JSON Language Definition

This module contains the metadata and configuration options for the JSON language within the Oak framework.

## ⚙️ Configuration

The `JsonLanguage` struct defines how the parser and lexer should behave to accommodate standard JSON as well as common extensions like JSON5:

```rust
pub struct JsonLanguage {
    /// Whether to allow trailing commas in objects and arrays
    pub trailing_comma: bool,
    /// Whether to allow bare keys (unquoted keys) in objects
    pub bare_keys: bool,
    /// Whether to allow single-quoted strings
    pub single_quotes: bool,
    /// Whether to allow comments (both line and block)
    pub comments: bool,
    /// Whether to allow hexadecimal numbers (e.g., 0xDEADBEEF)
    pub hex_numbers: bool,
    /// Whether to allow Infinity, -Infinity, and NaN
    pub infinity_and_nan: bool,
}
```

### Predefined Configurations

`JsonLanguage` provides several helper methods to create configurations for common JSON variants:

- **`standard()`**: Strict ANSI JSON configuration (no extensions).
- **`json5()`**: Full JSON5 support (comments, trailing commas, bare keys, etc.).
- **`relaxed()`**: Alias for JSON5, allowing for a more flexible parsing experience.

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering JSON values, structural characters, and extension-specific tokens.
- **`ElementType`**: Defined in the `parser` module, representing JSON objects, arrays, and value types.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the JSON document.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of JSON's unique syntax and its modern extensions.
