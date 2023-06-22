# Rust Parser

The Rust Parser module provides full Rust language syntax analysis functionality, converting the token stream produced by the lexer into an Abstract Syntax Tree (AST).

## Overview

This module implements an incremental parser based on the Oak framework, supporting the full syntax structure of the Rust language. The parser uses a Red-Green tree architecture, providing efficient incremental parsing and error recovery capabilities.

## Core Components

### RustElementType
Defines all AST node types for the Rust language:
- **Top-level Items**: `SourceFile`, `Module`, `Function`, `Struct`, `Enum`, `Trait`, `Impl`
- **Declarations**: `Use`, `Static`, `Const`, `TypeAlias`, `Macro`, `ExternCrate`, `ExternBlock`
- **Syntactic Structures**: `Block`, `ParameterList`, `StructBody`, `EnumBody`, `TraitBody`, `ImplBody`
- **Expressions**: `Expression`, `Statement`, `Pattern`, `Type`

### RustParser
Implements the `Parser<RustLanguage>` trait, providing:
- **Incremental Parsing**: Supports cache-based incremental parsing.
- **Error Recovery**: Capable of continuing parsing when encountering syntax errors.
- **Full Syntax Support**: Covers all syntax structures of the Rust language.

## Usage

```rust,ignore
use oak_rust::{RustParser, RustLanguage};
use oak_core::{IncrementalCache, source::StringSource};

let parser = RustParser::new();
let source = StringSource::new("fn main() { println!(\"Hello, world!\"); }");
let cache = IncrementalCache::<RustLanguage>::new();

let result = parser.parse_incremental(source, cache);
match result {
    Ok(ast) => {
        // Handle successfully parsed AST
        println!("Parsing successful: {:?}", ast);
    }
    Err(diagnostics) => {
        // Handle parsing errors
        for error in diagnostics.errors {
            println!("Syntax error: {}", error);
        }
    }
}
```

## Supported Rust Language Features

### Top-level Items
- **Function Definitions**: Functions defined with the `fn` keyword.
- **Structs**: Struct types defined with `struct`.
- **Enums**: Enum types defined with `enum`.
- **Traits**: Traits defined with `trait`.
- **Implementation Blocks**: `impl` blocks.
- **Modules**: `mod` module definitions.
- **Use Declarations**: `use` import declarations.
- **Constants**: `const` constant definitions.
- **Static Variables**: `static` static variable definitions.
- **Type Aliases**: `type` type alias definitions.

### Visibility Modifiers
- **Public Items**: Public items modified with `pub`.
- **Restricted Visibility**: `pub(crate)`, `pub(super)`, etc.

### Syntactic Structures
- **Parameter Lists**: Parsing of function parameters.
- **Code Blocks**: Code blocks enclosed in `{}`.
- **Struct Bodies**: Struct field definitions.
- **Enum Bodies**: Enum variant definitions.
- **Trait Bodies**: Trait method definitions.
- **Implementation Bodies**: Implementation method definitions.

## Error Handling

The parser provides a robust error handling mechanism:

### Syntax Error Detection
- **Missing Identifiers**: Detects missing function names, struct names, etc.
- **Mismatched Brackets**: Detects mismatched parentheses and braces.
- **Unexpected Tokens**: Detects tokens that do not follow syntax rules.

### Error Recovery
- **Skip Error Tokens**: Skips tokens on error and continues parsing.
- **Sync Point Recovery**: Performs error recovery at specific syntactic points.
- **Partial AST Construction**: Builds a partial AST even when errors are present.

## Performance Features

### Incremental Parsing
- **Caching Mechanism**: Utilizes previous parsing results.
- **Minimal Reparsing**: Only reparses modified parts.
- **Memory Efficiency**: Shares unchanged AST nodes.

### Red-Green Tree Architecture
- **Immutable Nodes**: Green nodes provide structure sharing.
- **Mutable Views**: Red nodes provide mutable AST views.
- **Efficient Updates**: Supports efficient AST modification operations.
