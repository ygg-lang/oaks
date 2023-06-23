# Dejavu AST Builder

This module provides functionality to build strongly-typed Abstract Syntax Trees (AST) from the parsed syntax tree for the Dejavu programming language.

## Overview

The builder module transforms the generic red-green tree structure produced by the parser into a strongly-typed AST that represents Dejavu source code. This transformation provides:

- **Type Safety**: Each AST node has a specific type that corresponds to Dejavu language constructs
- **Error Handling**: Comprehensive error reporting during AST construction
- **Incremental Building**: Support for incremental parsing and AST construction

## Key Components

### Builder Implementation

The `Builder<DejavuLanguage>` trait is implemented for `DejavuBuilder`, providing the main entry point for AST construction:

```rust,ignore
impl<'config> Builder<DejavuLanguage> for DejavuBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(
        &self,
        source: &S,
        edits: &[TextEdit],
        cache: &'a mut impl BuilderCache<DejavuLanguage>,
    ) -> BuildOutput<DejavuLanguage>;
}
```

### AST Construction Methods

The builder provides specialized methods for constructing different types of AST nodes:

- `build_root()` - Constructs the root AST node containing all top-level items
- `build_namespace()` - Builds namespace definitions
- `build_micro()` - Builds micro function definitions
- `build_expr()` - Builds various expression types
- `build_stmt()` - Constructs statement nodes

### Error Handling

The builder provides comprehensive error handling with detailed source location information:

- Syntax errors with precise location reporting
- Missing required elements detection
- Unexpected token/node validation

## Usage

The builder is typically used through the `DejavuBuilder` when parsing Dejavu source code:

```rust
use oak_dejavu::{DejavuBuilder, DejavuLanguage};
use oak_core::{Builder, parser::ParseSession, source::SourceText};

let language = DejavuLanguage::default();
let builder = DejavuBuilder::new(&language);
let source = SourceText::new("namespace MyNamespace { micro main() { let x = 42; } }");
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

## Dejavu Language Features

The builder supports all major Dejavu language constructs:

- **Namespaces**: Organizational units for code
- **Micro Functions**: Functions with parameters and block bodies
- **Expressions**: Identifiers, literals, binary/unary operations, calls, field access, indexing
- **Statements**: Variable bindings (`let`) and expression statements

## Architecture

The builder follows the red-green tree architecture:

1. **Green Tree**: Immutable, structural representation from the parser
2. **Red Tree**: Provides navigation and span information
3. **AST**: Strongly-typed, language-specific representation

This architecture enables efficient incremental parsing and provides excellent performance characteristics for IDE scenarios.
