# Less Builder Module

This module provides a builder for constructing Less Abstract Syntax Trees (ASTs) from parsed green trees.

## Builder Functionality

The `LessBuilder` struct implements the `Builder` trait from `oak_core` and provides the following functionality:

- **AST Construction**: Builds ASTs from parsed green trees
- **Error Handling**: Propagates parsing errors and diagnostics
- **Source Text Handling**: Processes source text and edits

## Usage

```rust
use oak_less::{LessBuilder, LessLanguage};
use oak_core::{SourceText, TextEdit};

// Create a new builder with default configuration
let config = LessLanguage::default();
let builder = LessBuilder::new(config);

// Build AST from source text
let source = SourceText::new("div { color: red; }".to_string());
let edits: Vec<TextEdit> = vec![];
let mut cache = oak_core::builder::DefaultBuilderCache::default();

let result = builder.build(&source, &edits, &mut cache);

match result.result {
    Ok(ast) => println!("AST built successfully: {:?}", ast),
    Err(error) => println!("Error building AST: {:?}", error),
}
```

## Features

- **Builder Trait Implementation**: Follows the standard Oak builder pattern
- **Integration with Parser**: Works seamlessly with the Less parser
- **Cache Support**: Uses builder cache for improved performance
- **Diagnostics**: Propagates parsing diagnostics to the caller