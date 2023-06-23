# 🌐 Perl Language Integration

This module defines the `PerlLanguage` struct, which serves as the central configuration point for the Perl parser within the Oak framework.

## 🛠️ Language Configuration

The `PerlLanguage` struct implements the `Language` trait, providing metadata and type definitions required by the Oak ecosystem.

```rust
pub struct PerlLanguage {
    // Future configuration options (e.g., Perl 5 vs Perl 6/Raku, specific module pragmas)
}
```

## 🧩 Integration

`PerlLanguage` links together the lexer and parser components:

- **TokenType**: `PerlTokenType`
- **ElementType**: `PerlElementType`
- **TypedRoot**: `PerlRoot`

## 🚀 Usage

```rust
use oak_perl::PerlLanguage;

let config = PerlLanguage::new();
// Pass this config to PerlParser::new(&config)
```
