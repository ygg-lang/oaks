# Oak Protobuf Parser

[![Crates.io](https://img.shields.io/crates/v/oak-protobuf.svg)](https://crates.io/crates/oak-protobuf)
[![Documentation](https://docs.rs/oak-protobuf/badge.svg)](https://docs.rs/oak-protobuf)

A high-performance Protobuf parser for Rust, built with the Oak parser combinator framework. Parse Protocol Buffer definitions with comprehensive AST generation and error handling.

## Overview

Oak Protobuf provides robust parsing capabilities for Protocol Buffer schema files, supporting messages, enums, services, fields, options, and all major Protobuf constructs. Built on the Oak parser combinator framework, it delivers excellent performance and detailed error messages.

## Features

- ✅ **Complete Protobuf Support**: Parse messages, enums, services, fields, options, and imports
- ✅ **Modern Rust API**: Type-safe parsing with comprehensive error handling
- ✅ **High Performance**: Built on the efficient Oak parser combinator framework
- ✅ **Rich AST**: Detailed Abstract Syntax Tree with source location tracking
- ✅ **Extensible**: Easy to extend for custom Protobuf dialects
- ✅ **Well Tested**: Comprehensive test suite with real-world examples

## Quick Start

## Parsing Examples

### Basic Message Parsing

```rust
use oak::{Parser, Language};
use oak_protobuf::ProtobufLanguage;

fn main() {
    let source = r#"
        kind = "proto3";
        
        package example;
        
        message Person {
            string name = 1;
            int32 age = 2;
            repeated string emails = 3;
        }
    "#;
    
    let mut parser = Parser::<ProtobufLanguage>::new();
    match parser.parse(&source) {
        Ok(ast) => {
            println!("Parsed AST: {:#?}", ast);
        }
        Err(error) => {
            eprintln!("Parse error: {}", error);
        }
    }
}
```

### Complex Schema with Services

```rust
use oak::{Parser, Language};
use oak_protobuf::ProtobufLanguage;

fn main() {
    let source = r#"
        kind = "proto3";
        
        package bookstore;
        
        import "google/protobuf/timestamp.proto";
        
        message Book {
            string isbn = 1;
            string title = 2;
            repeated string authors = 3;
            google.protobuf.Timestamp published_date = 4;
        }
        
        service BookService {
            rpc GetBook(GetBookRequest) returns (Book);
            rpc ListBooks(ListBooksRequest) returns (stream Book);
        }
        
        message GetBookRequest {
            string isbn = 1;
        }
        
        message ListBooksRequest {
            int32 page_size = 1;
            string page_token = 2;
        }
    "#;
    
    let mut parser = Parser::<ProtobufLanguage>::new();
    match parser.parse(&source) {
        Ok(ast) => {
            println!("Service definitions parsed successfully!");
        }
        Err(error) => {
            eprintln!("Parse error: {}", error);
        }
    }
}
```

## Advanced Features

### Custom Options

Oak Protobuf supports parsing custom options:

```rust
let source = r#"
    kind = "proto3";
    
    message MyMessage {
        string value = 1 [(custom_option) = "test"];
    }
"#;
```

### Enum Definitions

Parse enum types with aliases and custom options:

```rust
let source = r#"
    enum Status {
        UNKNOWN = 0;
        ACTIVE = 1;
        INACTIVE = 2 [(custom_option) = "deprecated"];
    }
"#;
```

## AST Structure

The parser generates a rich AST with the following main node types:

- `ProtobufFile` - Root node containing the entire file
- `Syntax` - Syntax version declaration
- `Package` - Package declaration
- `Import` - Import statements
- `Message` - Message definitions with fields
- `Enum` - Enum type definitions
- `Service` - Service definitions with RPC methods
- `Field` - Message fields with types and options
- `Option` - Custom options for various elements

## Performance

Oak Protobuf is designed for high performance:

- **Zero-copy parsing** where possible
- **Streaming support** for large files
- **Efficient memory usage** with minimal allocations
- **Fast error recovery** for better developer experience

## Integration

Oak Protobuf integrates seamlessly with the Oak ecosystem:

```rust
use oak::{Parser, Language};
use oak_protobuf::ProtobufLanguage;

// Use with other Oak parsers
let mut parser = Parser::<ProtobufLanguage>::new();
let result = parser.parse(protobuf_source);
```

## Examples

More examples can be found in the [examples directory](https://github.com/axodotdev/oak/tree/main/examples/oak-protobuf/examples):

- [Basic message parsing](examples/basic.rs)
- [Service definitions](examples/services.rs)
- [Custom options](examples/options.rs)
- [Error handling](examples/error_handling.rs)

## Contributing

We welcome contributions! Please see our [Contributing Guide](https://github.com/axodotdev/oak/blob/main/CONTRIBUTING.md) for details.