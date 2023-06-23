# Java Language Definition

This module contains the metadata and configuration options for the Java language within the Oak framework.

## ⚙️ Configuration

The `JavaLanguage` struct defines how the parser and lexer should behave to accommodate various Java versions and project structures:

```rust
pub struct JavaLanguage {}
```

Currently, `JavaLanguage` serves as a marker struct for Java support. Future versions may include configuration for:
- **`version`**: Target Java version (e.g., 8, 11, 17, 21).
- **`preview`**: Enables parsing support for preview features in the targeted Java version.
- **`strict`**: Configures how strictly the parser should enforce Java syntax rules.
- **`stdlib`**: Configures how the parser identifies and handles core Java standard library types.

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering keywords, identifiers, literals, and operators.
- **`ElementType`**: Defined in the `parser` module, representing classes, records, annotations, and methods.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the Java source file.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of Java's unique syntax and enterprise-oriented features.
