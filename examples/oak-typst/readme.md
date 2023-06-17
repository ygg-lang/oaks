# Oak Typst

A high-performance Typst parser built on the Oak framework.

## Features

- **Fast**: Incremental parsing with minimal re-parsing
- **Robust**: Error recovery and fault tolerance
- **Complete**: Full Typst syntax support
- **Flexible**: Configurable parsing options

## Usage

```rust
use oak_typst::{TypstLanguage, TypstLexer};
use oak_core::{parse, SourceText};

let source = SourceText::new("#let x = 42");
let language = TypstLanguage::standard();
let lexer = TypstLexer::new(&language);
let tree = parse(&source, &lexer);
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.