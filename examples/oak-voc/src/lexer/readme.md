# VOC Lexer Module

The VOC Lexer module provides lexical analysis for the VOC (V-Object-Compiler) language. It is responsible for transforming VOC source code into a stream of tokens, enabling further processing by the parser and other compiler tools.

## Purpose

The primary objective of this module is to provide a fast, accurate, and robust tokenizer for the VOC language. It handles the specific syntax rules of VOC, including its object-oriented features and modern syntax elements.

## Features

- **Full Keyword Support**: Recognizes all VOC keywords, including those for module management, visibility, and object-oriented constructs.
- **Identifier Handling**: Correctly parses identifiers, supporting Unicode and following VOC's naming conventions.
- **Numeric Literals**: Parses decimal, hexadecimal, octal, and binary literals, including support for underscores as digit separators.
- **String and Character Literals**: Supports single-quote and double-quote strings, including escape sequences and raw strings.
- **Comment Processing**: Efficiently handles both single-line (`//`) and multi-line (`/* ... */`) comments.
- **Span Information**: Each token includes its exact location in the source code (start and end positions), crucial for error reporting and IDE features.

## Token Types

### Keywords
- **Module & Imports**: `module`, `import`.
- **Visibility & Modifiers**: `pub`, `mut`, `const`, `__global`.
- **Declarations**: `fn`, `struct`, `interface`, `enum`, `type`, `union`.
- **Control Flow**: `if`, `else`, `for`, `in`, `match`, `return`, `defer`, `go`, `select`.
- **Built-in Types**: `int`, `string`, `bool`, `f32`, `f64`, `void`.

### Literals
- **Numeric**: `123`, `0xFF`, `0o755`, `0b1010`.
- **String**: `'Hello'`, `"World"`, `r'raw string'`.
- **Boolean**: `true`, `false`.
- **Identifiers**: `my_struct`, `calculate_sum`, `MainModule`.

### Operators and Symbols
- **Arithmetic**: `+`, `-`, `*`, `/`, `%`, `++`, `--`.
- **Assignment**: `=`, `:=`, `+=`, `-=`, `*=`, `/=`.
- **Comparison**: `==`, `!=`, `<`, `>`, `<=`, `>=`.
- **Logical**: `&&`, `||`, `!`.
- **Access & Navigation**: `.`, `?`, `->`, `::`.
- **Brackets & Punctuation**: `(`, `)`, `[`, `]`, `{`, `}`, `,`, `:`, `;`.

## Usage Example

```rust
use oak_voc::lexer::VocLexer;

fn main() {
    let voc_source = r#"
        module main
        import log

        pub fn main() {
            log.info('Starting VOC application...')
            x := 42
            println(x)
        }
    "#;

    let mut lexer = VocLexer::new();
    let tokens = lexer.tokenize(voc_source);

    for token in tokens {
        println!("{:?}: '{}' at {:?}", token.token_type, token.lexeme, token.span);
    }
}
```

## Error Handling

The lexer provides detailed error reporting:
- **Illegal Characters**: Detects characters not permitted in VOC source files.
- **Unterminated Literals**: Identifies strings or comments that are not properly closed.
- **Invalid Numeric Formats**: Catches malformed number literals.
- **Source Context**: Errors are accompanied by span information for precise highlighting.

## Design Principles

1. **Performance**: Optimized for high-speed tokenization.
2. **Robustness**: Designed to handle various edge cases and malformed input gracefully.
3. **Consistency**: Follows the lexical specifications of the VOC language strictly.
