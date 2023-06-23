# ActionScript Parser

The `ActionScriptParser` is a sophisticated, incremental parser that transforms a token stream into a lossless Green Tree.

## 🧠 Parsing Strategy

The parser employs a combination of recursive descent and **Pratt Parsing** (Top-Down Operator Precedence) to handle the complexities of ActionScript syntax:

- **Recursive Descent**: Used for high-level structures like packages, classes, and functions.
- **Pratt Parsing**: Specifically used for expression parsing, allowing for elegant handling of operator precedence and associativity.

## ✨ Key Features

- **Incremental Parsing**: Only re-parses the portions of the file that have changed, ensuring extreme performance in IDE scenarios.
- **Error Recovery**: Implements advanced recovery strategies to produce a valid tree even in the presence of syntax errors.
- **Full Fidelity**: The resulting `GreenNode` tree preserves every token, including whitespace and comments (trivia).

## 📂 Submodules

- **`element_type`**: Defines the various types of syntax elements (nodes) in the ActionScript language.
- **`parse`**: Contains the core parsing logic for different language constructs.
