# Dejavu Abstract Syntax Tree (AST) Module

This module defines the Abstract Syntax Tree (AST) structure for the Dejavu language. It provides a strongly-typed and comprehensive representation of Dejavu source code, facilitating compilation, analysis, and transformation.

## Purpose

The Dejavu AST is the central data structure used by the compiler and associated tools to represent the semantic structure of a Dejavu program. It is designed to be both expressive and efficient, supporting the language's modern features like algebraic data types, pattern matching, and functional programming constructs.

## AST Node Types

### Core Structure
- **`DejavuRoot`**: The root node representing a complete Dejavu source file or module.
- **`Module`**: A module definition containing declarations and imports.
- **`Import`**: Represents an import statement.

### Declarations
- **`MicroDefinition`**: Micro function definition with parameters, return type, and body.
- **`Class`**: Class definition with fields and methods.
- **`Namespace`**: Namespace definition for organizing code.
- **`Widget`**: UI widget definition.

### Expressions and Statements
- **`Expression`**: The base type for all expressions (literals, identifiers, function calls, etc.).
- **`Let`**: Local variable declaration.
- **`Match`**: Pattern matching construct.
- **`Block`**: A sequence of expressions/statements enclosed in braces.

### Patterns and Types
- **`Pattern`**: Patterns used in `match` and `let` bindings (e.g., `_`, `42`, variable names).

## Usage Example

```rust
use oak_dejavu::ast::*;

fn main() {
    // Constructing a simple AST for a Dejavu micro function
    // (Actual fields may vary based on current implementation)
}
```

## Design Principles

1. **Type Safety**: Uses Rust's enum and struct system to ensure only valid AST structures can be represented.
2. **Completeness**: Supports the full range of Dejavu language features.
3. **Rich Metadata**: Each node includes span information and can be extended with semantic information (like types) during analysis.
4. **Performance**: Optimized for memory efficiency and fast traversal.
