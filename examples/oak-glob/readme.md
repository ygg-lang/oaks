# oak-ignore

Ignore file language support for Oaks.

## Features

- Lexer for ignore file syntax
- Parser for ignore file syntax
- AST structure for ignore files
- LSP support
- Syntax highlighting support

## Usage

```rust
use oak_ignore::parse;

let ignore_content = "# This is a comment\n*.txt\n/build/";
let result = parse(ignore_content);

match result {
    Ok(root) => println!("Parsed successfully: {:?}", root),
    Err(e) => println!("Parse error: {}", e),
}
```
