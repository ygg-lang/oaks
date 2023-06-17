# Oak MATLAB

High-performance incremental MATLAB parser for the oak ecosystem with flexible configuration, optimized for scientific computing and data analysis.

## Features

- **Fast Lexical Analysis**: Efficient tokenization of MATLAB source code
- **Comprehensive Token Support**: Full coverage of MATLAB syntax including:
  - Keywords (function, if, while, for, etc.)
  - Operators (arithmetic, logical, comparison)
  - Matrix operations (element-wise operations with dot notation)
  - Delimiters and punctuation
  - Comments (line and block comments)
  - String and character literals
  - Numeric literals (integers, floats, scientific notation)

- **Error Handling**: Robust error detection and recovery
- **No Standard Library**: `#![no_std]` compatible for embedded use
- **Incremental Parsing**: Designed for efficient re-parsing of modified code

## Supported MATLAB Syntax

### Keywords
- Control flow: `if`, `else`, `elseif`, `while`, `for`, `break`, `continue`, `return`
- Functions: `function`, `end`
- Classes: `classdef`, `properties`, `methods`, `events`
- Variables: `global`, `persistent`
- Error handling: `try`, `catch`
- Switch statements: `switch`, `case`, `otherwise`

### Operators
- Arithmetic: `+`, `-`, `*`, `/`, `^`, `\`
- Element-wise: `.*`, `./`, `.\`, `.^`
- Comparison: `==`, `~=`, `<`, `>`, `<=`, `>=`
- Logical: `&`, `|`, `~`, `&&`, `||`
- Assignment: `=`
- Transpose: `'`, `.'`

### Delimiters
- Parentheses: `(`, `)`
- Brackets: `[`, `]`
- Braces: `{`, `}`
- Punctuation: `;`, `,`, `.`, `:`, `?`, `@`

## Usage

```rust
use oak_core::SourceText;
use oak_matlab::{MatlabLanguage, MatlabLexer};

let language = MatlabLanguage {};
let lexer = MatlabLexer::new(&language);
let source = SourceText::new("function result = add(a, b)\n    result = a + b;\nend");

let output = lexer.lex(&source);
// Process the tokens...
```

## Testing

Run the test suite:

```bash
cargo test
```