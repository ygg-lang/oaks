# CSS Language Definition

This module contains the metadata and configuration options for the CSS language within the Oak framework.

## ⚙️ Configuration

The `CssLanguage` struct defines how the parser and lexer should behave to accommodate various CSS versions and project requirements:

```rust
pub struct CssLanguage {}
```

Currently, `CssLanguage` serves as a marker struct for CSS support. Future versions may include configuration for:
- **`version`**: Target CSS version or module set (e.g., CSS 2.1, CSS3, Modern CSS).
- **`nesting`**: Configures support for the modern CSS Nesting module.
- **`variables`**: Enables parsing of CSS Variables (`--var-name`).
- **`strict`**: When enabled, the parser enforces stricter CSS rules.
- **`browsers`**: Configures the parser to be aware of specific browser-specific properties and hacks.

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering selectors, property names/values, and at-rules.
- **`ElementType`**: Defined in the `parser` module, representing rulesets, declarations, and media queries.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the CSS stylesheet.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of CSS's unique declarative syntax and the modern web design ecosystem.
