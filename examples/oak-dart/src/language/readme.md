# 🌐 Dart Language Integration

This module defines the `DartLanguage` struct, which serves as the central configuration point for the Dart parser within the Oak framework.

## 🛠️ Language Configuration

The `DartLanguage` struct implements the `Language` trait, providing metadata and type definitions required by the Oak ecosystem.

```rust
pub struct DartLanguage {
    // Future configuration options (e.g., language version, lint rules)
}
```

## 🧩 Integration

`DartLanguage` links together the lexer and parser components:

- **TokenType**: `DartTokenType`
- **ElementType**: `DartElementType`
- **TypedRoot**: `DartRoot`

## 🚀 Usage

```rust
use oak_dart::DartLanguage;

let config = DartLanguage::new();
// Pass this config to DartParser::new(&config)
```
