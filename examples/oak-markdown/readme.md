# Oak Markdown Parser

[![Crates.io](https://img.shields.io/crates/v/oak-markdown.svg)](https://crates.io/crates/oak-markdown)
[![Documentation](https://docs.rs/oak-markdown/badge.svg)](https://docs.rs/oak-markdown)

High-performance incremental Markdown parser for the oak ecosystem with flexible configuration, optimized for document processing and rendering.

## 🎯 Overview

Oak Markdown is a robust parser for Markdown, designed to handle complete Markdown syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for document processing and rendering.

## ✨ Features

- **Complete Markdown Syntax**: Supports all Markdown features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_markdown::{Parser, MarkdownLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
# Hello, Markdown!

This is a **paragraph** with *emphasis*.

## Features

- Lists
- Code blocks
- And more!
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed Markdown successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Document Parsing
```rust
use oak_markdown::{Parser, MarkdownLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
# My Document

This is a simple document.
"#);

let result = parser.parse(&source);
println!("Document parsed successfully.");
```

### Heading Parsing
```rust
use oak_markdown::{Parser, MarkdownLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
## My Heading

Some content here.
"#);

let result = parser.parse(&source);
println!("Heading parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_markdown::{Parser, MarkdownLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("# Heading\n\nParagraph text");
let result = parser.parse(&source);
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_markdown::{Parser, MarkdownLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
# Heading

This is a paragraph
## Another heading
# Unclosed heading
"#);

let result = parser.parse(&source);
if let Some(errors) = result.result.err() {
    println!("Parse errors found: {:?}", errors);
} else {
    println!("Parsed successfully.");
}
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **MarkdownDocument**: Root container for Markdown documents
- **Heading**: Heading elements with levels
- **Paragraph**: Text paragraphs
- **List**: Ordered and unordered lists
- **CodeBlock**: Fenced code blocks
- **Inline**: Emphasis, strong, links, and inline code

## 📊 Performance

- **Streaming**: Parse large Markdown files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak Markdown integrates seamlessly with:

- **Static Site Generators**: Convert Markdown to HTML for websites
- **Documentation Tools**: Process and render Markdown documentation
- **Content Management**: Handle user-generated Markdown content
- **IDE Support**: Language server protocol compatibility
- **Blog Platforms**: Parse and render blog posts in Markdown

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete Markdown document parsing
- Heading and list analysis
- Code transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-markdown) or open [issues](https://github.com/ygg-lang/oaks/issues).