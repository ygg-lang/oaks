# `oak-ada`

This crate provides a parser for the Ada language, built using the `oaks` parsing framework. It includes a lexer and language definition to facilitate parsing Ada code.

## Usage

To use the `oak-ada` parser, you typically need to interact with `AdaLanguage` and `AdaLexer`.

### `AdaLanguage`

The `AdaLanguage` struct defines the grammar and rules for Ada. It implements the `Language` trait from the `oaks` framework.

```rust
use oak_ada::AdaLanguage;

let language = AdaLanguage::default();
```

### `AdaLexer`

The `AdaLexer` is responsible for tokenizing the input Ada code based on the `AdaLanguage` definition.

```rust
use oak_ada::{AdaLanguage, AdaLexer};
use oak_core::{Lexer, source::SourceText, parser::session::ParseSession};

// Initialize the language
let language = Box::leak(Box::new(AdaLanguage::default()));

// Create a lexer instance
let lexer = AdaLexer::new(language);

// Prepare the input source code
let source_code = "procedure Hello is begin Put_Line(\"Hello, world!\"); end Hello;";
let source = SourceText::new(source_code);
let mut session = ParseSession::default();

// Lex the input
let lex_output = lexer.lex(&source, &[], &mut session);

// You can now process the lex_output which contains the tokens
println!("Lexed tokens: {:?}", lex_output.result);
```

This example demonstrates how to initialize the `AdaLanguage` and `AdaLexer`, and then use the lexer to tokenize a simple Ada code snippet. The `lex_output` will contain a list of `AdaToken`s that represent the structure of the input code.