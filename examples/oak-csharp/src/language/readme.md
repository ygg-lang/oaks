# C# Language Definition

This module contains the metadata and configuration options for the C# language within the Oak framework.

## ⚙️ Configuration

The `CsharpLanguage` struct defines how the parser and lexer should behave to accommodate various C# versions and project requirements:

```rust
pub struct CsharpLanguage {}
```

Currently, `CsharpLanguage` serves as a marker struct for C# support. Future versions may include configuration for:
- **`version`**: Target C# version (e.g., 10.0, 11.0, 12.0+).
- **`unsafe`**: Enables parsing of `unsafe` blocks and pointer operations.
- **`nullable`**: Configures the parser's awareness of nullable reference types.
- **`strict`**: When enabled, the parser enforces stricter C# syntax rules during analysis.

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering keywords, operators, literals (including raw string literals), and interpolation tokens.
- **`ElementType`**: Defined in the `parser` module, representing records, primary constructors, LINQ, and attributes.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the C# source file.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of C#'s sophisticated type system and its importance in the .NET ecosystem.
