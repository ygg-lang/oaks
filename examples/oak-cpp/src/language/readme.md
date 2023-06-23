# C++ Language Definition

This module contains the metadata and configuration options for the C++ language within the Oak framework.

## ⚙️ Configuration

The `CppLanguage` struct defines how the parser and lexer should behave to accommodate various C++ standards and complex language features:

```rust
pub struct CppLanguage {}
```

Currently, `CppLanguage` serves as a marker struct for C++ support. Future versions may include configuration for:
- **`standard`**: Target C++ standard (e.g., C++11, C++17, C++20, C++23).
- **`templates`**: Deep parsing support for template declarations and instantiations.
- **`stdlib`**: Configures how the parser identifies and handles standard library types and namespaces.
- **`extensions`**: Enables support for common compiler extensions (e.g., MSVC, GCC, Clang specific attributes).

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering keywords, identifiers, literals, and operators.
- **`ElementType`**: Defined in the `parser` module, representing classes, namespaces, templates, and functions.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the C++ translation unit.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of C++'s unique and complex syntax.
