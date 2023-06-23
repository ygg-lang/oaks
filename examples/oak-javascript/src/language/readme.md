# JavaScript Language Definition

This module contains the metadata and configuration options for the JavaScript language within the Oak framework.

## ⚙️ Configuration

The `JavaScriptLanguage` struct defines how the parser and lexer should behave to accommodate various ECMAScript versions and environment-specific features:

- **`ecma_version`**: Specifies the ECMAScript version to target (e.g., ES5, ES2015, ES2022+).
- **`source_type`**: Configures whether the source should be parsed as a `Script` or a `Module` (ESM).
- **`enable_jsx`**: Enables support for parsing JSX syntax, common in React and other modern web frameworks.
- **`allow_experimental_features`**: Enables parsing of experimental or stage-proposal features (e.g., decorators, private class fields).
- **`strict_mode`**: When enabled, the parser enforces stricter ECMAScript rules (similar to `"use strict"`).

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering all JS keywords, operators, literals, and JSX tokens.
- **`ElementType`**: Defined in the `parser` module, representing JS-specific syntax structures like arrow functions, classes, and JSX elements.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the JavaScript source file.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of JavaScript's unique syntax and the evolving web ecosystem.
