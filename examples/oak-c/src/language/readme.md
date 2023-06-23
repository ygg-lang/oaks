# C Language Definition

This module contains the metadata and configuration options for the C language within the Oak framework.

## ⚙️ Configuration

The `CLanguage` struct defines how the parser and lexer should behave to accommodate various C standards (C89, C99, C11, etc.):

```rust
pub struct CLanguage {}
```

Currently, `CLanguage` serves as a marker struct for C support. Future versions may include configuration for:
- **`standard`**: Target C standard (e.g., C89, C99, C11, C17, C23).
- **`extensions`**: Enable or disable specific compiler extensions (e.g., GCC, Clang, MSVC).
- **`preprocess`**: Configure how the preprocessor should be handled during parsing.

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering keywords, identifiers, literals, and operators.
- **`ElementType`**: Defined in the `parser` module, representing declarations, expressions, statements, and function definitions.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the C translation unit.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of C's unique syntax and semantics.
