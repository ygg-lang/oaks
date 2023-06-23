# HLSL Language Definition

This module contains the metadata and configuration options for the HLSL (High-Level Shading Language) within the Oak framework.

## ⚙️ Configuration

The `HlslLanguage` struct serves as the central definition for the HLSL parser:

```rust
pub struct HlslLanguage {
    pub allow_comment: bool,
}
```

- **`NAME`**: Identifies the language as "hlsl" across the Oak ecosystem.
- **`CATEGORY`**: Classified as `LanguageCategory::Programming`.
- **`TokenType`**: Maps to `HlslTokenType`, covering all HLSL keywords, shader semantics, operators, and literals.
- **`ElementType`**: Maps to `HlslElementType`, representing structural components like shader stages, buffer declarations, and expressions.
- **`TypedRoot`**: Provides `HlslRoot`, the strongly-typed entry point for AST analysis.

## 🧩 Oak Integration

By implementing the `Language` trait, this module enables:
1. **Generic Infrastructure**: Allows `oak-core` to handle file management and incremental logic without needing to know HLSL-specific details.
2. **Type-Safe Analysis**: Ensures that any tool working with `oak-hlsl` has access to the correct token and element types at compile time.
3. **Multi-Version Support**: Future-proofed to handle different HLSL versions (e.g., Shader Model 5.0 vs 6.0) through configuration updates within this module.
