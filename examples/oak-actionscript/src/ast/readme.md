# ActionScript AST (Abstract Syntax Tree)

This module defines the high-level, strongly-typed Abstract Syntax Tree (AST) for the ActionScript language.

## 🌳 Structure

The AST is designed to provide a developer-friendly representation of ActionScript source code, mapping the raw syntax elements into meaningful Rust structures and enums.

### Key Components

- **`ActionScriptRoot`**: The entry point of the AST, containing all top-level items in a source file.
- **`ActionScriptItem`**: An enum representing various top-level declarations:
    - `Package`: Package declarations.
    - `Class`: Class definitions.
    - `Interface`: Interface definitions.
    - `Function`: Global or class-level function definitions.
    - `Variable`: Variable and constant declarations.
    - `Import`: Import statements.

## 🚀 Usage

The AST is typically produced by the `ActionScriptBuilder`, which traverses the low-level Green/Red trees and constructs these high-level types. It serves as the primary input for higher-level analysis, refactoring, and code generation tools.
