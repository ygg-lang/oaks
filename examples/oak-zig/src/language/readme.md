# Zig Language Definition

This module contains the metadata and configuration options for the Zig language within the Oak framework.

## ⚙️ Configuration

The `ZigLanguage` struct defines how the parser and lexer should behave to accommodate various Zig versions and language features:

```rust
pub struct ZigLanguage {}
```

Currently, `ZigLanguage` serves as a marker struct for Zig support. Future versions may include configuration for:
- **`version`**: Target Zig version (e.g., 0.11.0, 0.12.0).
- **`comptime_eval`**: Configures how the parser should handle or simplify complex compile-time expressions.
- **`strict`**: When enabled, the parser enforces stricter Zig syntax rules.

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering keywords, operators, and literals.
- **`ElementType`**: Defined in the `parser` module, representing functions, comptime blocks, and expressions.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the Zig source file.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of Zig's unique syntax and systems-oriented features.
