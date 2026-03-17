# Oak Stylus

Stylus language support for the Oak framework.

## Features

- Lexical analysis for Stylus syntax
- Syntax parsing for Stylus constructs
- Abstract syntax tree (AST) representation
- Integration with Oak core framework

## Usage

```rust
use oak_stylus::{StylusLanguage, StylusLexer, StylusParser, StylusBuilder};
use oak_core::{Lexer, Parser, Builder, Source, SourceText};

// Create a source text
let source = Source::new(SourceText::from("body { color: red; }".to_string()));

// Lex the source
let lexer = StylusLexer;
let lex_output = lexer.lex(source.text());

// Parse the tokens
let parser = StylusParser;
let parse_output = parser.parse(&mut lex_output.into_session());

// Build the syntax tree
let builder = StylusBuilder;
let green_tree = builder.build(&mut Default::default(), &Default::default());
```

## Implementation Status

- [x] Language trait implementation
- [x] Token types and roles
- [x] Element types and roles
- [x] Lexer implementation (basic)
- [x] Parser implementation (basic)
- [x] AST structure
- [x] Builder implementation (basic)
- [ ] Full lexer implementation
- [ ] Full parser implementation
- [ ] LSP support
- [ ] MCP support

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue.
