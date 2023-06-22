# VOC Abstract Syntax Tree (AST) Module

This module defines the abstract syntax tree structure for the VOC language, used to represent parsed VOC code.

## AST Node Types

### Top-level Nodes

- **`VocRoot`**: The root node of VOC source code, containing module name, imports, and item list.
- **`VocItem`**: Top-level items, which can be structs, functions, enums, or constants.

### Type Definitions

- **`VocStruct`**: Struct definition.
- **`VocEnum`**: Enum definition.
- **`VocField`**: Struct field.

### Functions

- **`VocFunction`**: Function or method definition.
- **`VocReceiver`**: Receiver for a method.
- **`VocParam`**: Function parameter.

### Others

- **`VocConst`**: Constant definition.

## Usage Example

```rust,ignore
use oak_voc::ast::*;

fn main() {
    let root = VocRoot {
        module_name: "main".to_string(),
        imports: vec!["os".to_string()],
        items: vec![
            VocItem::Function(VocFunction {
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
