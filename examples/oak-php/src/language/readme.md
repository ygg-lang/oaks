# PHP Language Definition

This module contains the metadata and configuration options for the PHP language within the Oak framework.

## ⚙️ Configuration

The `PhpLanguage` struct defines how the parser and lexer should behave to accommodate various PHP versions and project requirements:

```rust
pub struct PhpLanguage {}
```

Currently, `PhpLanguage` serves as a marker struct for PHP support. Future versions may include configuration for:
- **`version`**: Target PHP version (e.g., 7.4, 8.0, 8.2, 8.3+).
- **`attributes`**: Enables parsing of PHP 8.0+ attributes (`#[...]`).
- **`short_tags`**: Configures support for short opening tags (`<? ... ?>`).
- **`strict_types`**: Configures the parser's awareness of `declare(strict_types=1)`.

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering keywords, operators, literals, and complex string tokens.
- **`ElementType`**: Defined in the `parser` module, representing classes, enums, traits, and functions.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the PHP source file.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of PHP's evolving syntax and its massive ecosystem.
