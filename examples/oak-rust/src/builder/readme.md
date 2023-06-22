# Rust AST Builder

This module provides functionality to convert parsed syntax trees into strongly-typed Abstract Syntax Trees (ASTs), specifically for the Rust programming language.

## Overview

The builder module converts the generic Red-Green tree structures produced by the parser into strongly-typed ASTs representing Rust source code. This conversion provides:

- **Type Safety**: Each AST node has a specific type corresponding to a Rust language construct.
- **Error Handling**: Comprehensive error reporting during the AST construction process.
- **Incremental Building**: Supports incremental parsing and AST construction.

## Core Components

### Builder Implementation

The `Builder<RustLanguage>` trait is implemented for `RustBuilder`, providing the main entry point for AST construction:

```rust,ignore
impl<'config> Builder<RustLanguage> for RustBuilder<'config> {
    fn build_incremental(
        &self,
        text: impl Source,
        changed: usize,
        cache: IncrementalCache<RustLanguage>,
    ) -> OakDiagnostics<RustRoot>;
}
```

### AST Construction Methods

The builder provides specialized methods for constructing different types of AST nodes:

- `build_root()` - Builds the root AST node containing all top-level items.
- `build_function()` - Builds function definitions.
- `build_struct()` - Builds struct definitions.
- `build_enum()` - Builds enum definitions.
- `build_trait()` - Builds trait definitions.
- `build_impl()` - Builds impl blocks.
- `build_module()` - Builds module definitions.
- `build_expr()` - Builds various expression types.
- `build_stmt()` - Builds statement nodes.

### Error Handling

The builder provides comprehensive error handling with detailed source position information:

- Syntax errors with precise location reporting.
- Detection of missing required elements.
- Validation of unexpected tokens/nodes.

## Usage

The builder is typically used through `RustBuilder` during Rust source code parsing:

```rust,ignore
use oak_rust::{RustBuilder, RustLanguage};
use oak_core::Builder;

let language = RustLanguage::default();
let builder = RustBuilder::new(&language);
let source = "fn main() { let x = 42; println!(\"Hello, world!\"); }";
let result = builder.build_incremental(source, 0, Default::default());

match result.result {
    Ok(ast) => {
        // Use strongly-typed AST
        println!("Parsed {} items", ast.items.len());
    }
    Err(error) => {
        eprintln!("Parsing error: {}", error);
    }
}
```

## Rust Language Features

The builder supports all major Rust language constructs:

- **Functions**: Supports generics, parameters, return types, and bodies.
- **Structs**: Includes field definitions and generic parameters.
- **Enums**: Supports variants and associated data.
- **Traits**: Interface definitions and method signatures.
- **Impl Blocks**: Type implementations and trait implementations.
- **Modules**: Units of code organization.
- **Expressions**: Identifiers, literals, binary/unary operations, calls, field access, indexing, etc.
- **Statements**: Variable bindings (`let`), expression statements, etc.
- **Type System**: Primitive types, references, generics, lifetimes, etc.

## Architecture

The builder follows a Red-Green tree architecture:

1. **Green Tree**: Immutable structural representation from the parser.
2. **Red Tree**: Provides navigation and span information.
3. **AST**: Strongly-typed, language-specific representation.

This architecture supports efficient incremental parsing and provides excellent performance characteristics for IDE scenarios.

## Design Principles
