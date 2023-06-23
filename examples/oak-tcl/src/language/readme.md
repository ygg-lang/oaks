# Tcl Language Definition

This module contains the metadata and configuration options for the Tcl language within the Oak framework.

## ⚙️ Configuration

The `TclLanguage` struct defines how the parser and lexer should behave to accommodate various Tcl versions and project requirements:

```rust
pub struct TclLanguage {}
```

Currently, `TclLanguage` serves as a marker struct for Tcl support. Future versions may include configuration for:
- **`version`**: Target Tcl version (e.g., 8.6, 8.7, 9.0).
- **`commands`**: Custom command definitions for better semantic analysis.
- **`encoding`**: Source file encoding configuration.

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering commands, words, variables, and substitutions.
- **`ElementType`**: Defined in the `parser` module, representing commands, scripts, and expressions.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the Tcl source file.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of Tcl's unique command-centric syntax.
