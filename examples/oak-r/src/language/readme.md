# R Language Definition

This module contains the metadata and configuration options for the R language within the Oak framework.

## ⚙️ Configuration

The `RLanguage` struct defines how the parser and lexer should behave to accommodate various R versions and features:

```rust
pub struct RLanguage {}
```

Currently, `RLanguage` serves as a marker struct for R support. Future versions may include configuration for:
- **`version`**: Target R version (e.g., 3.6, 4.0, 4.3).
- **`strict`**: When enabled, the parser enforces stricter R syntax rules.
- **`extensions`**: Enables support for specific R package-related syntax extensions if applicable.

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering keywords, operators, and literals.
- **`ElementType`**: Defined in the `parser` module, representing functions, vectorized operations, and formulas.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the R script.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of R's unique syntax and data-oriented features.
