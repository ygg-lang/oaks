# Dejavu Language Definition

This module contains the metadata and configuration options for the Dejavu language within the Oak framework.

## ⚙️ Configuration

The `DejavuLanguage` struct serves as the central definition for the Dejavu parser:

```rust
pub struct DejavuLanguage {
    pub dejavu_version: &'static str,
    pub enable_experimental_features: bool,
    pub strict_mode: bool,
}
```

- **`NAME`**: Identifies the language as "dejavu" across the Oak ecosystem.
- **`CATEGORY`**: Classified as `LanguageCategory::Programming`.
- **`TokenType`**: Maps to `DejavuTokenType`, covering all Dejavu keywords, operators, and literals.
- **`ElementType`**: Maps to `DejavuElementType`, representing structural components like namespaces, micro functions, and expressions.
- **`TypedRoot`**: Provides `DejavuRoot`, the strongly-typed entry point for AST analysis.

## 🧩 Oak Integration

By implementing the `Language` trait, this module enables:
1. **Generic Infrastructure**: Allows `oak-core` to handle file management and incremental logic without needing to know Dejavu-specific details.
2. **Type-Safe Analysis**: Ensures that any tool working with `oak-dejavu` has access to the correct token and element types at compile time.
3. **Multi-Version Support**: Future-proofed to handle different Dejavu versions through configuration updates within this module.
