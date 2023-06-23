# Rust Formatter Module

The `RustFormatter` module provides code formatting capabilities for Rust source code. It transforms Rust Abstract Syntax Trees (AST) into properly formatted source code following Rust's official style guidelines.

## Overview

This formatter is designed to:
- Convert Rust AST structures back into readable source code
- Apply consistent indentation and spacing
- Follow Rust community style conventions
- Handle various Rust language constructs including functions, structs, enums, traits, and more

## Features

### Core Functionality
- **AST to Code Conversion**: Transform parsed Rust AST back into formatted source code
- **Configurable Formatting**: Support for custom indentation and line length settings
- **Comprehensive Language Support**: Handle all major Rust constructs including:
  - Functions (with async, unsafe modifiers)
  - Structs and enums
  - Traits and impl blocks
  - Modules and use statements
  - Constants and static variables
  - Type aliases
  - Control flow statements (if, loop, while, for)
  - Expressions (binary, unary, function calls, field access)

### Formatting Rules
- **Indentation**: 4 spaces by default (configurable)
- **Line Length**: 100 characters maximum by default (configurable)
- **Spacing**: Consistent spacing around operators and keywords
- **Line Breaks**: Appropriate line breaks for better readability

## Usage

### Basic Usage

```rust,ignore
use oak_rust::formatter::RustFormatter;

let formatter = RustFormatter::new();
let formatted_code = formatter.format_ast(&ast_root);
```

### Custom Configuration

```rust,ignore
use oak_rust::formatter::RustFormatter;

let formatter = RustFormatter::with_config("    ".to_string(), 120);
let formatted_code = formatter.format_ast(&ast_root);
```

## Architecture

### Core Components

#### `RustFormatter` Struct
The main formatter struct that maintains formatting state:
- `indent_level`: Current indentation level
- `indent_str`: String used for indentation (default: 4 spaces)
- `max_line_length`: Maximum line length before wrapping

#### Key Methods

- `format_ast(&RustRoot) -> String`: Format an entire AST root node
- `format_item(&Item) -> String`: Format individual AST items
- `format_function(&Function) -> String`: Format function definitions
- `format_statement(&Statement) -> String`: Format statements
- `format_expression(&Expr) -> String`: Format expressions

### Formatting Process

1. **AST Traversal**: Recursively traverse the AST starting from the root
2. **Context-Aware Formatting**: Apply formatting rules based on the current context
3. **Indentation Management**: Track and apply appropriate indentation levels
4. **String Construction**: Build the formatted output string

## Supported Constructs

### Functions
- Regular functions, async functions, unsafe functions
- Parameters with types and mutability
- Return type annotations
- Function bodies with proper block formatting

### Variables and Bindings
- `let` statements with optional type annotations
- Mutable and immutable bindings
- Pattern matching in let statements

### Expressions
- Literals (strings, numbers, booleans)
- Identifiers and paths
- Binary and unary operations
- Function calls and method calls
- Field access and indexing
- Parenthesized expressions
- Code blocks

### Control Flow
- Return statements
- Break and continue statements
- Block expressions

### Type System
- Basic type formatting (placeholder implementation)
- Generic parameters (placeholder implementation)

## Limitations

### Current Limitations
- Some advanced Rust features are marked as placeholder implementations
- Type formatting is not fully implemented
- Pattern matching formatting is basic
- Macro formatting is not implemented
- Lifetime annotations are not handled

### Planned Improvements
- Complete type system formatting
- Advanced pattern matching
- Macro and attribute formatting
- Lifetime and trait bound formatting
- Better error handling and recovery

## Examples

### Function Formatting
```rust,ignore
fn main() {
    println!("Hello, world!");
}
```

### Struct Formatting
```rust,ignore
struct Point {
    x: i32,
    y: i32,
}
```

### Let Statement Formatting
```rust,ignore
let x = 42;
let mut name = String::from("Rust");
```

## Integration

This formatter is designed to work seamlessly with the Oak Rust parser:

```rust,ignore
use oak_rust::{parser::RustParser, formatter::RustFormatter};

let parser = RustParser::new();
let formatter = RustFormatter::new();

// Parse source code
let ast = parser.parse(source_code)?;

// Format back to source
let formatted = formatter.format_ast(&ast);
```

## Testing

The formatter includes comprehensive tests to ensure:
- Correct formatting of all supported constructs
- Preservation of semantic meaning
- Consistent output formatting
- Edge case handling

## Contributing

When contributing to the formatter:
1. Follow Rust's official style guidelines
2. Add comprehensive tests for new features
3. Ensure backward compatibility
4. Update documentation for new formatting rules

## License

This module is part of the Oak project and follows the same licensing terms.
