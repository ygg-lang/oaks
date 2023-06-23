# Objective-C Language Definition

This module contains the metadata and configuration options for the Objective-C language within the Oak framework.

## ⚙️ Configuration

The `ObjectiveCLanguage` struct defines how the parser and lexer should behave to accommodate different Objective-C versions and project requirements:

- **`objc_version`**: Specifies the Objective-C version to target (e.g., 1.0, 2.0).
- **`enable_arc`**: Configures support for Automatic Reference Counting (ARC) related keywords and patterns.
- **`allow_objc_cpp`**: Enables parsing of Objective-C++ files (`.mm`), integrating C++ syntax.
- **`strict_mode`**: When enabled, the parser enforces stricter Objective-C syntax rules during analysis.
- **`runtime_awareness`**: Configures how the parser identifies and handles Objective-C runtime-specific constructs.

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering all Objective-C keywords (`@`-prefixed), operators, literals, and message selector tokens.
- **`ElementType`**: Defined in the `parser` module, representing Objective-C specific syntax structures like interfaces, implementations, properties, protocols, and message expressions.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the Objective-C source file.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of Objective-C's unique syntax and the legacy Apple development ecosystem.
