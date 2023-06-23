# Ruby Language Definition

This module contains the metadata and configuration options for the Ruby language within the Oak framework.

## ⚙️ Configuration

The `RubyLanguage` struct defines how the parser and lexer should behave to accommodate various Ruby versions and project requirements:

```rust
pub struct RubyLanguage {}
```

Currently, `RubyLanguage` serves as a marker struct for Ruby support. Future versions may include configuration for:
- **`version`**: Target Ruby version (e.g., 2.7, 3.0, 3.2+).
- **`dsl_optimizations`**: Configures internal heuristics for better parsing of common Ruby DSL patterns.
- **`experimental`**: Enables parsing of experimental syntax from recent Ruby versions.
- **`strict`**: When enabled, the parser enforces stricter Ruby syntax rules during analysis.

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering keywords, operators, literals, and interpolation tokens.
- **`ElementType`**: Defined in the `parser` module, representing classes, methods, blocks, and method calls.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the Ruby source file.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of Ruby's unique expressive power.
