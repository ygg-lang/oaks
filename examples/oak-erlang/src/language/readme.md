# 🌐 Erlang Language Integration

This module defines the `ErlangLanguage` struct, which serves as the central configuration point for the Erlang parser within the Oak framework.

## 🛠️ Language Configuration

The `ErlangLanguage` struct implements the `Language` trait, providing metadata and type definitions required by the Oak ecosystem.

```rust
pub struct ErlangLanguage {
    // Future configuration options (e.g., Erlang version, OTP release compatibility)
}
```

## 🧩 Integration

`ErlangLanguage` links together the lexer and parser components:

- **TokenType**: `ErlangTokenType`
- **ElementType**: `ErlangElementType`
- **TypedRoot**: `()` (Future work will include typed root support)

## 🚀 Usage

```rust
use oak_erlang::ErlangLanguage;

let config = ErlangLanguage::new();
// Pass this config to ErlangParser::new(&config)
```
