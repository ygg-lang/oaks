# TypeScript Language Definition

This module contains the metadata and configuration options for the TypeScript language within the Oak framework.

## ⚙️ Configuration

The `TypeScriptLanguage` struct defines how the parser and lexer should behave to accommodate various TypeScript versions and project-specific requirements:

- **`ts_version`**: Specifies the TypeScript version to target (e.g., 4.5, 5.0, 5.4+).
- **`enable_tsx`**: Enables support for parsing TSX syntax, common in modern React development.
- **`allow_experimental_decorators`**: Configures support for legacy experimental decorators vs. modern TC39 decorators.
- **`emit_decorator_metadata`**: Enables parsing logic that respects metadata-heavy decorator usage.
- **`strict_mode`**: When enabled, the parser enforces stricter TypeScript syntax rules during analysis.

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering all TS/JS keywords, operators, type-only tokens, and TSX tags.
- **`ElementType`**: Defined in the `parser` module, representing TS-specific syntax structures like interfaces, generics, enums, and TSX elements.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the TypeScript source file.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of TypeScript's complex type-level syntax and the modern web ecosystem.
