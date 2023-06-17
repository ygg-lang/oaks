# Oak HTML Parser

[![Crates.io](https://img.shields.io/crates/v/oak-html.svg)](https://crates.io/crates/oak-html)
[![Documentation](https://docs.rs/oak-html/badge.svg)](https://docs.rs/oak-html)

High-performance incremental HTML parser for the oak ecosystem with flexible configuration, optimized for web development and document processing.

## 🎯 Overview

Oak-html is a robust parser for HTML, designed to handle complete HTML syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for web development and document processing.

## ✨ Features

- **Complete HTML Syntax**: Supports all HTML features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_html::HtmlParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = HtmlParser::new();
    let html_content = r#"
<!DOCTYPE html>
<html>
<head>
    <title>My Page</title>
</head>
<body>
    <h1>Hello, HTML!</h1>
    <p>This is a paragraph.</p>
</body>
</html>
    "#;
    
    let document = parser.parse_document(html_content)?;
    println!("Parsed HTML document successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Document Parsing
```rust
use oak_html::{HtmlParser, ast::Document};

let parser = HtmlParser::new();
let html_content = r#"
<!DOCTYPE html>
<html>
<head><title>Test</title></head>
<body><h1>Hello</h1></body>
</html>
"#;

let document = parser.parse_document(html_content)?;
println!("Document has {} elements", document.elements.len());
```

### Element Parsing
```rust
use oak_html::{HtmlParser, ast::Element};

let parser = HtmlParser::new();
let html_content = r#"
<div class="container" id="main">
    <p>Content</p>
</div>
"#;

let element = parser.parse_element(html_content)?;
println!("Element tag: {}", element.tag_name);
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_html::{HtmlParser, lexer::Token};

let parser = HtmlParser::new();
let tokens = parser.tokenize("<div>content</div>")?;
for token in tokens {
    println!("{:?}", token.kind);
}
```

### Error Handling
```rust
use oak_html::HtmlParser;

let parser = HtmlParser::new();
let invalid_html = r#"
<html>
<head><title>Test</title>
<body><h1>Hello</h1>
<!-- Missing closing tags -->
"#;

match parser.parse_document(invalid_html) {
    Ok(document) => println!("Parsed HTML document successfully."),
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

- **Document**: Root container for HTML documents
- **Element**: HTML elements with tags and attributes
- **Attribute**: Element attributes with name-value pairs
- **Text**: Text content nodes
- **Comment**: HTML comments

## 📊 Performance

- **Streaming**: Parse large HTML files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak of html integrates seamlessly with:

- **Web Scraping**: Extract data from HTML documents
- **Template Engines**: Parse and process HTML templates
- **Static Site Generators**: Process HTML content for websites
- **IDE Support**: Language server protocol compatibility
- **Web Development**: HTML parsing for development tools

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete HTML document parsing
- Element and attribute analysis
- Code transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-html) or open [issues](https://github.com/ygg-lang/oaks/issues).