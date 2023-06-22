# Wolfram Lexer Module

The Wolfram Lexer module provides comprehensive lexical analysis for the Wolfram Language (Mathematica). It is responsible for converting complex Wolfram source code into a stream of tokens, handling its unique and extensive symbolic syntax.

## Purpose

The primary goal of this module is to accurately tokenize Wolfram Language expressions, which can range from simple arithmetic to highly complex symbolic representations. It handles specialized characters, long-name symbols, and various numeric forms unique to Wolfram.

## Features

- **Extensive Symbol Support**: Correctly identifies Wolfram symbols, including those with special characters and long names (e.g., `\[Alpha]`).
- **Complex Numeric Literals**: Parses standard integers, reals, and specialized forms like base-n numbers (e.g., `16^^FF`) and precision-specified numbers.
- **String Handling**: Supports Wolfram-style strings with various escape sequences and special characters.
- **Operator Recognition**: Tokenizes a vast array of Wolfram operators, including prefix, infix, and postfix forms.
- **Comment Processing**: Handles Wolfram comments enclosed in `(* ... *)`, including nested comments.
- **Whitespace Management**: Correctly handles whitespace, which can be significant in certain Wolfram constructs.
- **Precise Span Tracking**: Each token is associated with its exact location in the source code for accurate error reporting and IDE integration.

## Token Types

### Symbols & Identifiers
- **System Symbols**: `Plot`, `List`, `Table`, `Integrate`.
- **User Symbols**: `myVar`, `data123`.
- **Special Characters**: `\[Infinity]`, `\[DifferentialD]`.

### Literals
- **Numeric**: `123`, `3.14`, `16^^ABCD`, `123.456``20` (precision).
- **Strings**: `"Wolfram Language"`, `"String with \"escapes\""`.

### Operators & Delimiters
- **Arithmetic**: `+`, `-`, `*`, `/`, `^`.
- **Structural**: `{`, `}`, `[`, `]`, `(`, `)`, `[[`, `]]`.
- **Logical & Relational**: `&&`, `||`, `!`, `==`, `!=`, `<`, `>`, `<=`, `>=`.
- **Wolfram Specific**: `:=`, `->`, `:>`, `/.`, `//`, `&`, `#`, `##`.

## Usage Example

```rust
use oak_wolfram::lexer::WolframLexer;

fn main() {
    let wolfram_source = r#"
        Plot[Sin[x], {x, 0, 2 Pi}]
        data = {1, 2, 3, 4, 5}
        Total[data] (* Calculate sum *)
    "#;

    let mut lexer = WolframLexer::new();
    let tokens = lexer.tokenize(wolfram_source);

    for token in tokens {
        println!("{:?}: '{}' at {:?}", token.token_type, token.lexeme, token.span);
    }
}
```

## Error Handling

The lexer detects and reports:
- **Invalid Characters**: Characters not recognized in the Wolfram Language.
- **Unterminated Comments**: Comments that are not properly closed with `*)`.
- **Unterminated Strings**: Strings missing their closing quote.
- **Malformed Numbers**: Incorrectly formatted base-n or precision numbers.

## Design Principles

1. **Symbolic Accuracy**: Prioritizes correct identification of the vast symbolic set in Wolfram.
2. **Performance**: Optimized to handle large and complex symbolic expressions efficiently.
3. **Robustness**: Designed to be resilient to malformed input while providing clear diagnostics.
