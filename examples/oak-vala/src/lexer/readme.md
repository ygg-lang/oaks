# Vala Lexer Module

The Vala Lexer module provides robust lexical analysis for the [Vala programming language](https://vala.dev/). It transforms Vala source code into a stream of tokens, handling its C#-like syntax and integration with the GObject system.

## Purpose

This module is designed to provide a high-performance tokenizer for Vala, supporting its modern syntax features while being aware of its close relationship with C and the GLib/GObject ecosystem. It serves as the initial stage for the Vala parser and other static analysis tools.

## Features

- **Keyword Recognition**: Supports all Vala keywords, including GObject-specific ones like `signal`, `property`, and `construct`.
- **Identifier Handling**: Correctly parses Vala identifiers, including support for verbatim identifiers (e.g., `↯class`).
- **Comprehensive Literals**: Parses decimal, hexadecimal, octal, and binary numbers, as well as character and string literals (including verbatim and template strings).
- **Comment Processing**: Handles single-line (`//`) and multi-line (`/* ... */`) comments, as well as documentation comments (`/** ... */`).
- **Preprocessor Support**: Recognizes basic Vala preprocessor directives.
- **Precise Span Information**: Each token includes its exact source location (start and end offsets), enabling detailed error reporting and IDE integration.

## Token Types

### Keywords
- **Declarations**: `class`, `interface`, `struct`, `enum`, `namespace`, `using`, `delegate`, `signal`.
- **Visibility & Modifiers**: `public`, `private`, `protected`, `internal`, `static`, `virtual`, `abstract`, `override`, `async`, `yield`.
- **Control Flow**: `if`, `else`, `switch`, `case`, `default`, `for`, `foreach`, `while`, `do`, `break`, `continue`, `return`, `throw`, `try`, `catch`, `finally`.
- **Types**: `int`, `uint`, `string`, `bool`, `float`, `double`, `void`, `var`.

### Literals
- **Numeric**: `42`, `0x2A`, `0b101010`, `3.14f`.
- **String**: `"Hello Vala"`, `"""Verbatim string"""`, `↯"Template ${name}"`.
- **Boolean**: `true`, `false`.
- **Null**: `null`.

### Operators & Symbols
- **Arithmetic**: `+`, `-`, `*`, `/`, `%`, `++`, `--`.
- **Logical**: `&&`, `||`, `!`.
- **Comparison**: `==`, `!=`, `<`, `>`, `<=`, `>=`.
- **Assignment**: `=`, `+=`, `-=`, `*=`, `/=`, `:=`.
- **Navigation**: `.`, `->`, `?`, `??`.
- **Structural**: `(`, `)`, `[`, `]`, `{`, `}`, `,`, `:`, `;`.

## Usage Example

```rust
use oak_vala::lexer::ValaLexer;

fn main() {
    let vala_source = r#"
        using GLib;

        public class HelloWorld : Object {
            public static void main(string[] args) {
                stdout.printf("Hello, World!\n");
            }
        }
    "#;

    let mut lexer = ValaLexer::new();
    let tokens = lexer.tokenize(vala_source);

    for token in tokens {
        println!("{:?}: '{}' at {:?}", token.token_type, token.lexeme, token.span);
    }
}
```

## Error Handling

The lexer detects and reports:
- **Illegal Characters**: Characters that are not valid in Vala source code.
- **Unterminated Literals**: Unclosed strings, characters, or comments.
- **Invalid Numeric Formats**: Malformed numeric literals.
- **Source Context**: All errors include precise span information.

## Design Principles

1. **GObject Awareness**: Designed with Vala's GObject integration in mind.
2. **Performance**: Optimized for fast processing of large Vala projects.
3. **Accuracy**: Aims for full compatibility with the official Vala compiler's lexical rules.
