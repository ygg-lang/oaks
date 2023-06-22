# Jasm Lexer Module

The Jasm Lexer module provides efficient lexical analysis for the [Jasm assembly language](https://github.com/openjdk/jasm), a modern assembly language for the Java Virtual Machine (JVM). It converts Jasm source code into a stream of tokens, enabling the assembly and analysis of JVM bytecode.

## Purpose

The primary objective of this module is to provide a fast and reliable tokenizer for Jasm. It handles the specific syntax of Jasm, including its support for modern JVM features like invokedynamic, records, and sealed classes, serving as the first stage for the Jasm parser.

## Features

- **JVM-Centric Tokenization**: Accurately identifies JVM-specific keywords, instructions, and descriptors.
- **Modern JVM Support**: Handles tokens for the latest JVM features, including nestmates, records, and sealed classes.
- **Efficient Descriptor Parsing**: Correctly tokenizes JVM type and method descriptors.
- **Comprehensive Literal Support**:
    - **Numeric**: Parses integer and floating-point literals, including those used in constant pool definitions.
    - **String**: Handles Java-style string literals with escape sequences.
- **Instruction Recognition**: Tokenizes all standard JVM instructions supported by Jasm.
- **Comment Handling**: Supports standard Java-style single-line (`//`) and multi-line (`/* ... */`) comments.
- **Precise Span Information**: Each token includes its exact source location for accurate error reporting and assembler diagnostics.

## Token Types

### Keywords & Directives
- **Structure**: `public`, `private`, `protected`, `static`, `final`, `class`, `interface`, `extends`, `implements`, `package`.
- **Method & Field**: `method`, `field`, `throws`, `signature`, `annotation`.
- **Special**: `version`, `stack`, `locals`, `catch`, `lookupswitch`, `tableswitch`.

### JVM Instructions
- **Stack**: `aload`, `astore`, `push`, `pop`, `dup`.
- **Arithmetic**: `iadd`, `fsub`, `imul`, `idiv`.
- **Control Flow**: `goto`, `if_icmpne`, `invokevirtual`, `invokestatic`, `invokedynamic`.
- **Objects**: `new`, `getfield`, `putstatic`, `checkcast`, `instanceof`.

### Literals & Identifiers
- **Numeric**: `123`, `3.14f`, `0x7B`.
- **String**: `"Ljava/lang/String;"`, `"Hello Jasm"`.
- **Identifiers**: `java/lang/Object`, `my_method`, `Lcom/example/MyClass;`.

### Operators & Symbols
- **Structural**: `{`, `}`, `(`, `)`, `[`, `]`, `,`, `:`, `;`, `=`, `->`.

## Usage Example

```rust
use oak_jasm::lexer::JasmLexer;

fn main() {
    let jasm_source = r#"
        public class HelloWorld {
            public static method main : ([Ljava/lang/String;)V {
                getstatic java/lang/System.out : Ljava/io/PrintStream;
                ldc "Hello, Jasm!"
                invokevirtual java/io/PrintStream.println : (Ljava/lang/String;)V
                return
            }
        }
    "#;

    let mut lexer = JasmLexer::new();
    let tokens = lexer.tokenize(jasm_source);

    for token in tokens {
        println!("{:?}: '{}' at {:?}", token.token_type, token.lexeme, token.span);
    }
}
```

## Error Handling

The lexer identifies and reports:
- **Illegal Characters**: Characters not permitted in Jasm source files.
- **Unterminated Literals**: Unclosed strings or comments.
- **Invalid Descriptors**: Malformed JVM type or method descriptors.
- **Source Context**: All errors include precise span information for accurate diagnostics.

## Design Principles

1. **JVM Accuracy**: Prioritizes correct representation of JVM bytecode structures.
2. **Performance**: Optimized for fast tokenization of large assembly files.
3. **Completeness**: Aims to support all Jasm language features and JVM instructions.
