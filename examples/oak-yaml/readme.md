# Oak YAML Parser

[![Crates.io](https://img.shields.io/crates/v/oak-yaml.svg)](https://crates.io/crates/oak-yaml)
[![Documentation](https://docs.rs/oak-yaml/badge.svg)](https://docs.rs/oak-yaml)

High-performance incremental YAML parser for the oak ecosystem with flexible configuration, optimized for data serialization and configuration file processing.

## 🎯 Overview

Oak of yaml is a robust parser for YAML, designed to handle complete YAML syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for data serialization and configuration processing.

## ✨ Features

- **Complete YAML Syntax**: Supports all YAML features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_yaml::YamlParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = YamlParser::new();
    let yaml_content = r#"
server:
  host: localhost
  port: 8080
  
database:
  url: postgresql://localhost/mydb
  pool_size: 10
    "#;
    
    let document = parser.parse_document(yaml_content)?;
    println!("Parsed YAML document successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Document Parsing
```rust
use oak_yaml::{YamlParser, ast::Document};

let parser = YamlParser::new();
let yaml_content = r#"
name: John Doe
age: 30
hobbies:
  - reading
  - hiking
  - coding
"#;

let document = parser.parse_document(yaml_content)?;
println!("Person name: {:?}", document.get_value("name"));
```

### Mapping Parsing
```rust
use oak_yaml::{YamlParser, ast::Mapping};

let parser = YamlParser::new();
let mapping_content = r#"
config:
  debug: true
  timeout: 30
  retries: 3
"#;

let mapping = parser.parse_mapping(mapping_content)?;
println!("Debug mode: {:?}", mapping.get_value("debug"));
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_yaml::{YamlParser, lexer::Token};

let parser = YamlParser::new();
let tokens = parser.tokenize("key: value\nlist:\n  - item1\n  - item2")?;
for token in tokens {
    println!("{:?}", token.kind);
}
```

### Error Handling
```rust
use oak_yaml::YamlParser;

let parser = YamlParser::new();
let invalid_yaml = r#"
config:
  key: value
    nested: invalid_indentation
"#;

match parser.parse_document(invalid_yaml) {
    Ok(document) => println!("Parsed YAML document successfully."),
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

- **Document**: Root container for YAML documents
- **Mapping**: YAML key-value mappings
- **Sequence**: YAML arrays/sequences
- **Scalar**: YAML scalar values (string, number, boolean, null)
- **Anchor**: YAML anchor references
- **Alias**: YAML alias references

## 📊 Performance

- **Streaming**: Parse large YAML files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak of yaml integrates seamlessly with:

- **Configuration Management**: Parse and validate application configurations
- **Data Serialization**: Serialize and deserialize YAML data
- **API Processing**: Handle YAML API requests and responses
- **IDE Support**: Language server protocol compatibility
- **DevOps Tools**: Configuration validation and processing

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete YAML document parsing
- Configuration file analysis
- Data extraction and transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-yaml) or open [issues](https://github.com/ygg-lang/oaks/issues).