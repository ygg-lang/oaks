# Vampire Abstract Syntax Tree (AST) Module

This module defines the abstract syntax tree structure for the Vampire theorem prover.

## AST Node Types

- **`VampireRoot`**: The root node, containing multiple formulas.
- **`VampireFormula`**: A formula, including name, role, and formula text.
- **`VampireInclude`**: An include directive used to import other files.

## Usage Example

```rust,no_run
#![feature(new_range_api)]
use oak_vampire::ast::*;
use core::range::Range;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = VampireRoot {
        span: Range { start: 0, end: 100 },
        formulas: vec![
            VampireFormula {
                span: Range { start: 0, end: 50 },
                name: "f1".to_string(),
                role: "conjecture".to_string(),
                formula: "p(a)".to_string(),
            }
        ],
    };
    
    Ok(())
}
```

## Design Principles

1. **Completeness**: Supports full Vampire syntax.
2. **Extensibility**: Easy to add new AST node types.
3. **Type Safety**: Uses Rust's type system to ensure AST validity.
4. **Performance**: Efficient memory usage and access patterns.
