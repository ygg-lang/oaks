# VON Lexer Module

The VON Lexer module provides lexical analysis for the VON (V-Object-Notation) language. It transforms VON source code into a stream of tokens, enabling efficient data parsing and structure analysis.

## Purpose

VON is a lightweight data-interchange format designed for the V ecosystem. This lexer is built to be fast, reliable, and capable of handling complex nested structures and various data types defined in the VON specification.

## Features

- **Efficient Tokenization**: Converts raw text into a sequence of meaningful tokens with minimal overhead.
- **Data Type Support**: Recognizes and parses all VON data types, including objects, arrays, strings, numbers, and booleans.
- **Flexible Identifier Handling**: Supports identifiers for keys and property names, following V's naming rules.
- **Advanced Literal Parsing**: Correctly handles various numeric formats (integer, floating-point) and string escape sequences.
- **Comment Support**: Recognizes single-line (`//`) and multi-line (`/* ... */`) comments, allowing for documentation within VON data files.
- **Precise Span Information**: Each token includes start and end offsets, facilitating accurate error reporting and source mapping.

## Token Types

### Structure & Delimiters
- **Object Delimiters**: `{`, `}`.
- **Array Delimiters**: `[`, `]`.
- **Separators**: `:`, `,`.
- **Assignment**: `=`.

### Literals
- **Numeric**: `42`, `-123`, `3.14159`, `2.5e10`.
- **String**: `"Double quoted string"`, `'Single quoted string'`.
- **Boolean**: `true`, `false`.
- **Null Value**: `null`, `none`.

### Keywords (Metadata)
- **Schema & Versioning**: `version`, `schema`, `type`.
- **Property Modifiers**: `readonly`, `optional`.

### Identifiers
- **Keys**: `user_id`, `items`, `metadata`.

## Usage Example

```rust
use oak_von::lexer::VonLexer;

fn main() {
    let von_data = r#"
        {
            "project": "Oak",
            "version": 1.0,
            "features": ["parsing", "lexing", "ast"],
            "active": true
        }
    "#;

    let mut lexer = VonLexer::new();
    let tokens = lexer.tokenize(von_data);

    for token in tokens {
        println!("{:?}: '{}' at {:?}", token.token_type, token.lexeme, token.span);
    }
}
```

## Error Handling

The lexer provides clear error information for:
- **Invalid Syntax**: Characters that do not conform to the VON specification.
- **Unclosed Structures**: Unterminated strings, arrays, or objects.
- **Malformed Data**: Incorrectly formatted numbers or booleans.
- **Source Context**: All errors point to the exact location in the source text.

## Design Principles

1. **Lightweight**: Minimal dependencies and small footprint.
2. **Speed**: Optimized for fast processing of large data sets.
3. **Accuracy**: Strictly adheres to the VON language definition.
