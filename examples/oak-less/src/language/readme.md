# Less Language Definition

This module contains the metadata and configuration options for the Less language within the Oak framework.

## ⚙️ Configuration

The `LessLanguage` struct defines how the parser and lexer should behave to accommodate various Less versions and project requirements:

```rust
pub struct LessLanguage {}
```

Currently, `LessLanguage` serves as a marker struct for Less support. Future versions may include configuration for:
- **`version`**: Target Less version or module set.
- **`nesting`**: Configures support for Less nesting features.
- **`variables`**: Enables parsing of Less Variables (`@var-name`).
- **`strict`**: When enabled, the parser enforces stricter Less rules.
- **`browsers`**: Configures the parser to be aware of specific browser-specific properties and hacks.

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering selectors, property names/values, and at-rules.
- **`ElementType`**: Defined in the `parser` module, representing rulesets, declarations, and media queries.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the Less stylesheet.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of Less's unique declarative syntax and the modern web design ecosystem.
