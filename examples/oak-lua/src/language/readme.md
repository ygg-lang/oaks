# Lua Language Definition

This module contains the metadata and configuration options for the Lua language within the Oak framework.

## ⚙️ Configuration

The `LuaLanguage` struct defines how the parser and lexer should behave to accommodate various Lua versions and project requirements:

```rust
pub struct LuaLanguage {}
```

Currently, `LuaLanguage` serves as a marker struct for Lua support. Future versions may include configuration for:
- **`version`**: Target Lua version (e.g., 5.1, 5.2, 5.3, 5.4, Luau).
- **`compat_mode`**: Enables specific compatibility behaviors for older Lua versions.
- **`jit_extensions`**: Enables support for LuaJIT-specific extensions (e.g., FFI, 64-bit integers).

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering keywords, operators, table constructors, and long string tokens.
- **`ElementType`**: Defined in the `parser` module, representing functions, table assignments, loops, and variable declarations.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the Lua source file.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of Lua's lightweight and expressive syntax.
