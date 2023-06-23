# Ada Language Definition

This module contains the metadata and configuration options for the Ada language within the Oak framework.

## ⚙️ Configuration

The `AdaLanguage` struct defines how the parser and lexer should behave to accommodate various Ada standards (Ada 83, 95, 2005, 2012, 2022):

```rust
pub struct AdaLanguage {}
```

Currently, `AdaLanguage` serves as a marker struct for Ada support. Future versions may include configuration for:
- **`standard`**: Target Ada standard (e.g., Ada 2012, Ada 2022).
- **`strict`**: When enabled, the parser enforces stricter Ada syntax rules.

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering keywords, operators, and literals.
- **`ElementType`**: Defined in the `parser` module, representing packages, subprograms, and tasking constructs.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the Ada compilation unit.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of Ada's unique syntax and safety features.
