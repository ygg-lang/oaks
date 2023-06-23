# Kotlin Language Definition

This module contains the metadata and configuration options for the Kotlin language within the Oak framework.

## ⚙️ Configuration

The `KotlinLanguage` struct defines how the parser and lexer should behave to accommodate various Kotlin versions and project requirements:

```rust
pub struct KotlinLanguage {}
```

Currently, `KotlinLanguage` serves as a marker struct for Kotlin support. Future versions may include configuration for:
- **`version`**: Target Kotlin version (e.g., 1.8, 1.9, 2.0+).
- **`context_receivers`**: Enables parsing of experimental context receivers syntax.
- **`kmp_support`**: Enables support for Kotlin Multiplatform keywords like `expect` and `actual`.
- **`strict`**: When enabled, the parser enforces stricter Kotlin syntax rules during analysis.

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering keywords, operators, literals, and string template tokens.
- **`ElementType`**: Defined in the `parser` module, representing data classes, lambdas, generics, and coroutines.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the Kotlin source file.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of Kotlin's sophisticated type system and its multiplatform capabilities.
