# J Language Definition

This module contains the metadata and configuration options for the J language within the Oak framework.

## 鈿欙笍 Configuration

The `JLanguage` struct defines how the parser and lexer should behave to accommodate various J standards (J 83, 95, 2005, 2012, 2022):

```rust
pub struct JLanguage {}
```

Currently, `JLanguage` serves as a marker struct for J support. Future versions may include configuration for:
- **`standard`**: Target J standard (e.g., J 2012, J 2022).
- **`strict`**: When enabled, the parser enforces stricter J syntax rules.

## 馃З Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering keywords, operators, and literals.
- **`ElementType`**: Defined in the `parser` module, representing packages, subprograms, and tasking constructs.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the J compilation unit.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of J's unique syntax and safety features.
