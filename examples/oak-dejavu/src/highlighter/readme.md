# Dejavu Syntax Highlighter

The Dejavu Syntax Highlighter provides comprehensive syntax highlighting capabilities for Dejavu source code, supporting both lexer-based and parser-based highlighting modes.

## Overview

The highlighter is designed to:
- Provide accurate syntax highlighting for Dejavu language constructs
- Support multiple highlighting modes (lexer-based and parser-based)
- Identify and highlight various code elements with appropriate styling
- Integrate seamlessly with editors and IDEs

## Key Components

### DejavuHighlighter

The main highlighter struct that implements the `Highlighter` trait:

- **highlight()** - Main highlighting method that processes Dejavu source code
- **highlight_keywords()** - Identifies and highlights Dejavu keywords
- **highlight_strings()** - Highlights string literals with proper escape handling
- **highlight_numbers()** - Highlights numeric literals (integers and floats)
- **highlight_comments()** - Highlights single-line and multi-line comments

## Supported Highlight Categories

### Keywords
- **Control Flow**: `if`, `else`, `while`, `for`, `return`, `break`, `continue`
- **Declarations**: `namespace`, `micro`, `let`
- **Literals**: `true`, `false`, `null`

### Literals
- **String Literals**: Quoted strings with escape sequence support
- **Numeric Literals**: Integer and floating-point numbers
- **Boolean Literals**: `true` and `false` values

### Comments
- **Single-line Comments**: Lines starting with `//`
- **Documentation Comments**: Special comment formats for documentation

### Identifiers
- **Function Names**: Function identifiers (in `micro` declarations and calls)
- **Variable Names**: Variable identifiers in declarations and usage
- **Namespace Names**: Namespace identifiers

## Usage Modes

### Lexer-based Highlighting (Default)
```rust
use oak_dejavu::highlighter::DejavuHighlighter;
use oak_highlight::{highlighter::Highlighter, themes::Theme};

let highlighter = DejavuHighlighter::new();
let highlights = highlighter.highlight("micro main() { let x = 42; }", "dejavu", Theme::Default);
```

### Parser-based Highlighting (Enhanced)
```rust
use oak_dejavu::highlighter::DejavuHighlighter;
use oak_highlight::{highlighter::Highlighter, themes::Theme};

let highlighter = DejavuHighlighter { use_parser: true };
let highlights = highlighter.highlight("namespace Test { micro PI = 3.14; }", "dejavu", Theme::Default);
```

## Highlight Kinds

The highlighter uses the following `HighlightKind` categories:
- `Keyword` - Language keywords and reserved words
- `String` - String literals and character sequences
- `Number` - Numeric literals (integers, floats)
- `Comment` - Comments and documentation
- `Identifier` - Variable and function names
- `Operator` - Operators and punctuation
- `Type` - Type annotations and declarations

## Integration

The highlighter integrates with the Oak framework's highlighting system and can be used in:
- Code editors and IDEs
- Syntax highlighting plugins
- Documentation generators
- Code analysis tools

## Future Enhancements

- Semantic highlighting based on AST analysis
- Context-aware highlighting for different scopes
- Customizable color schemes and themes
- Support for embedded languages and templates
