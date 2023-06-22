# VLang Lexer Module

The VLang Lexer module provides lexical analysis for the [V programming language](https://vlang.io/). It converts V source code into a stream of tokens, which is the first step in the compilation or analysis process.

## Purpose

This module is designed to be a fast and reliable lexer for V, supporting its modern syntax and features. It serves as the foundation for the V parser and other V-related tools in the Oak ecosystem.

## Features

- **Keyword Recognition**: Supports all V keywords, including `module`, `import`, `pub`, `mut`, `fn`, `struct`, `interface`, etc.
- **Modern Syntax Support**: Handles V's specific features like string interpolation (future), multiple return types, and more.
- **String Literals**: Supports both single-quote (`'`) and double-quote (`"`) strings, as well as raw strings.
- **Numeric Literals**: Correctly parses decimal, hexadecimal (`0x`), binary (`0b`), and octal (`0o`) numbers, including underscores for readability (e.g., `1_000_000`).
- **Comment Handling**: Supports line comments (`//`) and multi-line comments (`/* ... */`).
- **Fast and Lightweight**: Optimized for performance and low memory overhead.
- **Source Mapping**: Every token contains span information (start and end positions) for precise error reporting and IDE support.

## Token Types

### Keywords
- **Module & Imports**: `module`, `import`.
- **Visibility & Mutability**: `pub`, `mut`, `const`, `__global`.
- **Declarations**: `fn`, `struct`, `interface`, `enum`, `type`, `union`.
- **Control Flow**: `if`, `else`, `for`, `in`, `match`, `return`, `defer`, `go`, `select`.
- **Types**: `int`, `string`, `bool`, `f32`, `f64`, etc.

### Literals
- **Numeric**: `123`, `0x7B`, `0b1111011`, `3.14`.
- **String**: `'Hello V'`, `"Hello V"`, `r'raw string'`.
- **Boolean**: `true`, `false`.
- **Identifiers**: `main`, `my_var`, `UserStruct`.

### Operators and Symbols
- **Arithmetic**: `+`, `-`, `*`, `/`, `%`.
- **Assignment**: `=`, `:=`, `+=`, `-=`, `*=`, `/=`.
- **Comparison**: `==`, `!=`, `<`, `>`, `<=`, `>=`.
- **Logical**: `&&`, `||`, `!`.
- **Others**: `(`, `)`, `[`, `]`, `{`, `}`, `.`, `,`, `:`, `;`, `?`, `!`, `<-`.

## Usage Example

```rust
use oak_vlang::lexer::VLexer;

fn main() {
    let v_source = r#"
        module main
        import os

        pub fn main() {
            name := 'Oak'
            println('Hello, $name!')
        }
    "#;

    let mut lexer = VLexer::new();
    let tokens = lexer.tokenize(v_source);

    for token in tokens {
        println!("{:?}: '{}' at {:?}", token.token_type, token.lexeme, token.span);
    }
}
```

## Error Handling

The lexer detects and reports:
- **Illegal Characters**: Characters that are not valid in V source code.
- **Unterminated Strings**: Strings that don't have a matching closing quote.
- **Invalid Number Formats**: Malformed numeric literals (e.g., `0xG1`).
- **Unterminated Comments**: Multi-line comments that are never closed.

## Design Principles

1. **Simplicity**: Matches V's philosophy of being simple and easy to understand.
2. **Speed**: Built to handle large V codebases quickly.
3. **Accuracy**: Aims for 100% compatibility with the official V compiler's lexical rules.
