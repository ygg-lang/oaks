# VOML Abstract Syntax Tree (AST) Module

This module defines the abstract syntax tree structure for the VOML language, used to represent parsed VOML code.

## AST Node Types

### Top-level Nodes

- **`VomlRoot`**: The root node of VOML source code, containing module name, imports, and item list.
- **`VomlItem`**: Top-level items, which can be structs, functions, enums, or constants.

### Type Definitions

- **`VomlStruct`**: Struct definition.
- **`VomlEnum`**: Enum definition.
- **`VomlField`**: Struct field.

### Functions

- **`VomlFunction`**: Function or method definition.
- **`VomlReceiver`**: Receiver for a method.
- **`VomlParam`**: Function parameter.

### Others

- **`VomlConst`**: Constant definition.

## Usage Example

```rust,ignore
use oak_voml::ast::*;

fn main() {
    let root = VomlRoot {
        module_name: "main".to_string(),
        imports: vec!["os".to_string()],
        items: vec![
            VomlItem::Function(VomlFunction {
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
