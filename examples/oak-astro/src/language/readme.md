# Svelte Language Definition

This module contains the metadata and configuration options for the Svelte language within the Oak framework.

## ⚙️ Configuration

The `SvelteLanguage` struct defines how the parser and lexer should behave to accommodate various Svelte versions and project requirements:

```rust
pub struct SvelteLanguage {}
```

Currently, `SvelteLanguage` serves as a marker struct for Svelte support. Future versions may include configuration for:
- **`svelte_version`**: Specific Svelte version targeting (e.g., 3, 4, or 5).
- **`experimental_features`**: Enables support for experimental Svelte features.

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering HTML tags, JS/TS tokens, and Svelte structural tokens (control blocks, snippets).
- **`ElementType`**: Defined in the `parser` module, representing template elements, logic blocks, and directives.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the Svelte component.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of Svelte's unique component syntax.
