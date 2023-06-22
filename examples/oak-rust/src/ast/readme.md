# Rust Abstract Syntax Tree (AST) Module

This module defines the abstract syntax tree structure for the Rust language, used to represent parsed Rust code.
AST nodes correspond to various constructs in the Rust language, such as functions, structs, enums, modules, expressions, etc.

## AST Node Types

### Top-level Items

- **`Function`**: Function definition
- **`Struct`**: Struct definition
- **`Enum`**: Enum definition
- **`Module`**: Module definition
- **`UseItem`**: use statement
- **`Trait`**: trait definition
- **`Impl`**: impl block
- **`TypeAlias`**: Type alias
- **`Const`**: Constant definition
- **`Static`**: Static variable definition

### Type System

- **`Type`**: Type representation (paths, references, tuples, arrays, slices, function pointers)
- **`Identifier`**: Identifier
- **`Param`**: Function parameter
- **`Field`**: Struct field

### Statements and Expressions

- **`Statement`**: Statements (let, expression statements, return, break, continue)
- **`Expr`**: Expressions (identifiers, literals, binary operations, function calls, field access, control flow, etc.)
- **`Block`**: Code block
- **`Pattern`**: Pattern matching patterns

### Control Flow

- **`If`**: if expression
- **`While`**: while loop
- **`For`**: for loop
- **`Loop`**: loop
- **`Match`**: match expression
- **`MatchArm`**: match arm

## Usage Example

```rust,ignore
use oak_rust::ast::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a simple Rust program AST
    let root = RustRoot {
        items: vec![
            Item::Function(Function {
                name: Identifier {
                    name: "main".to_string(),
                    span: 0..4,
                },
                params: vec![],
                return_type: None,
                body: Block {
                    statements: vec![],
                    span: 5..7,
                },
                span: 0..7,
            })
        ],
    };
    
    println!("Created Rust AST with {} items", root.items.len());
    Ok(())
}
```

## Design Principles

1. **Completeness**: Supports full Rust syntax structures.
2. **Extensibility**: Easy to add new AST node types.
3. **Type Safety**: Uses Rust's type system to ensure AST validity.
4. **Performance**: Efficient memory usage and access patterns.
5. **Position Information**: Each node contains source code position information for easy error reporting and tool support.
