# COBOL Language Definition

This module contains the metadata and configuration options for the COBOL language within the Oak framework.

## ⚙️ Configuration

The `CobolLanguage` struct defines how the parser and lexer should behave to accommodate various COBOL standards (COBOL-68, 74, 85, 2002, 2014) and dialects (IBM, Micro Focus, GnuCOBOL):

```rust
pub struct CobolLanguage {}
```

Currently, `CobolLanguage` serves as a marker struct for COBOL support. Future versions may include configuration for:
- **`standard`**: Target COBOL standard (e.g., COBOL-85, COBOL-2014).
- **`format`**: Source format configuration (Fixed Format vs. Free Format).
- **`dialect`**: Specific dialect-dependent features and reserved words.

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering keywords, picture clauses, literals, and delimiters.
- **`ElementType`**: Defined in the `parser` module, representing divisions, sections, paragraphs, and verbs.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the COBOL source file.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of COBOL's unique syntax and hierarchical structure.
