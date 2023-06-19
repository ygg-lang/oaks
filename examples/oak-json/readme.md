# Oak JSON Parser

[![Crates.io](https://img.shields.io/crates/v/oak-json.svg)](https://crates.io/crates/oak-json)
[![Documentation](https://docs.rs/oak-json/badge.svg)](https://docs.rs/oak-json)

High-performance incremental JSON parser for the oak ecosystem with flexible configuration, optimized for data processing and analysis.

## 🎯 Overview

Oak JSON is a robust parser for JSON, designed to handle complete JSON syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for data processing and analysis.

## ✨ Features

- **Complete JSON Syntax**: Supports all JSON features including objects, arrays, primitives
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_json::{Parser, JsonLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
        {
            "name": "Alice",
            "age": 30,
            "skills": ["Rust", "JavaScript", "Python"]
        }
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed JSON successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Object Parsing
```rust
use oak_json::{Parser, JsonLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"{"name": "Alice", "age": 30}"#);

let result = parser.parse(&source);
println!("Object parsed successfully.");
```

### Array Parsing
```rust
use oak_json::{Parser, JsonLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"[1, 2, 3, 4, 5]"#);

let result = parser.parse(&source);
println!("Array parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_json::{Parser, JsonLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"{"key": "value"}"#);
let result = parser.parse(&source);
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_json::{Parser, JsonLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
    {
        "name": "Alice",
        "age": 30,
        "skills": ["Rust", "JavaScript", "Python"
    // Missing closing brace
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

- **JsonDocument**: Root container for JSON documents
- **Object**: JSON objects with key-value pairs
- **Array**: JSON arrays with ordered elements
- **Value**: JSON values (string, number, boolean, null)
- **Property**: Object properties with key-value pairs

## 📊 Performance

- **Streaming**: Parse large JSON files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better processing

## 🔗 Integration

Oak JSON integrates seamlessly with:

- **Data Processing**: JSON data extraction and transformation
- **Configuration Files**: Parsing application configurations
- **API Integration**: Processing JSON API responses
- **Static Analysis**: JSON schema validation and analysis
- **Code Generation**: Generating code from JSON schemas

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete JSON document parsing
- Object and array analysis
- Data transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-json) or open [issues](https://github.com/ygg-lang/oaks/issues).