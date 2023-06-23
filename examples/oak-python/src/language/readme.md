# Python Language Definition

This module contains the metadata and configuration options for the Python language within the Oak framework.

## ⚙️ Configuration

The `PythonLanguage` struct defines how the parser and lexer should behave to accommodate various Python versions and coding styles:

```rust
pub struct PythonLanguage {}
```

Currently, `PythonLanguage` serves as a marker struct for Python support. Future versions may include configuration for:
- **`version`**: Target Python version (e.g., 3.8, 3.10, 3.12).
- **`type_hints`**: Configures support for PEP 484 and subsequent type annotation standards.
- **`strict_indentation`**: When enabled, the parser strictly enforces consistent indentation.
- **`future_features`**: Enables support for upcoming Python features defined in `__future__` imports.

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering keywords, operators, indentation tokens, and f-strings.
- **`ElementType`**: Defined in the `parser` module, representing classes, functions, pattern matching, and decorators.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the Python source file.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of Python's unique indentation-based syntax and dynamic nature.
