# Oak AsciiDoc Parser

[![Crates.io](https://img.shields.io/crates/v/oak-ascii-doc.svg)](https://crates.io/crates/oak-ascii-doc)
[![Documentation](https://docs.rs/oak-ascii-doc/badge.svg)](https://docs.rs/oak-ascii-doc)

High-performance incremental AsciiDoc parser for the oak ecosystem with flexible configuration, optimized for document processing and AST generation.

## 🎯 Overview

Oak-ascii-doc is a robust parser for AsciiDoc, designed to handle complete AsciiDoc syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for document processing and rendering.

## ✨ Features

- **Complete AsciiDoc Syntax**: Supports all AsciiDoc features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_ascii_doc::AsciiDocParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = AsciiDocParser::new();
    let adoc_code = r#"
= My Document
:author: John Doe

Hello, *AsciiDoc*!

This is a paragraph.

. List item 1
. List item 2
"#;
    
    let document = parser.parse_document(adoc_code)?;
    println!("Parsed AsciiDoc document successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Document Parsing
```rust
use oak_ascii_doc::{AsciiDocParser, ast::Document};

let parser = AsciiDocParser::new();
let adoc_code = r#"
= My Title

A simple document.
"#;

let document = parser.parse_document(adoc_code)?;
println!("Document title: {}", document.title);
```

### Block Parsing
```rust
use oak_ascii_doc::{AsciiDocParser, ast::Block};

let parser = AsciiDocParser::new();
let adoc_code = r#"
== Section 1

This is a paragraph.

[source,rust]
----
fn main() {}
----
"#;

let document = parser.parse_document(adoc_code)?;
for block in &document.blocks {
    match block {
        Block::Section(section) => println!("Section title: {}", section.title),
        Block::Paragraph(paragraph) => println!("Paragraph content: {}", paragraph.content),
        _ => {}
    }
}
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_ascii_doc::{AsciiDocParser, lexer::Token};

let parser = AsciiDocParser::new();
let tokens = parser.tokenize("= Document Title")?;
for token in tokens {
    println!("{:?}", token.kind);
}
```

### Error Handling
```rust
use oak_ascii_doc::AsciiDocParser;

let parser = AsciiDocParser::new();
let invalid_adoc = r#"
= My Document

This is an invalid
= Section
"#;

match parser.parse_document(invalid_adoc) {
    Ok(document) => println!("Parsed AsciiDoc document successfully."),
    Err(e) => {
        println!("Parse error at line {} column {}: {}", 
            e.line(), e.column(), e.message());
        if let Some(context) = e.context() {
            println!("Error context: {}", context);
        }
    }
}
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **Document**: Root container for AsciiDoc documents
- **Block**: Sections, paragraphs, lists, code blocks, tables
- **Inline**: Text, emphasis, strong, links, images

## 📊 Performance

- **Streaming**: Parse large AsciiDoc files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak-ascii-doc integrates seamlessly with:

- **Document Processing**: AsciiDoc document conversion and transformation
- **Static Analysis**: Document structure and content analysis
- **IDE Support**: Language server protocol compatibility for AsciiDoc
- **Content Management**: Automated document processing workflows
- **Documentation**: Generating documentation from AsciiDoc sources

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete AsciiDoc document parsing
- Block and inline element analysis
- Code transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/projects/oak-ascii-doc) or open [issues](https://github.com/ygg-lang/oaks/issues).