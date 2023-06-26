# oak-rst

High-performance incremental reStructuredText parser for the oak ecosystem.

## Features

- Incremental parsing of reStructuredText documents
- Support for directives, substitutions, roles, footnotes, and citations
- Extensible architecture for custom reStructuredText extensions
- Integration with the oak ecosystem for language services

## Usage

```rust
use oak_rst::{RstLexer, RstParser, RstLanguage};
use oak_core::{ParseSession, SourceText, Parser};

let code = "# Heading\n\nParagraph with *emphasis* and **strong** text.";
let language = RstLanguage::default();
let lexer = RstLexer::new(&language);
let parser = RstParser::new(&language);

// Parse the code
let source = SourceText::new(code);
let mut session = ParseSession::new(16);
let result = parser.parse(&source, &[], &mut session);
```

## Features Flags

- `serde`: Enables serialization/deserialization support
- `lsp`: Enables Language Server Protocol support
- `mcp`: Enables Multi-Compiler Protocol support

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
