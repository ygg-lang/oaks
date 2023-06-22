# Wolfram Abstract Syntax Tree (AST) Module

This module defines the Abstract Syntax Tree (AST) structure for the Wolfram Language. It provides a strongly-typed representation of Wolfram expressions, which is essential for symbolic manipulation, evaluation, and analysis.

## Purpose

The Wolfram AST is designed to capture the rich and recursive nature of Wolfram Language expressions. It provides a structured way to represent everything from simple atoms to complex function applications and structural patterns.

## AST Node Types

### Core Nodes
- **`WolframRoot`**: The top-level node representing a complete Wolfram source file or a sequence of expressions.
- **`Expression`**: The fundamental unit of the Wolfram Language, which can be an atom or a composite expression.

### Atoms
- **`Symbol`**: Represents a named entity (e.g., `x`, `List`, `Plot`).
- **`Integer`**: A whole number.
- **`Real`**: A floating-point number.
- **`String`**: A sequence of characters.

### Composite Expressions
- **`FunctionCall`**: Represents an expression of the form `f[x, y, ...]` (e.g., `Sin[x]`).
- **`List`**: A collection of expressions enclosed in `{...}`.
- **`Association`**: A collection of key-value pairs `<| k -> v, ... |>`.
- **`BinaryExpression`**: An expression involving an infix operator (e.g., `a + b`, `x := 42`).
- **`UnaryExpression`**: An expression involving a prefix or postfix operator (e.g., `-x`, `n!`).

### Specialized Constructs
- **`Pattern`**: Represents Wolfram patterns (e.g., `x_`, `x__`, `x_Integer`).
- **`Rule`**: Represents rules (e.g., `x -> y`, `x :> y`).
- **`Part`**: Represents part access (e.g., `list[[1]]`).

## Usage Example

```rust
use oak_wolfram::ast::*;

fn main() {
    // Manually constructing a simple AST for: Plot[Sin[x], {x, 0, 2 Pi}]
    let root = WolframRoot {
        expressions: vec![
            Expression::FunctionCall(FunctionCall {
                head: Box::new(Expression::Symbol(Symbol { name: "Plot".to_string() })),
                arguments: vec![
                    Expression::FunctionCall(FunctionCall {
                        head: Box::new(Expression::Symbol(Symbol { name: "Sin".to_string() })),
                        arguments: vec![Expression::Symbol(Symbol { name: "x".to_string() })],
                    }),
                    Expression::List(List {
                        elements: vec![
                            Expression::Symbol(Symbol { name: "x".to_string() }),
                            Expression::Integer(0),
                            Expression::BinaryExpression(BinaryExpression {
                                operator: "*".to_string(),
                                left: Box::new(Expression::Integer(2)),
                                right: Box::new(Expression::Symbol(Symbol { name: "Pi".to_string() })),
                            }),
                        ],
                    }),
                ],
            })
        ],
    };
}
```

## Design Principles

1. **Symbolic Fidelity**: Accurately represents the symbolic nature of the Wolfram Language.
2. **Recursion Support**: Designed to handle deeply nested expression structures.
3. **Extensibility**: Easy to add support for new or specialized Wolfram constructs.
4. **Type Safety**: Leverages Rust's type system to ensure the structural integrity of the AST.
