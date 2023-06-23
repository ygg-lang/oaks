# HTML Language Definition

This module contains the metadata and configuration options for the HTML language within the Oak framework.

## ⚙️ Configuration

The `HtmlLanguage` struct defines how the parser and lexer should behave to accommodate various HTML versions and project requirements:

```rust
pub struct HtmlLanguage {}
```

Currently, `HtmlLanguage` serves as a marker struct for HTML support. Future versions may include configuration for:
- **`version`**: Target HTML version (e.g., HTML 4.01, HTML5).
- **`foreign`**: Configures support for foreign elements like SVG and MathML within HTML.
- **`strict`**: When enabled, the parser enforces stricter HTML5 rules.
- **`self_closing`**: Configures how the parser handles trailing slashes on non-void elements (e.g., `<div />`).

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering tags, attributes, text, and comments.
- **`ElementType`**: Defined in the `parser` module, representing elements, attributes, and doctypes.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the HTML source file.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of HTML's unique hierarchical syntax and web-centric features.
