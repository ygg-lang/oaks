# YAML Language Definition

This module contains the metadata and configuration options for the YAML language within the Oak framework.

## ⚙️ Configuration

The `YamlLanguage` struct defines how the parser and lexer should behave to accommodate different YAML versions and features:

```rust
pub struct YamlLanguage {}
```

Currently, `YamlLanguage` serves as a marker struct for YAML support. Future versions may include configuration for:
- **`version`**: Target YAML version (e.g., 1.1, 1.2).
- **`strict_mode`**: Enforce stricter adherence to the YAML specification.
- **`allow_custom_tags`**: Enable or disable support for custom YAML tags.

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering YAML structural tokens, scalars, and document markers.
- **`ElementType`**: Defined in the `parser` module, representing YAML mappings, sequences, and scalars.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the YAML document.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of YAML's unique syntax and hierarchical data model.
