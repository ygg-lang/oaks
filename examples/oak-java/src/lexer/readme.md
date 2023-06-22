# Java Lexer Module

The Java Lexer module provides robust and high-performance lexical analysis for the [Java programming language](https://www.oracle.com/java/). It transforms Java source code into a stream of tokens, strictly following the Java Language Specification (JLS).

## Purpose

This module is designed to provide a foundational tokenizer for Java development tools. It handles the complexities of Java's lexical structure, including Unicode identifiers, various numeric literal formats, and the extensive set of Java keywords and operators.

## Features

- **JLS Compliance**: Strictly adheres to the latest Java Language Specification for tokenization.
- **Unicode Identifier Support**: Full support for Unicode characters in identifiers, as per the Java standard.
- **Comprehensive Literal Parsing**:
    - **Numeric**: Decimal, Hexadecimal (`0x`), Octal (`0`), and Binary (`0b`) integers, as well as floating-point literals with optional scientific notation and suffixes (`f`, `d`, `L`).
    - **Character & String**: Supports all Java escape sequences, including Unicode escapes (`\uXXXX`).
    - **Text Blocks**: (Future support) Handles Java's multi-line text blocks.
- **Comment Processing**: Correctly handles single-line (`//`), multi-line (`/* ... */`), and Javadoc (`/** ... */`) comments.
- **Operator & Delimiter Recognition**: Tokenizes all Java operators, including compound assignment and lambda operators (`->`).
- **Precise Span Tracking**: Each token includes its exact source location, enabling detailed error reporting and IDE features.

## Token Types

### Keywords
- **Declarations**: `class`, `interface`, `enum`, `record`, `extends`, `implements`, `package`, `import`.
- **Visibility & Modifiers**: `public`, `private`, `protected`, `static`, `final`, `abstract`, `synchronized`, `volatile`, `transient`, `native`, `strictfp`.
- **Control Flow**: `if`, `else`, `switch`, `case`, `default`, `while`, `do`, `for`, `break`, `continue`, `return`, `throw`, `try`, `catch`, `finally`, `assert`.
- **Types & Literals**: `int`, `long`, `short`, `byte`, `char`, `float`, `double`, `boolean`, `void`, `true`, `false`, `null`, `this`, `super`, `new`, `instanceof`.

### Literals
- **Numeric**: `123`, `0x7B`, `0b1111011`, `3.14f`, `1.0e-10d`.
- **String**: `"Hello Java"`, `"String with \n escapes"`.
- **Character**: `'A'`, `'\n'`, `'\u2615'`.

### Operators & Symbols
- **Arithmetic**: `+`, `-`, `*`, `/`, `%`, `++`, `--`.
- **Logical & Bitwise**: `&&`, `||`, `!`, `&`, `|`, `^`, `~`, `<<`, `>>`, `>>>`.
- **Comparison**: `==`, `!=`, `<`, `>`, `<=`, `>=`.
- **Assignment**: `=`, `+=`, `-=`, `*=`, `/=`, `%=`, `&=`, `|=`, `^=`, `<<=`, `>>=`, `>>>=`.
- **Lambda & Reference**: `->`, `::`.
- **Structural**: `(`, `)`, `[`, `]`, `{`, `}`, `,`, `.`, `:`, `;`, `@`, `...`.

## Usage Example

```rust
use oak_java::lexer::JavaLexer;

fn main() {
    let java_source = r#"
        package com.example;

        public class Main {
            public static void main(String[] args) {
                System.out.println("Hello, Java!");
            }
        }
    "#;

    let mut lexer = JavaLexer::new();
    let tokens = lexer.tokenize(java_source);

    for token in tokens {
        println!("{:?}: '{}' at {:?}", token.token_type, token.lexeme, token.span);
    }
}
```

## Error Handling

The lexer identifies and reports:
- **Illegal Characters**: Characters not permitted in Java source files.
- **Unterminated Literals**: Unclosed strings, characters, or comments.
- **Invalid Numeric Formats**: Malformed numeric literals (e.g., `0b12`).
- **Source Context**: All errors include precise span information for accurate diagnostics.

## Design Principles

1. **Strict Adherence**: Prioritizes 100% compatibility with the JLS.
2. **Performance**: Optimized for fast tokenization of large Java codebases.
3. **Robustness**: Designed to handle complex source structures and provide meaningful diagnostics.
