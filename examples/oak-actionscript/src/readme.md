This crate provides a parser for the ActionScript language, built using the `oaks` parsing framework. It includes a lexer and language definition to facilitate parsing ActionScript code.

## Usage

To use the `oak-actionscript` parser, you typically need to interact with `ActionScriptLanguage` and `ActionScriptLexer`.

### `ActionScriptLanguage`

The `ActionScriptLanguage` struct defines the grammar and rules for ActionScript. It implements the `Language` trait from the `oaks` framework.

```rust
use oak_actionscript::ActionScriptLanguage;

let language = ActionScriptLanguage::default();
```

### `ActionScriptLexer`

The `ActionScriptLexer` is responsible for tokenizing the input ActionScript code based on the `ActionScriptLanguage` definition.

```rust
use oak_actionscript::{ActionScriptLanguage, ActionScriptLexer};
use oak_core::{Lexer, source::SourceText, parser::session::ParseSession};

// Initialize the language
let language = Box::leak(Box::new(ActionScriptLanguage::default()));

// Create a lexer instance
let lexer = ActionScriptLexer::new(language);

// Prepare the input source code
let source_code = "var x:int = 10;";
let source = SourceText::new(source_code);
let mut session = ParseSession::default();

// Lex the input
let lex_output = lexer.lex(&source, &[], &mut session);

// Access tokens via .result
println!("Lexed tokens: {:?}", lex_output.result);
```

This example demonstrates how to initialize the `ActionScriptLanguage` and `ActionScriptLexer`, and then use the lexer to tokenize a simple ActionScript code snippet. The `lex_output` will contain a list of `ActionScriptToken`s that represent the structure of the input code.