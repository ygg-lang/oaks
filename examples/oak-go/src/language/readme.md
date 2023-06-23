# Go Language Definition

This module contains the metadata and configuration options for the Go language within the Oak framework.

## ⚙️ Configuration

The `GoLanguage` struct defines how the parser and lexer should behave to accommodate various Go versions and project structures:

```rust
pub struct GoLanguage {}
```

Currently, `GoLanguage` serves as a marker struct for Go support. Future versions may include configuration for:
- **`version`**: Target Go version (e.g., 1.18+ for generics support).
- **`generics`**: Explicitly enables or disables support for Go generics (Type Parameters).
- **`strict`**: Configures how strictly the parser should enforce Go syntax rules during analysis.

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering keywords, operators, and literals.
- **`ElementType`**: Defined in the `parser` module, representing packages, interfaces, goroutines, and methods.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the Go source file.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of Go's unique syntax and concurrency model.
