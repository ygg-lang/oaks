# Oak XML Parser

[![Crates.io](https://img.shields.io/crates/v/oak-xml.svg)](https://crates.io/crates/oak-xml)
[![Documentation](https://docs.rs/oak-xml/badge.svg)](https://docs.rs/oak-xml)

High-performance incremental XML parser for the oak ecosystem with flexible configuration, optimized for data processing and document parsing.

## 🎯 Overview

Oak of xml is a robust parser for XML, designed to handle complete XML syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for data processing and document parsing.

## ✨ Features

- **Complete XML Syntax**: Supports all XML features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_xml::XmlParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = XmlParser::new();
    let xml_content = r#"
<?xml version="1.0" encoding="UTF-8"?>
<root>
    <item id="1" type="example">Value 1</item>
    <item id="2" type="sample">Value 2</item>
</root>
    "#;
    
    let document = parser.parse_document(xml_content)?;
    println!("Parsed XML document successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Document Parsing
```rust
use oak_xml::{XmlParser, ast::Document};

let parser = XmlParser::new();
let xml_content = r#"
<?xml version="1.0"?>
<catalog>
    <book id="bk101">
        <title>XML Developer's Guide</title>
    </book>
</catalog>
"#;

let document = parser.parse_document(xml_content)?;
println!("Root element: {}", document.root_element.name);
```

### Element Parsing
```rust
use oak_xml::{XmlParser, ast::Element};

let parser = XmlParser::new();
let xml_content = r#"
<person age="30">
    <name>John Doe</name>
    <email>john@example.com</email>
</person>
"#;

let element = parser.parse_element(xml_content)?;
println!("Element tag: {}", element.tag_name);
println!("Attributes: {}", element.attributes.len());
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_xml::{XmlParser, lexer::Token};

let parser = XmlParser::new();
let tokens = parser.tokenize("<root><item>value</item></root>")?;
for token in tokens {
    println!("{:?}", token.kind);
}
```

### Error Handling
```rust
use oak_xml::XmlParser;

let parser = XmlParser::new();
let invalid_xml = r#"
<?xml version="1.0"?>
<root>
    <item>Missing closing tag
</root>
"#;

match parser.parse_document(invalid_xml) {
    Ok(document) => println!("Parsed XML document successfully."),
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

- **Document**: Root container for XML documents
- **Element**: XML elements with tags and attributes
- **Attribute**: Element attributes with name-value pairs
- **Text**: Text content nodes
- **Comment**: XML comments
- **ProcessingInstruction**: XML processing instructions

## 📊 Performance

- **Streaming**: Parse large XML files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak of xml integrates seamlessly with:

- **Data Processing**: Extract data from XML documents
- **Configuration Files**: Parse XML configuration files
- **Web Services**: Process XML API responses
- **IDE Support**: Language server protocol compatibility
- **Document Processing**: XML parsing for document management

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete XML document parsing
- Element and attribute analysis
- Data extraction and transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-xml) or open [issues](https://github.com/ygg-lang/oaks/issues).