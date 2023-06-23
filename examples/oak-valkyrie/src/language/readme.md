# Valkyrie Language Definition

This module contains the metadata and configuration options for the Valkyrie language within the Oak framework.

## ⚙️ Configuration

The `ValkyrieLanguage` struct serves as the central definition for the Valkyrie parser:

```rust
pub struct ValkyrieLanguage {
    pub valkyrie_version: &'static str,
    pub enable_experimental_features: bool,
    pub strict_mode: bool,
}
```

- **`NAME`**: Identifies the language as "valkyrie" across the Oak ecosystem.
- **`CATEGORY`**: Classified as `LanguageCategory::Programming`.
- **`TokenType`**: Maps to `ValkyrieTokenType`, covering all Valkyrie keywords, operators, and literals.
- **`ElementType`**: Maps to `ValkyrieElementType`, representing structural components like namespaces, micro functions, and expressions.
- **`TypedRoot`**: Provides `ValkyrieRoot`, the strongly-typed entry point for AST analysis.

## 🧩 Oak Integration

By implementing the `Language` trait, this module enables:
1. **Generic Infrastructure**: Allows `oak-core` to handle file management and incremental logic without needing to know Valkyrie-specific details.
2. **Type-Safe Analysis**: Ensures that any tool working with `oak-valkyrie` has access to the correct token and element types at compile time.
3. **Multi-Version Support**: Future-proofed to handle different Valkyrie versions through configuration updates within this module.
