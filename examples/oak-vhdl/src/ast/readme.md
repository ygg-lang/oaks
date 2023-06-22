# VHDL Abstract Syntax Tree (AST) Module

This module defines the abstract syntax tree structure for the VHDL language, used to represent parsed VHDL code.

## AST Node Types

### Root Node

- **`VhdlRoot`**: The root node of a VHDL file, containing multiple design units.

### Design Units

- **`DesignUnit`**: A VHDL design unit, which can be an entity, architecture body, or package.
- **`EntityDeclaration`**: An entity declaration defining ports.
- **`ArchitectureBody`**: An architecture body defining the specific implementation of an entity.
- **`PackageDeclaration`**: A package declaration defining reusable types and functions.

### Ports and Signals

- **`PortDeclaration`**: Port declarations (in, out, inout, etc.).
- **`SignalDeclaration`**: Signal declarations.
- **`PortDirection`**: Port direction enumeration.

## Usage Example

```rust
use oak_vhdl::ast::*;

fn main() {
    // Create a simple VHDL entity AST
    let entity = EntityDeclaration {
        name: "counter".to_string(),
        ports: vec![
            PortDeclaration {
                name: "clk".to_string(),
                direction: PortDirection::In,
                data_type: "std_logic".to_string(),
            },
            PortDeclaration {
                name: "count".to_string(),
                direction: PortDirection::Out,
                data_type: "std_logic_vector(3 downto 0)".to_string(),
            },
        ],
    };
    
    let root = VhdlRoot {
        units: vec![DesignUnit::Entity(entity)],
    };
}
```

## Design Principles

1. **Completeness**: Supports full VHDL syntax.
2. **Extensibility**: Easy to add new AST node types.
3. **Type Safety**: Uses Rust's type system to ensure AST validity.
4. **Performance**: Efficient memory usage and access patterns.
