# Groovy Lexer Module

The Groovy Lexer module provides comprehensive lexical analysis for the [Groovy programming language](https://groovy-lang.org/). It converts Groovy source code into a stream of tokens, handling its dynamic and versatile syntax, including its close relationship with Java.

## Purpose

The primary objective of this module is to provide a fast and accurate tokenizer for Groovy. It handles Groovy's specific lexical features like optional semicolons, flexible string types (GStrings), and dynamic language constructs, serving as the first stage for the Groovy parser and analysis tools.

## Features

- **Java Compatibility**: Correctly identifies all Java keywords and operators, maintaining high compatibility with Java's lexical structure.
- **Dynamic Language Support**: Handles Groovy's unique keywords like `def`, `as`, `trait`, and `in`.
- **Advanced String Handling**:
    - **Single/Double Quoted Strings**: Supports both standard string types.
    - **GStrings**: Recognizes string interpolation syntax (`${expression}` and `$variable`).
    - **Slashy and Triple-Slashy Strings**: Correctly tokenizes regex-friendly and multi-line string formats.
- **Flexible Numeric Literals**: Parses various numeric formats, including those with Groovy-specific suffixes like `G` (BigInteger) and `D` (BigDecimal).
- **Comment Processing**: Supports single-line (`//`), multi-line (`/* ... */`), and Groovy's specialized shebang comment support (`#!`).
- **Operator Overloading Support**: Recognizes the extensive set of Groovy operators, including the spaceship operator (`<=>`), safe navigation (`?.`), and Elvis operator (`?:`).
- **Precise Span Information**: Each token includes its exact source location for accurate error reporting and IDE features.

## Token Types

### Keywords
- **Declarations**: `class`, `interface`, `enum`, `trait`, `def`, `as`, `import`, `package`.
- **Visibility & Modifiers**: `public`, `private`, `protected`, `static`, `final`, `abstract`, `synchronized`, `volatile`, `transient`, `native`, `strictfp`.
- **Control Flow**: `if`, `else`, `switch`, `case`, `default`, `while`, `do`, `for`, `in`, `break`, `continue`, `return`, `throw`, `try`, `catch`, `finally`, `assert`.
- **Types & Literals**: `int`, `long`, `short`, `byte`, `char`, `float`, `double`, `boolean`, `void`, `true`, `false`, `null`, `this`, `super`, `new`, `instanceof`.

### Literals
- **Numeric**: `123`, `123G`, `3.14D`, `0x7B`, `0b1111011`.
- **String**: `'Single'`, `"Double with ${interp}"`, `/slashy/`, `$/dollar-slashy/$`.
- **Boolean**: `true`, `false`.

### Operators & Symbols
- **Arithmetic**: `+`, `-`, `*`, `/`, `%`, `**`, `++`, `--`.
- **Comparison**: `==`, `!=`, `<`, `>`, `<=`, `>=`, `<=>`, `===`, `!==`.
- **Logical**: `&&`, `||`, `!`.
- **Assignment**: `=`, `+=`, `-=`, `*=`, `/=`, `%=`, `&=`, `|=`, `^=`, `<<=`, `>>=`, `>>>=`, `?=`.
- **Navigation & Access**: `.`, `?.`, `*.`, `&.`, `::`, `->`, `@`.
- **Structural**: `(`, `)`, `[`, `]`, `{`, `}`, `,`, `:`, `;`, `...`.

## Usage Example

```rust
use oak_groovy::lexer::GroovyLexer;

fn main() {
    let groovy_source = r#"
        def name = "Oak"
        println "Hello, ${name}!"
        
        def list = [1, 2, 3]
        list.each { println it }
    "#;

    let mut lexer = GroovyLexer::new();
    let tokens = lexer.tokenize(groovy_source);

    for token in tokens {
        println!("{:?}: '{}' at {:?}", token.token_type, token.lexeme, token.span);
    }
}
```

## Error Handling

The lexer identifies and reports:
- **Illegal Characters**: Characters not permitted in Groovy source files.
- **Unterminated Literals**: Unclosed strings, GStrings, or comments.
- **Invalid Numeric Formats**: Malformed numeric literals.
- **Source Context**: All errors include precise span information for accurate diagnostics.

## Design Principles

1. **Java Harmony**: Maintains a high degree of lexical compatibility with Java.
2. **Dynamic Flexibility**: Designed to handle Groovy's dynamic and expressive syntax.
3. **Performance**: Optimized for fast tokenization of large Groovy scripts and projects.
