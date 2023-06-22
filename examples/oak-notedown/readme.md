# Oak Notedown Parser

[![Crates.io](https://img.shields.io/crates/v/oak-notedown.svg)](https://crates.io/crates/oak-notedown)
[![Documentation](https://docs.rs/oak-notedown/badge.svg)](https://docs.rs/oak-notedown)

High-performance incremental Notedown parser for the oak ecosystem with flexible configuration, optimized for document processing and rendering.

## 🎯 Overview

Oak Notedown is a robust parser for Notedown, designed to handle complete Notedown syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for document processing and rendering.

## ✨ Features

- **Complete Notedown Syntax**: Supports all Notedown features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_notedown::{NotedownLexer, NotedownLanguage};
use oak_core::{Lexer, Source};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let language = NotedownLanguage::default();
    let lexer = NotedownLexer::new(&language);
    let source = Source::new(r#"
# Hello, Notedown!

This is a **paragraph** with *emphasis*.

## Features

- Lists
- Code blocks
- And more!
    "#);
    
    let result = lexer.tokenize(&source);
    println!("Parsed Notedown successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Document Parsing
```rust
use oak_notedown::{NotedownLexer, NotedownLanguage};
use oak_core::{Lexer, Source};

let language = NotedownLanguage::default();
let lexer = NotedownLexer::new(&language);
let source = Source::new(r#"
# My Document

This is a simple document.
"#);

let result = lexer.tokenize(&source);
println!("Document parsed successfully.");
```

### Heading Parsing
```rust
use oak_notedown::{NotedownLexer, NotedownLanguage};
use oak_core::{Lexer, Source};

let language = NotedownLanguage::default();
let lexer = NotedownLexer::new(&language);
let source = Source::new(r#"
## My Heading

Some content here.
"#);

let result = lexer.tokenize(&source);
println!("Heading parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_notedown::{NotedownLexer, NotedownLanguage};
use oak_core::{Lexer, Source};

let language = NotedownLanguage::default();
let lexer = NotedownLexer::new(&language);
let source = Source::new("# Heading\n\nParagraph text");
let result = lexer.tokenize(&source);
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_notedown::{NotedownLexer, NotedownLanguage};
use oak_core::{Lexer, Source};

let language = NotedownLanguage::default();
let lexer = NotedownLexer::new(&language);
let source = Source::new(r#"
# Heading

This is a paragraph
## Another heading
# Unclosed heading
"#);

let result = lexer.tokenize(&source);
if let Some(errors) = result.errors() {
    println!("Parse errors found: {:?}", errors);
} else {
    println!("Parsed successfully.");
}
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **Document**: Root container for Notedown documents
- **Heading**: Heading elements with levels (1-6)
- **Paragraph**: Text paragraphs
- **List**: Ordered and unordered lists
- **CodeBlock**: Fenced code blocks
- **Inline**: Emphasis, strong, links, and inline code

## 📊 Performance

- **Streaming**: Parse large Notedown files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak Notedown integrates seamlessly with:

- **Static Site Generators**: Convert Notedown to HTML for websites
- **Documentation Tools**: Process and render Notedown documentation
- **Content Management**: Handle user-generated Notedown content
- **IDE Support**: Language server protocol compatibility
- **Blog Platforms**: Parse and render blog posts in Notedown

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete Notedown document parsing
- Heading and list analysis
- Code transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-notedown) or open [issues](https://github.com/ygg-lang/oaks/issues).