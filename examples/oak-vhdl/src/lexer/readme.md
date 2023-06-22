# VHDL Lexer Module

The VHDL Lexer module provides high-performance lexical analysis for the VHDL (VHSIC Hardware Description Language) programming language. It is responsible for converting raw VHDL source text into a stream of meaningful tokens for further processing by the parser.

## Purpose

The primary goal of this module is to provide a robust and efficient way to tokenize VHDL source code, handling the complexities of hardware description language syntax, including case-insensitivity, specialized numeric literals, and various design unit structures.

## Features

- **Full Keyword Support**: Recognizes all standard VHDL keywords across different versions (87, 93, 2002, 2008, 2019).
- **Case Insensitivity**: Correctly handles VHDL's case-insensitive nature for keywords and identifiers.
- **Advanced Numeric Literals**: Supports decimal literals, based literals (e.g., `16#FF#`), and physical literals.
- **Identifier Handling**: Correctly parses basic identifiers and extended identifiers.
- **Comment Processing**: Supports standard line comments (`--`) and block comments (in newer versions).
- **Position Tracking**: Each token includes precise source code coordinates (line, column, offset) for accurate error reporting.

## Token Types

### Keywords
- **Design Units**: `entity`, `architecture`, `package`, `configuration`, `context`.
- **Declarations**: `port`, `generic`, `signal`, `variable`, `constant`, `type`, `subtype`, `component`.
- **Directional**: `in`, `out`, `inout`, `buffer`, `linkage`.
- **Control Flow**: `if`, `else`, `elsif`, `case`, `when`, `loop`, `for`, `while`, `generate`.

### Literals
- **Numeric**: Decimal (`123`, `1.5`, `1.0E-9`) and Based (`2#1010_1010#`, `16#DEAD_BEEF#`).
- **Character/String**: Character literals (`'A'`), String literals (`"hello"`), and Bit String literals (`x"FF"`, `b"1010"`).
- **Identifiers**: `clk`, `reset_n`, `\my extended identifier\`.

### Operators and Delimiters
- **Operators**: `+`, `-`, `*`, `/`, `**`, `abs`, `not`, `and`, `or`, `nand`, `nor`, `xor`, `xnor`.
- **Delimiters**: `(`, `)`, `,`, `;`, `:`, `:=`, `=>`, `<=`.

## Usage Example

```rust
use oak_vhdl::lexer::VhdlLexer;

fn main() {
    let vhdl_source = r#"
        entity counter is
            port (
                clk : in std_logic;
                count : out std_logic_vector(3 downto 0)
            );
        end entity;
    "#;

    let mut lexer = VhdlLexer::new();
    let tokens = lexer.tokenize(vhdl_source);

    for token in tokens {
        println!("{:?}: '{}' at {:?}", token.token_type, token.lexeme, token.span);
    }
}
```

## Error Handling

The lexer provides detailed error reporting for:
- **Invalid Characters**: Detects characters not allowed in VHDL source.
- **Malformed Literals**: Identifies incorrectly formatted numeric or bit-string literals.
- **Unterminated Strings**: Detects strings or extended identifiers that are not closed.

## Design Principles

1. **Efficiency**: Optimized for fast tokenization of large VHDL files.
2. **Standard Compliance**: Aims to strictly follow VHDL language specifications.
3. **Tool Friendly**: Designed to be used in IDEs, linters, and synthesis tool frontends.
