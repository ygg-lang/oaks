# ActionScript Language Definition

This module contains the metadata and configuration options for the ActionScript 3.0 language within the Oak framework.

## ⚙️ Configuration

The `ActionScriptLanguage` struct defines how the parser and lexer should behave to accommodate different AS3/Flex development environments:

- **`strict_mode`**: When enabled, the parser enforces stricter ActionScript 3.0 rules and type checking, aligned with the modern MXMLC compiler.
- **`as3_features`**: Enables specific ActionScript 3.0 features like packages, namespaces, and modern class syntax.
- **`enable_e4x`**: Configures support for ECMAScript for XML (E4X) extensions, allowing the parser to handle inline XML literals and operators.
- **`metadata_awareness`**: Enables the parser to correctly identify and structured metadata tags (e.g., `[Event]`, `[Bindable]`, `[Style]`).

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering all AS3 keywords, operators, and literals.
- **`ElementType`**: Defined in the `parser` module, representing AS3-specific syntax structures like classes, packages, and E4X expressions.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the ActionScript source file.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of ActionScript's unique syntax and legacy requirements.
