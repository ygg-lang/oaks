# Coq Language Definition

This module contains the metadata and configuration options for the Coq language within the Oak framework.

## ⚙️ Configuration

The `CoqLanguage` struct defines how the parser and lexer should behave to accommodate various Coq versions and interactive features:

```rust
pub struct CoqLanguage {}
```

Currently, `CoqLanguage` serves as a marker struct for Coq support. Future versions may include configuration for:
- **`version`**: Target Coq version (e.g., 8.15, 8.16, 8.17).
- **`notation_level`**: Configures how the parser should handle complex nested notations.
- **`tactic_depth`**: Limits the depth of tactic analysis for large proof scripts.

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering keywords, operators, and notation delimiters.
- **`ElementType`**: Defined in the `parser` module, representing vernacular commands, Gallina terms, and tactics.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the Coq script file.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of Coq's unique syntax and formal verification model.
