# 🌐 Elixir Language Integration

This module defines the `ElixirLanguage` struct, which serves as the central configuration point for the Elixir parser within the Oak framework.

## 🛠️ Language Configuration

The `ElixirLanguage` struct implements the `Language` trait, providing metadata and type definitions required by the Oak ecosystem.

```rust
pub struct ElixirLanguage {
    // Future configuration options (e.g., Elixir version, compiler options)
}
```

## 🧩 Integration

`ElixirLanguage` links together the lexer and parser components:

- **TokenType**: `ElixirTokenType`
- **ElementType**: `ElixirElementType`
- **TypedRoot**: `()` (Future work will include typed root support)

## 🚀 Usage

```rust
use oak_elixir::ElixirLanguage;

let config = ElixirLanguage::new();
// Pass this config to ElixirParser::new(&config)
```
