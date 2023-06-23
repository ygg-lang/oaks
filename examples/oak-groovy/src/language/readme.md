# 🌐 Groovy Language Integration

This module defines the `GroovyLanguage` struct, which serves as the central configuration point for the Groovy parser within the Oak framework.

## 🛠️ Language Configuration

The `GroovyLanguage` struct implements the `Language` trait, providing metadata and type definitions required by the Oak ecosystem.

```rust
pub struct GroovyLanguage {
    // Future configuration options (e.g., Groovy version, dynamic vs static compilation)
}
```

## 🧩 Integration

`GroovyLanguage` links together the lexer and parser components:

- **TokenType**: `GroovyTokenType`
- **ElementType**: `GroovyElementType`
- **TypedRoot**: `()` (Future work will include typed root support)

## 🚀 Usage

```rust
use oak_groovy::GroovyLanguage;

let config = GroovyLanguage::new();
// Pass this config to GroovyParser::new(&config)
```
