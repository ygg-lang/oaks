# VLang Abstract Syntax Tree (AST) Module

This module defines the abstract syntax tree structure for the V language, used to represent parsed V code.

## AST Node Types

### Top-level Nodes

- **`VRoot`**: The root node of V source code, containing module name, imports, and item list.
- **`VItem`**: Top-level items, which can be structs, functions, enums, or constants.

### Type Definitions

- **`VStruct`**: Struct definition.
- **`VEnum`**: Enum definition.
- **`VField`**: Struct field.

### Functions

- **`VFunction`**: Function or method definition.
- **`VReceiver`**: Receiver for a method.
- **`VParam`**: Function parameter.

### Others

- **`VConst`**: Constant definition.

## Usage Example

```rust,ignore
use oak_vlang::ast::*;

fn main() {
    let root = VRoot {
        module_name: "main".to_string(),
        imports: vec!["os".to_string()],
        items: vec![
            VItem::Function(VFunction {
                name: "main".to_string(),
                is_pub: true,
                receiver: None,
                params: vec![],
                return_type: None,
                body: vec!["println('hello world')".to_string()],
            }),
        ],
    };
}
```
