# Lua Abstract Syntax Tree (AST)

This module defines the high-fidelity, strongly-typed AST for Lua. Built on the Green/Red tree architecture, it provides a lossless representation of the source code.

## 🌳 Core Structures

- **`LuaRoot`**: The entry point of the AST, representing a full Lua chunk or script.
- **`Statement`**: Enum covering all Lua statements (Local, Assignment, Call, If, For, etc.).
- **`Expression`**: Enum for all expression types (TableConstructor, FunctionDefinition, Binary, etc.).
- **`TableField`**: Represents entries in a table constructor, handling both list-style and record-style fields.

## ✨ Key Features

1. **Lossless**: Captures all comments and whitespace, making it ideal for formatters and refactoring tools.
2. **Type-Safe**: Provides a convenient API to navigate the tree without manual type casting.
3. **Incremental Ready**: Designed to work with the Oak framework's partial tree updates.

## 🔍 Usage

You can traverse the tree using the `TypedNode` traits:
```rust
let root = result.typed_root::<LuaRoot>();
for stmt in root.statements() {
    match stmt {
        Statement::Local(l) => println!("Found local: {:?}", l.names()),
        _ => {}
    }
}
```
