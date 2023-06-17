# Oak Org-Mode Parser

[![Crates.io](https://img.shields.io/crates/v/oak-org-mode.svg)](https://crates.io/crates/oak-org-mode)
[![Documentation](https://docs.rs/oak-org-mode/badge.svg)](https://docs.rs/oak-org-mode)


A comprehensive Org-mode parser supporting Emacs Org-mode syntax, built on oak-core for accurate document parsing and AST generation.

## 🎯 Overview

Oak-org-mode is a robust Org-mode parser designed to handle the complete Emacs Org-mode specification including headlines, lists, code blocks, and metadata. Built on the solid foundation of oak-core, it provides accurate parsing of Org-mode documents with detailed AST generation and comprehensive extension support.

## ✨ Features

- **Complete Org-mode Syntax**: Full support for Emacs Org-mode specification
- **Headline Support**: Support for hierarchical headlines with properties
- **Streaming Support**: Parse large Org-mode documents efficiently
- **AST Generation**: Detailed Abstract Syntax Tree for document manipulation
- **Code Block Support**: Support for source code blocks and their metadata
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_markdown::MarkdownParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = MarkdownParser::new();
    let markdown = r#"# Hello World

This is a **bold** statement and this is *italic* text.

## Features

- First item
- Second item
- Third item

[Link to documentation](https://docs.rs/oak-markdown)"#;
    
    let document = parser.parse(markdown)?;
    println!("Parsed {} blocks", document.blocks.len());
    Ok(())
}
```

## 📋 Parsing Examples

### Document Structure
```rust
use oak_markdown::{MarkdownParser, ast::Block};

let parser = MarkdownParser::new();
let markdown = r#"# Main Title

## Section 1

This is the first paragraph with **bold** and *italic* text.

### Subsection

Here's a [link](https://example.com) and some `inline code`.

## Section 2

- List item 1
- List item 2
- List item 3"#;

let document = parser.parse(markdown)?;
for block in &document.blocks {
    match block {
        Block::Heading(heading) => {
            println!("Heading level {}: {}", heading.level, heading.text);
        }
        Block::Paragraph(paragraph) => {
            println!("Paragraph with {} inline elements", paragraph.inlines.len());
        }
        _ => {}
    }
}
```

### Table Parsing
```rust
use oak_markdown::{MarkdownParser, ast::Block};

let parser = MarkdownParser::new();
let markdown = r#"| Name | Age | City |
|------|-----|------|
| Alice | 25 | New York |
| Bob | 30 | London |
| Carol | 28 | Tokyo |

## Code Block

```rust
fn main() {
    println!("Hello, World!");
}
```"#;

let document = parser.parse(markdown)?;
for block in &document.blocks {
    match block {
        Block::Table(table) => {
            println!("Table with {} rows and {} columns", 
                table.rows.len(), table.headers.len());
        }
        Block::CodeBlock(code) => {
            println!("Code block ({}): {}", code.language.as_deref().unwrap_or("text"), code.content);
        }
        _ => {}
    }
}
```

### Task Lists and Extensions
```rust
use oak_markdown::{MarkdownParser, ast::Block, extensions::Extensions};

let mut parser = MarkdownParser::new();
parser.enable_extensions(Extensions::all());

let markdown = r#"## Todo List

- [x] Complete the project
- [ ] Write documentation
- [ ] Add tests
- [x] Review code

### Strikethrough

This is ~~deleted~~ text and this is ==highlighted== text.

### Autolinks

Visit https://github.com for more information."#;

let document = parser.parse(markdown)?;
for block in &document.blocks {
    match block {
        Block::List(list) => {
            println!("List with {} items:", list.items.len());
            for item in &list.items {
                if let Some(checked) = item.checked {
                    println!("  - [{}] {}", 
                        if checked { "x" } else { " " }, 
                        item.text);
                }
            }
        }
        _ => {}
    }
}
```

## 🔧 Advanced Features

### Custom Extensions
```rust
use oak_markdown::{MarkdownParser, extensions::Extension};

struct CustomEmojiExtension;

impl Extension for CustomEmojiExtension {
    fn name(&self) -> &str { "custom_emoji" }
    
    fn process_inline(&self, text: &str) -> Option<Vec<ast::Inline>> {
        // Convert :smile: to emoji
        if text.contains(":smile:") {
            Some(vec![ast::Inline::Text("😊".to_string())])
        } else {
            None
        }
    }
}

let mut parser = MarkdownParser::new();
parser.add_extension(Box::new(CustomEmojiExtension));

let markdown = "Hello :smile: World!";
let document = parser.parse(markdown)?;
```

### AST Manipulation
```rust
use oak_markdown::{MarkdownParser, ast::{Block, Document}};

let parser = MarkdownParser::new();
let markdown = "# Original Title\n\nOriginal content.";
let mut document = parser.parse(markdown)?;

// Add a new heading
document.blocks.push(Block::Heading(ast::Heading {
    level: 2,
    text: "Added Section".to_string(),
    inlines: vec![ast::Inline::Text("Added Section".to_string())]
}));

// Serialize back to markdown
let new_markdown = document.to_markdown();
println!("Modified markdown:\n{}", new_markdown);
```

### HTML Generation
```rust
use oak_markdown::{MarkdownParser, html::HtmlRenderer};

let parser = MarkdownParser::new();
let markdown = r#"# Document Title

This is a paragraph with **bold** text.

- List item 1
- List item 2

[Link](https://example.com)"#;

let document = parser.parse(markdown)?;
let renderer = HtmlRenderer::new();
let html = renderer.render(&document)?;

println!("Generated HTML:\n{}", html);
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **Document**: Root container with metadata and blocks
- **Blocks**: Headings, paragraphs, lists, code blocks, tables, blockquotes
- **Inlines**: Text, emphasis, strong, code, links, images
- **Extensions**: Tables, task lists, strikethrough, autolinks

## 📊 Performance

- **Streaming**: Parse large Markdown documents efficiently
- **Incremental**: Re-parse only changed sections
- **Fast Recovery**: Quick error recovery for better IDE integration
- **Memory Efficient**: Minimal memory footprint for large documents

## 🔗 Integration

Oak of markdown integrates seamlessly with:

- **Static Site Generators**: Convert Markdown to HTML for websites
- **Documentation Tools**: Parse documentation in Markdown format
- **Content Management**: Handle user-generated Markdown content
- **IDE Support**: Language server protocol compatibility
- **Blog Platforms**: Process blog posts and articles

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Basic Markdown parsing and HTML generation
- Custom extensions and plugins
- Document manipulation and transformation
- Performance benchmarks
- Integration with web frameworks

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

---

**Pex Markdown Parser** - Comprehensive Markdown parsing for Rust applications 🚀