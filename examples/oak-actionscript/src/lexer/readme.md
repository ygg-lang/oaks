# ActionScript Lexer

The `ActionScriptLexer` is a high-performance scanner that converts raw ActionScript source code into a stream of tokens.

## ✨ Features

- **Incremental Lexing**: Efficiently updates the token stream when source code is edited.
- **Full Trivia Support**: Captures whitespace and comments, ensuring the resulting syntax tree is lossless.
- **Robust Error Handling**: Gracefully handles unexpected characters and malformed literals.

## 🔍 Token Categories

The lexer recognizes a wide range of ActionScript tokens, including:
- **Keywords**: `class`, `function`, `var`, `public`, `private`, etc.
- **Literals**: String, Numeric (integer and hex), and Character literals.
- **Operators**: Arithmetic, logical, bitwise, and assignment operators.
- **Delimiters**: Braces, parentheses, brackets, and semicolons.

## 🛠️ Implementation Details

The lexer is built using the Oak `Lexer` framework, leveraging specialized configurations for:
- **Comments**: Supports both single-line (`//`) and multi-line (`/* */`) comments.
- **Strings**: Handles double-quoted strings with escape sequences.
- **Whitespace**: Recognizes Unicode-compliant whitespace characters.
