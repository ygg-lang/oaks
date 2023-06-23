# TeX Language Definition

This module contains the metadata and configuration options for the TeX/LaTeX language within the Oak framework.

## ⚙️ Configuration

The `TexLanguage` struct defines how the parser and lexer should behave to accommodate various TeX engines and project requirements:

```rust
pub struct TexLanguage {}
```

Currently, `TexLanguage` serves as a marker struct for TeX support. Future versions may include configuration for:
- **`engine`**: Target TeX engine (e.g., pdfTeX, XeTeX, LuaTeX).
- **`format`**: Target format (e.g., Plain TeX, LaTeX, ConTeXt).
- **`packages`**: Pre-loaded package definitions for better semantic analysis.

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering control sequences, braces, math mode delimiters, and text.
- **`ElementType`**: Defined in the `parser` module, representing commands, environments, math groups, and paragraphs.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the TeX source file.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of TeX's complex macro-based syntax.
