# Bash Language Definition

This module contains the metadata and configuration options for the Bash language within the Oak framework.

## ⚙️ Configuration

The `BashLanguage` struct defines how the parser and lexer should behave to accommodate various shell dialects and project requirements:

```rust
pub struct BashLanguage {}
```

Currently, `BashLanguage` serves as a marker struct for Bash support. Future versions may include configuration for:
- **`dialect`**: Target shell dialect (e.g., POSIX, Bash, Zsh, Sh).
- **`compat_mode`**: Enables specific compatibility behaviors for older shell versions.
- **`alias_expansion`**: Configures how shell aliases should be handled during parsing.

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering keywords, operators, redirections, and expansion tokens.
- **`ElementType`**: Defined in the `parser` module, representing commands, pipelines, loops, and function definitions.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the Bash script.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of Bash's flexible and powerful syntax.
