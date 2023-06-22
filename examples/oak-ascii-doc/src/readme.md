# `oak-ascii-doc`

This crate provides a parser for the AsciiDoc language, built using the `oaks` parsing framework. It includes a lexer and language definition to facilitate parsing AsciiDoc content.

## Usage

To use the `oak-ascii-doc` parser, you typically need to interact with `AsciiDocLanguage` and `AsciiDocLexer`.

### `AsciiDocLanguage`

The `AsciiDocLanguage` struct defines the grammar and rules for AsciiDoc. It implements the `Language` trait from the `oaks` framework.

```rust
use oak_ascii_doc::AsciiDocLanguage;

let language = AsciiDocLanguage::default();
```

### `AsciiDocLexer`

The `AsciiDocLexer` is responsible for tokenizing the input AsciiDoc content based on the `AsciiDocLanguage` definition.

```rust
use oak_ascii_doc::{AsciiDocLanguage, AsciiDocLexer};
use oak_core::{Lexer, source::SourceText, parser::session::ParseSession};

// Initialize the language
let language = Box::leak(Box::new(AsciiDocLanguage::default()));

// Create a lexer instance
let lexer = AsciiDocLexer::new(language);

// Prepare the input source code
let source_code = "= Document Title\n\nHello, *world*!";
let source = SourceText::new(source_code);
let mut session = ParseSession::default();

// Lex the input
let lex_output = lexer.lex(&source, &[], &mut session);

// You can now process the lex_output which contains the tokens
println!("Lexed tokens: {:?}", lex_output.result);
```

This example demonstrates how to initialize the `AsciiDocLanguage` and `AsciiDocLexer`, and then use the lexer to tokenize a simple AsciiDoc snippet. The `lex_output` will contain a list of `AsciiDocToken`s that represent the structure of the input content.