# Vue Language Definition

This module contains the metadata and configuration options for the Vue language within the Oak framework.

## ⚙️ Configuration

The `VueLanguage` struct defines how the parser and lexer should behave to accommodate various Vue versions and project requirements:

```rust
pub struct VueLanguage {}
```

Currently, `VueLanguage` serves as a marker struct for Vue support. Future versions may include configuration for:
- **`script_lang`**: Default language for script blocks (e.g., JS, TS).
- **`style_lang`**: Default language for style blocks (e.g., CSS, SCSS, Less).
- **`experimental_features`**: Enables support for experimental Vue features (e.g., Vapor mode).

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering HTML tags, JS tokens, CSS selectors, and SFC structural tokens.
- **`ElementType`**: Defined in the `parser` module, representing template elements, script setup blocks, style blocks, and directives.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the Vue SFC.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of Vue's multi-paradigm SFC syntax.
