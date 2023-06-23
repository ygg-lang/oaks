# XML Language Definition

This module contains the metadata and configuration options for the XML language within the Oak framework.

## ⚙️ Configuration

The `XmlLanguage` struct defines how the parser and lexer should behave to accommodate various XML standards and features:

```rust
pub struct XmlLanguage {}
```

Currently, `XmlLanguage` serves as a marker struct for XML support. Future versions may include configuration for:
- **`version`**: Target XML version (e.g., 1.0, 1.1).
- **`strict_mode`**: Enforce stricter adherence to the XML specification (e.g., DTD validation).
- **`namespace_aware`**: Enable or disable XML namespace processing.
- **`handle_comments`**: Configure how comments are processed and retained.

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering XML tags, attributes, text content, and entities.
- **`ElementType`**: Defined in the `parser` module, representing XML elements, attribute lists, and processing instructions.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the XML document.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of XML's unique markup syntax and tree-based data model.
