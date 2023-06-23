# Swift Language Definition

This module contains the metadata and configuration options for the Swift language within the Oak framework.

## ⚙️ Configuration

The `SwiftLanguage` struct defines how the parser and lexer should behave to accommodate various Swift versions and project requirements:

```rust
pub struct SwiftLanguage {}
```

Currently, `SwiftLanguage` serves as a marker struct for Swift support. Future versions may include configuration for:
- **`version`**: Target Swift version (e.g., 5.9, 5.10, 6.0+).
- **`macros`**: Enables support for parsing Swift macros (`#...`).
- **`experimental`**: Enables parsing of experimental syntax from recent Swift Evolution proposals.
- **`concurrency`**: Configures the parser's awareness of strict concurrency checks (e.g., `@Sendable`).

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering keywords, custom operators, literals, and interpolation tokens.
- **`ElementType`**: Defined in the `parser` module, representing property wrappers, actors, result builders, and generic parameters.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the Swift source file.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of Swift's sophisticated type system and its importance in the Apple development ecosystem.
