# oak-asciidoc

High-performance incremental AsciiDoc parser for the oak ecosystem.

## Features

- Incremental parsing of AsciiDoc documents
- Support for headings, blocks, lists, tables, code blocks, and more
- Extensible architecture for custom AsciiDoc extensions
- Integration with the oak ecosystem for language services

## Usage

```rust
use oak_asciidoc::{AsciidocLexer, AsciidocParser, AsciidocLanguage};
use oak_core::{Lexer, Parser, Source, ParseSession};

let code = "= Heading\n\nParagraph with *emphasis* and **strong** text.";
let language = AsciidocLanguage::default();
let lexer = AsciidocLexer::new(&language);
let parser = AsciidocParser::new(&language);

// Tokenize and parse the code
let mut lex_cache = ();
let lex_output = lexer.lex(code, &[], &mut lex_cache);

let mut parse_cache = ParseSession::new();
let parse_output = parser.parse(code, &[], &mut parse_cache);
```

## Features Flags

- `serde`: Enables serialization/deserialization support
- `lsp`: Enables Language Server Protocol support
- `mcp`: Enables Multi-Compiler Protocol support

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
