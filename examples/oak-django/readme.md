# Oak Django Parser

[![Crates.io](https://img.shields.io/crates/v/oak-django.svg)](https://crates.io/crates/oak-django)
[![Documentation](https://docs.rs/oak-django/badge.svg)](https://docs.rs/oak-django)

A high-performance Django template parser with streaming support, built on oak-core for efficient template parsing and validation.

## 🎯 Overview

oak-django is a fast and memory-efficient Django template parser designed to handle Django template syntax including template tags, filters, and variables. Built on the reliable oak-core foundation, it provides comprehensive template parsing with excellent error reporting and validation capabilities.

## ✨ Features

- **Streaming Support**: Parse large JSON files without loading entirely into memory
- **RFC 7159 Compliant**: Full compliance with JSON specification
- **Zero-Copy**: Efficient parsing with minimal memory allocations
- **Error Recovery**: Detailed error messages with line/column information
- **Type Validation**: Strict type checking and validation
- **Fast Performance**: Optimized for speed and memory efficiency

## 🚀 Quick Start

Basic example:

```rust
use oak_json::JsonParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = JsonParser::new();
    let json = r#"{
        "name": "John Doe",
        "age": 30,
        "email": "john@example.com",
        "active": true
    }"#;
    
    let value = parser.parse(json)?;
    println!("Parsed JSON: {:?}", value);
    Ok(())
}
```

## 📋 Parsing Examples

### Object Parsing
```rust
use oak_json::{JsonParser, JsonValue};

let parser = JsonParser::new();
let json = r#"{
    "user": {
        "id": 123,
        "name": "Alice",
        "roles": ["admin", "user"]
    },
    "settings": {
        "theme": "dark",
        "notifications": true
    }
}"#;

let value = parser.parse(json)?;
if let JsonValue::Object(obj) = value {
    println!("User ID: {:?}", obj.get("user").and_then(|u| u.get("id")));
}
```

### Array Parsing
```rust
use oak_json::{JsonParser, JsonValue};

let parser = JsonParser::new();
let json = r#"[
    {"id": 1, "name": "Item 1", "price": 10.50},
    {"id": 2, "name": "Item 2", "price": 25.99},
    {"id": 3, "name": "Item 3", "price": 7.25}
]"#;

let value = parser.parse(json)?;
if let JsonValue::Array(items) = value {
    for item in items {
        if let JsonValue::Object(obj) = item {
            println!("Item: {:?} - ${:?}", 
                obj.get("name"), obj.get("price"));
        }
    }
}
```

### Streaming Large Files
```rust
use oak_json::{JsonParser, JsonValue};
use std::fs::File;
use std::io::BufReader;

let parser = JsonParser::new();
let file = File::open("large_data.json")?;
let reader = BufReader::new(file);

// Parse large JSON files efficiently
let value = parser.parse_reader(reader)?;
println!("Parsed large JSON with {} top-level keys", 
    value.as_object().map(|o| o.len()).unwrap_or(0));
```

## 🔧 Advanced Features

### Custom Validation
```rust
use oak_json::{JsonParser, JsonValue, ValidationError};

let parser = JsonParser::new();
let json = r#"{
    "username": "john_doe",
    "age": 25,
    "email": "john@example.com"
}"#;

let value = parser.parse(json)?;

// Validate required fields
fn validate_user(data: &JsonValue) -> Result<(), ValidationError> {
    if let JsonValue::Object(obj) = data {
        obj.get("username").ok_or("Missing username")?;
        obj.get("email").ok_or("Missing email")?;
        
        // Validate email format
        if let Some(JsonValue::String(email)) = obj.get("email") {
            if !email.contains('@') {
                return Err("Invalid email format".into());
            }
        }
    }
    Ok(())
}

validate_user(&value)?;
```

### Partial Parsing
```rust
use oak_json::{JsonParser, JsonPath};

let parser = JsonParser::new();
let json = r#"{
    "users": [
        {"id": 1, "name": "Alice"},
        {"id": 2, "name": "Bob"}
    ],
    "total": 2
}"#;

// Parse only specific parts
let users = parser.parse_path(json, JsonPath::new("users"))?;
println!("Users array: {:?}", users);
```

### Error Handling
```rust
use oak_json::JsonParser;

let parser = JsonParser::new();
let invalid_json = r#"{
    "name": "John",
    "age": 30,
    "active": true,
    "scores": [85, 92, "invalid", 78]
}"#;

match parser.parse(invalid_json) {
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

## 🏗️ JSON Structure

The parser generates a comprehensive JSON value structure:

- **JsonValue::Null**: JSON null values
- **JsonValue::Bool**: Boolean true/false
- **JsonValue::Number**: Numeric values (integers and floats)
- **JsonValue::String**: String values
- **JsonValue::Array**: Ordered collections of values
- **JsonValue::Object**: Key-value mappings

## 📊 Performance

- **Streaming**: Parse multi-GB JSON files with minimal memory usage
- **Zero-Copy**: Efficient string handling with minimal allocations
- **Fast Parsing**: Optimized parser for maximum throughput
- **Incremental**: Support for incremental parsing of partial data

## 🔗 Integration

oak-json integrates seamlessly with:

- **Web APIs**: Parse JSON responses from HTTP APIs
- **Configuration Files**: Handle application configuration in JSON format
- **Data Processing**: Process large JSON datasets efficiently
- **Logging**: Parse structured log data in JSON format
- **Testing**: Generate and validate test data in JSON

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Basic JSON parsing and validation
- Streaming large JSON files
- Custom validation and error handling
- Partial parsing with JSONPath
- Performance benchmarks

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

---

**Pex JSON Parser** - Fast, reliable JSON parsing for Rust applications 🚀