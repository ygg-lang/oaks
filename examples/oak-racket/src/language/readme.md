# Scheme Language Definition

This module contains the metadata and configuration options for the Scheme language within the Oak framework.

## ⚙️ Configuration

The `SchemeLanguage` struct defines how the parser and lexer should behave to accommodate various Scheme standards and project requirements:

```rust
pub struct SchemeLanguage {}
```

Currently, `SchemeLanguage` serves as a marker struct for Scheme support. Future versions may include configuration for:
- **`standard`**: Target Scheme standard (e.g., R5RS, R6RS, R7RS).
- **`extensions`**: Enables support for specific implementation extensions (e.g., Guile, Chicken, Racket).
- **`case_sensitivity`**: Configures symbol case sensitivity.

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering parentheses, symbols, literals, and keywords.
- **`ElementType`**: Defined in the `parser` module, representing lists, atoms, definitions, and expressions.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the Scheme source file.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of Scheme's minimalist and powerful syntax.
