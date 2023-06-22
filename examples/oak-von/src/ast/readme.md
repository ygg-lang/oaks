# VON Abstract Syntax Tree (AST) Module

This module defines the abstract syntax tree structure for the VON language, used to represent parsed VON code.

## AST Node Types

### Top-level Nodes

- **`VonRoot`**: The root node of VON source code, containing module name, imports, and item list.
- **`VonItem`**: Top-level items, which can be structs, functions, enums, or constants.

### Type Definitions

- **`VonStruct`**: Struct definition.
- **`VonEnum`**: Enum definition.
- **`VonField`**: Struct field.

### Functions

- **`VonFunction`**: Function or method definition.
- **`VonReceiver`**: Receiver for a method.
- **`VonParam`**: Function parameter.

### Others

- **`VonConst`**: Constant definition.

## Usage Example

```rust,ignore
use oak_von::ast::*;

fn main() {
    let root = VonRoot {
        module_name: "main".to_string(),
        imports: vec!["os".to_string()],
        items: vec![
            VonItem::Function(VonFunction {
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
