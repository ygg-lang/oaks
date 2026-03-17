# 🔍 AsciiDoc Lexer

The AsciiDoc lexer is responsible for tokenizing AsciiDoc source code into meaningful tokens for further processing by the parser.

## ✨ Core Features

- **Comprehensive Tokenization**: Handles all AsciiDoc syntax elements including headings, lists, tables, and more.
- **Efficient Scanning**: Optimized for performance with minimal backtracking.
- **Error Recovery**: Gracefully handles malformed syntax and continues processing.
- **Configurable**: Supports customization through the AsciiDocLanguage configuration.

## 📋 Token Types

The lexer produces tokens for:
- Headings
- Lists
- Tables
- Code blocks
- Emphasis
- Images
- Attributes and macros
- Comments
- And many more AsciiDoc-specific elements

## 🔧 Usage

```rust
use oak_asciidoc::{language::AsciidocLanguage, lexer::AsciidocLexer};
use oak_core::source::StringSource;

let language = AsciidocLanguage::default();
let lexer = AsciidocLexer::new(&language);
let source = StringSource::new("# Hello World\n\nThis is AsciiDoc");

let output = lexer.lex_internal(&source);
for token in output.tokens {
    println!("{:?}", token);
}
```