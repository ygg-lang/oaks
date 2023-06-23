# Lua Lexer

High-performance lexical analyzer for Lua source code.

## ⚡ Features

- **Full Lua 5.x Support**: Handles all keywords, operators, and literals.
- **Long Literals**: Efficiently parses `[[ ... ]]` strings and `--[[ ... ]]` comments, including arbitrary levels of `=` signs.
- **Numeric Precision**: Support for hexadecimal floats, standard scientific notation, and standard decimal/hex integers.
- **Trivia Preservation**: Correctly identifies whitespace and comments to support lossless AST construction.

## 🏗️ Implementation Details

The lexer is built for speed and reliability, using a state-machine approach to handle Lua's unique multi-line string and comment syntax without excessive backtracking. It produces a stream of `LuaTokenType` that is consumed by the parser.
