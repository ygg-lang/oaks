# Scala Language Definition

This module contains the metadata and configuration options for the Scala language within the Oak framework.

## ⚙️ Configuration

The `ScalaLanguage` struct defines how the parser and lexer should behave to accommodate various Scala versions and project requirements:

```rust
pub struct ScalaLanguage {}
```

Currently, `ScalaLanguage` serves as a marker struct for Scala support. Future versions may include configuration for:
- **`version`**: Target Scala version (e.g., Scala 2.12, 2.13, 3.x).
- **`experimental_features`**: Enables support for experimental Scala features.
- **`dialect`**: Specific Scala dialects (e.g., Dotty, Typelevel Scala).

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering keywords, operators, identifiers, and literals.
- **`ElementType`**: Defined in the `parser` module, representing packages, classes, objects, traits, and expressions.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the Scala source file.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of Scala's rich and complex syntax.
