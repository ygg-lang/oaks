# Oak INI Parser

[![Crates.io](https://img.shields.io/crates/v/oak-ini.svg)](https://crates.io/crates/oak-ini)
[![Documentation](https://docs.rs/oak-ini/badge.svg)](https://docs.rs/oak-ini)

A high-performance INI parser with streaming support, built on oak-core for efficient INI parsing and validation.

## 🎯 Overview

Oak-ini is a fast and memory-efficient INI parser designed to handle both small INI documents and large configuration files. Built on the reliable oak-core foundation, it provides comprehensive INI parsing with excellent error reporting and validation capabilities.

## ✨ Features

- **Streaming Support**: Parse large INI files without loading entirely into memory
- **Complete INI Syntax**: Full support for standard INI format
- **Zero-Copy**: Efficient parsing with minimal memory allocations
- **Error Recovery**: Detailed error messages with line/column information
- **Type Validation**: Strict type checking and validation
- **Fast Performance**: Optimized for speed and memory efficiency

## 🚀 Quick Start


Basic example:

```rust
use oak_ini::IniParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = IniParser::new();
    let ini = r#"
[database]
host = localhost
port = 5432
username = admin
password = secret123

[server]
host = 0.0.0.0
port = 8080
debug = true
    "#;
    
    let document = parser.parse(ini)?;
    println!("Parsed INI: {:?}", document);
    Ok(())
}
```

## 📋 Parsing Examples

### Document Parsing
```rust
use oak_ini::{IniParser, IniValue};

let parser = IniParser::new();
let ini = r#"[application]
name = MyApp
version = 1.0.0
author = John Doe

[database]
host = localhost
port = 5432"#;

let value = parser.parse(ini)?;
if let IniValue::Document(doc) = value {
    println!("Application name: {:?}", doc.get_section("application")
        .and_then(|s| s.get("name")));
}
```

### Section Parsing
```rust
use oak_ini::{IniParser, IniValue};

let parser = IniParser::new();
let ini = r#"[server]
host = 127.0.0.1
port = 3000
ssl = true"#;

let value = parser.parse(ini)?;
if let IniValue::Section(section) = value {
    println!("Server host: {:?}", section.get("host"));
}
```

### Property Parsing
```rust
use oak_ini::{IniParser, IniValue};

let parser = IniParser::new();
let ini = "max_connections = 100";

let value = parser.parse(ini)?;
if let IniValue::Property(key, value) = value {
    println!("Property: {} = {}", key, value);
}
```

### Streaming Large Files
```rust
use oak_ini::{IniParser, IniValue};
use std::fs::File;
use std::io::BufReader;

let parser = IniParser::new();
let file = File::open("large_config.ini")?;
let reader = BufReader::new(file);

// Parse large INI files efficiently
let value = parser.parse_reader(reader)?;
println!("Parsed large INI configuration");
```

## 🔧 Advanced Features

### Custom Validation
```rust
use oak_ini::{IniParser, IniValue, ValidationError};

let parser = IniParser::new();
let ini = r#"[server]
host = localhost
port = 8080
ssl = true"#;

let value = parser.parse(ini)?;

// Validate required sections
fn validate_config(data: &IniValue) -> Result<(), ValidationError> {
    if let IniValue::Document(doc) = data {
        doc.get_section("server").ok_or("Missing server section")?;
        
        // Validate port range
        if let Some(port) = doc.get_section("server")
            .and_then(|s| s.get("port"))
            .and_then(|p| p.parse::<u16>().ok()) {
            if port > 65535 {
                return Err("Invalid port range".into());
            }
        }
    }
    Ok(())
}

validate_config(&value)?;
```

### Partial Parsing
```rust
use oak_ini::{IniParser, IniPath};

let parser = IniParser::new();
let ini = r#"[database]
host = localhost
port = 5432

[server]
host = 127.0.0.1
port = 3000"#;

// Parse only specific sections
let db_config = parser.parse_path(ini, IniPath::new("database"))?;
println!("Database config: {:?}", db_config);
```

### Error Handling
```rust
use oak_ini::IniParser;

let parser = IniParser::new();
let invalid_ini = r#"[database]
host = localhost
port = not_a_number

[server]
host = 127.0.0.1
port = 3000"#;

match parser.parse(invalid_ini) {
    Ok(value) => println!("Parsed: {:?}", value),
    Err(e) => {
        println!("Parse error at line {} column {}: {}", 
            e.line(), e.column(), e.message());
        // Get detailed error information
        if let Some(context) = e.context() {
            println!("Error context: {}", context);
        }
    }
}
```

## 🏗️ INI Structure

The parser generates a comprehensive INI value structure:

- **IniValue::Document**: Root container for INI documents
- **IniValue::Section**: INI sections with names and properties
- **IniValue::Property**: Key-value pairs
- **IniValue::Comment**: Comment lines
- **IniValue::Empty**: Empty lines

## 📊 Performance

- **Streaming**: Parse large INI files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak-ini integrates seamlessly with:

- **Configuration Files**: Parse INI configuration files
- **Settings Management**: Process application settings
- **Legacy Systems**: Handle legacy INI format files
- **IDE Support**: Language server protocol compatibility
- **Build Tools**: INI parsing for build configuration

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete INI document parsing
- Section and property analysis
- Configuration extraction and transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-ini) or open [issues](https://github.com/ygg-lang/oaks/issues).