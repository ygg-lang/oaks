# Valkyrie AST Builder

This module provides functionality to build strongly-typed Abstract Syntax Trees (AST) from the parsed syntax tree for the Valkyrie programming language.

## Overview

The builder module transforms the generic red-green tree structure produced by the parser into a strongly-typed AST that represents Valkyrie source code. This transformation provides:

- **Type Safety**: Each AST node has a specific type that corresponds to Valkyrie language constructs
- **Error Handling**: Comprehensive error reporting during AST construction
- **Incremental Building**: Support for incremental parsing and AST construction

## Key Components

### Builder Implementation

The `Builder<ValkyrieLanguage>` trait is implemented for `ValkyrieBuilder`, providing the main entry point for AST construction:

```rust,ignore
impl<'config> Builder<ValkyrieLanguage> for ValkyrieBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(
        &self,
        source: &S,
        edits: &[TextEdit],
        cache: &'a mut impl BuilderCache<ValkyrieLanguage>,
    ) -> BuildOutput<ValkyrieLanguage>;
}
```

### AST Construction Methods

The builder provides specialized methods for constructing different types of AST nodes:

- `build_root()` - Constructs the root AST node containing all top-level items
- `build_namespace()` - Builds namespace definitions
- `build_micro()` - Builds micro definitions (Valkyrie-specific constructs)
- `build_function()` - Constructs function definitions
- `build_expr()` - Builds various expression types
- `build_stmt()` - Constructs statement nodes

### Error Handling

The builder provides comprehensive error handling with detailed source location information:

- Syntax errors with precise location reporting
- Missing required elements detection
- Unexpected token/node validation

## Usage

The builder is typically used through the `ValkyrieBuilder` when parsing Valkyrie source code:

```rust
use oak_valkyrie::{ValkyrieBuilder, ValkyrieLanguage};
use oak_core::{Builder, parser::ParseSession, source::SourceText};

let language = ValkyrieLanguage::default();
let builder = ValkyrieBuilder::new(&language);
let source = SourceText::new("namespace MyNamespace { fn main() { let x = 42; } }");
let mut cache = ParseSession::default();
let result = builder.build(&source, &[], &mut cache);

match result.result {
    Ok(ast) => {
        // Work with the strongly-typed AST
        println!("Parsed {} items", ast.items.len());
    }
    Err(error) => {
        eprintln!("Parse error: {}", error);
    }
}
```

## Valkyrie Language Features

The builder supports all major Valkyrie language constructs:

- **Namespaces**: Organizational units for code
- **Micro Definitions**: Valkyrie-specific specialized constructs
- **Functions**: With parameters and block bodies
- **Expressions**: Identifiers, literals, binary/unary operations, calls, field access, indexing
- **Statements**: Variable bindings (`let`) and expression statements

## Architecture

The builder follows the red-green tree architecture:

1. **Green Tree**: Immutable, structural representation from the parser
2. **Red Tree**: Provides navigation and span information
3. **AST**: Strongly-typed, language-specific representation

This architecture enables efficient incremental parsing and provides excellent performance characteristics for IDE scenarios.