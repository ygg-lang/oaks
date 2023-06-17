# Oak TOML Parser

[![Crates.io](https://img.shields.io/crates/v/oak-toml.svg)](https://crates.io/crates/oak-toml)
[![Documentation](https://docs.rs/oak-toml/badge.svg)](https://docs.rs/oak-toml)

High-performance incremental TOML parser for the oak ecosystem with flexible configuration, optimized for configuration file processing and data serialization.

## 🎯 Overview

Oak of toml is a robust parser for TOML, designed to handle complete TOML syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for configuration processing and data serialization.

## ✨ Features

- **Complete TOML Syntax**: Supports all TOML features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_toml::TomlParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = TomlParser::new();
    let toml_content = r#"
[package]
name = "oak-toml"
version = "0.1.0"
authors = ["Oak Contributors"]


    "#;
    
    let document = parser.parse_document(toml_content)?;
    println!("Parsed TOML document successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Document Parsing
```rust
use oak_toml::{TomlParser, ast::Document};

let parser = TomlParser::new();
let toml_content = r#"
[server]
host = "localhost"
port = 8080

[database]
url = "postgresql://localhost/mydb"
pool_size = 10
"#;

let document = parser.parse_document(toml_content)?;
println!("Server config: {:?}", document.get_table("server"));
```

### Table Parsing
```rust
use oak_toml::{TomlParser, ast::Table};

let parser = TomlParser::new();
let table_content = r#"
[package]
name = "my-project"
version = "1.0.0"
description = "A sample project"
"#;

let table = parser.parse_table(table_content)?;
println!("Package name: {:?}", table.get_value("name"));
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_toml::{TomlParser, lexer::Token};

let parser = TomlParser::new();
let tokens = parser.tokenize("key = 'value'\narray = [1, 2, 3]")?;
for token in tokens {
    println!("{:?}", token.kind);
}
```

### Error Handling
```rust
use oak_toml::TomlParser;

let parser = TomlParser::new();
let invalid_toml = r#"
[section
key = "value"
"#;

match parser.parse_document(invalid_toml) {
    Ok(document) => println!("Parsed TOML document successfully."),
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

- **Document**: Root container for TOML documents
- **Table**: TOML tables with key-value pairs
- **Array**: TOML arrays of values
- **Value**: Various value types (string, integer, float, boolean, datetime)
- **Key**: Table and key names

## 📊 Performance

- **Streaming**: Parse large TOML files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak of toml integrates seamlessly with:

- **Configuration Management**: Parse and validate application configurations
- **Build Tools**: Process Cargo.toml and similar configuration files
- **Data Serialization**: Serialize and deserialize TOML data
- **IDE Support**: Language server protocol compatibility
- **DevOps Tools**: Configuration validation and processing

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete TOML document parsing
- Configuration file analysis
- Data extraction and transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-toml) or open [issues](https://github.com/ygg-lang/oaks/issues).