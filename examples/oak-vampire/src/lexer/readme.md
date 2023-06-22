# Vampire Lexer Module

Vampire Lexer for tokenizing Vampire logic problems.

## Features

- **Keyword Recognition**: Identifies Vampire keywords (`fof`, `cnf`, `axiom`, `conjecture`, etc.)
- **Identifier Parsing**: Handles identifiers and names
- **Numeric Literals**: Parses numeric values
- **Logical Connectives**: Recognizes logical operators like `&`, `|`, `~`, `=>`, `<=>`
- **Quantifiers**: Supports `! [X] :` and `? [X] :`
- **Comments**: Supports line comments and block comments

## Token Types

### Keywords
- Top-level: `fof`, `cnf`, `tff`, `thf`, `include`
- Roles: `axiom`, `conjecture`, `hypothesis`, `lemma`, `theorem`, `definition`

### Literals and Identifiers
- Numeric: `123`, `0.5`, `1.2e-3`
- Identifiers: `socrates`, `human`, `X`, `A`
- Strings: `'single quoted'`, `"double quoted"`

## Usage Example

```rust,no_run
use oak_vampire::{VampireLexer, VampireLanguage};
use oak_core::{Lexer, source::SourceText, parser::session::ParseSession};

let language = VampireLanguage::default();
let lexer = VampireLexer::new(&language);
let vampire_code = "fof(ax1, axiom, ( human(socrates) )).";
    
let source = SourceText::new(vampire_code);
let mut session = ParseSession::<VampireLanguage>::default();
let output = lexer.lex(&source, &[], &mut session);

println!("Tokens: {}", output.result.map_or(0, |tokens| tokens.len()));
```

## Error Handling

The lexer provides detailed diagnostics:
- Illegal characters
- Unterminated strings
- Invalid numeric formats
- Position information tracking
