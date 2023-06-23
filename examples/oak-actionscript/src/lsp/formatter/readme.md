# ActionScript Formatter

The `ActionScriptFormatter` is responsible for converting the ActionScript AST or source code into a beautifully formatted string, adhering to standard coding conventions.

## ✨ Features

- **AST-Based Formatting**: Can generate source code directly from a strongly-typed `ActionScriptRoot`.
- **Indentation Management**: Configurable indentation levels and strings (defaults to 4 spaces).
- **Structure Awareness**: Properly handles spacing between top-level items like classes, interfaces, and imports.

## 🚀 Usage

```rust
let formatter = ActionScriptFormatter::new();
let formatted_code = formatter.format_ast(&ast_root);
```
