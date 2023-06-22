# `oak-bash`

This crate provides a parser for the Bash language, built using the `oaks` parsing framework. It includes a lexer and language definition to facilitate parsing Bash scripts.

## Usage

To use the `oak-bash` parser, you typically need to interact with `BashLanguage` and `BashLexer`.

### `BashLanguage`

The `BashLanguage` struct defines the grammar and rules for Bash. It implements the `Language` trait from the `oaks` framework.

```rust
use oak_bash::BashLanguage;

let language = BashLanguage::default();
```

### `BashLexer`

The `BashLexer` is responsible for tokenizing the input Bash code based on the `BashLanguage` definition.

```rust
use oak_bash::{BashLanguage, BashLexer};
use oak_core::{Lexer, source::SourceText, parser::session::ParseSession};

// Initialize the language
let language = Box::leak(Box::new(BashLanguage::default()));

// Create a lexer instance
let lexer = BashLexer::new(language);

// Prepare the input source code
let source_code = "#!/bin/bash\necho \"Hello, world!\"";
let source = SourceText::new(source_code);
let mut cache = ParseSession::default();

// Lex the input
let lex_output = lexer.lex(&source, &[], &mut cache);

// You can now process the lex_output which contains the tokens
println!("Lexed tokens: {:?}", lex_output.result);
```

This example demonstrates how to initialize the `BashLanguage` and `BashLexer`, and then use the lexer to tokenize a simple Bash script. The `lex_output` will contain a list of `BashToken`s that represent the structure of the input code.