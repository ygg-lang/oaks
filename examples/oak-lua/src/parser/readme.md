# Lua Parser

The core syntax analyzer for Lua, responsible for transforming a token stream into a Green/Red tree.

## 🏗️ Architecture

- **Recursive Descent**: Used for high-level structures like statements, blocks, and function definitions.
- **Pratt Parsing**: Employed for expression parsing to handle Lua's operator precedence and associativity (especially for `^` and `..`).
- **Incremental Engine**: Fully integrates with the Oak framework's incremental re-parsing logic.

## ✨ Capabilities

- **Fault Tolerance**: Capable of recovering from common syntax errors (e.g., missing `end`, malformed `if` conditions) to provide continuous diagnostics.
- **High Fidelity**: Every token, including whitespace and comments, is attached to the tree.
- **Standards Compliance**: Validated against Lua 5.1, 5.2, 5.3, and 5.4 syntax patterns.

## 🔍 Diagnostics

Produces precise error messages with line and column information, helping developers quickly identify and fix syntax issues in their Lua scripts.
