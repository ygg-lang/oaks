# 🛠️ XML Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-xml`.

## 🚦 Quick Start

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
oak-xml = { path = "..." }
```

### Basic Parsing Example

The following is a standard workflow for parsing an XML document:

```rust
use oak_xml::{XmlParser, SourceText, XmlLanguage};

fn main() {
    // 1. Prepare source code
    let code = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <note>
            <to>Tove</to>
            <from>Jani</from>
            <heading>Reminder</heading>
            <body>Don't forget me this weekend!</body>
        </note>
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser
    let config = XmlLanguage::new();
    let parser = XmlParser::new(&config);

    // 3. Execute parsing
    let result = parser.parse(&source);

    // 4. Handle results
    if result.is_success() {
        println!("Parsing successful! AST node count: {}", result.node_count());
    } else {
        eprintln!("Errors found during parsing.");
    }
}
```

## 🔍 Core API Usage

### 1. Syntax Tree Traversal
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract XML elements, attributes, text content, and handle namespaces.

### 2. Incremental Parsing
No need to re-parse massive XML data exports or documents when small changes occur:
```rust
// Assuming you have an old parse result 'old_result' and new source text 'new_source'
let new_result = parser.reparse(&new_source, &old_result);
```

### 3. Diagnostics
`oak-xml` provides precise error feedback for malformed XML, such as unclosed tags, attribute quoting issues, or namespace prefix errors:
```rust
for diag in result.diagnostics() {
    println!("[{}:{}] {}", diag.line, diag.column, diag.message);
}
```

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes XML source text into a stream of tokens, handling tag boundaries, attribute names/values, character data, and processing instructions.
- **Parser**: Syntax analyzer based on the Pratt parsing algorithm to handle XML's hierarchical structure and various node types.
- **AST**: A strongly-typed syntax abstraction layer designed for high-performance XML analysis, formatting, and validation tools.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/](tests/) for handling of various XML standards and edge cases.
