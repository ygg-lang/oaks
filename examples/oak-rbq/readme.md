# Oak RBQ Parser

[![Crates.io](https://img.shields.io/crates/v/oak-rbq.svg)](https://crates.io/crates/oak-rbq)
[![Documentation](https://docs.rs/oak-rbq/badge.svg)](https://docs.rs/oak-rbq)

High-performance incremental RBQ (Rusty Brief Query) parser for the oak ecosystem with flexible configuration, optimized for data modeling and query analysis.

## 🎯 Overview

Oak RBQ is a robust parser for RBQ, a declarative data modeling and query language designed for the Rust ecosystem. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for domain-driven design and database schema management.

## ✨ Features

- **Declarative Modeling**: Support for `namespace`, `struct`, and `enum` definitions.
- **Annotation System**: Built-in support for `@table`, `@primary_key`, `@relation`, etc.
- **Type Safety**: Strong typing with support for `List<T>`, `Option<T>`, and domain-specific types.
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees.
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics.

## 🚀 Quick Start

Basic example:

```rust
use oak_rbq::{Parser, RbqLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
        @schema("auth")
        namespace Auth {
            struct User {
                @primary_key
                id: UUID,
                name: String,
                email: String?,
            }
        }
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed RBQ successfully.");
    Ok(())
}
```

## 📋 Modeling Examples

### Struct and Enum Definitions
```rust
use oak_rbq::{Parser, RbqLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
    enum Role {
        Admin,
        User,
        Guest
    }

    struct Account {
        id: i64,
        role: Role,
        created_at: DateTime
    }
"#);

let result = parser.parse(&source);
println!("RBQ model parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_sql::{Parser, SqlLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("SELECT * FROM users WHERE id = 1;");
let result = parser.parse(&source);
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_sql::{Parser, SqlLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
    SELECT name, 
    FROM users
    WHERE id = 1;
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

- **SelectStatement**: SELECT queries with columns, tables, conditions
- **InsertStatement**: INSERT statements with table and values
- **UpdateStatement**: UPDATE statements with table, sets, and conditions
- **DeleteStatement**: DELETE statements with table and conditions
- **CreateTableStatement**: CREATE TABLE statements with schema definitions
- **Expression**: Various expression types (comparison, logical, arithmetic)

## 📊 Performance

- **Streaming**: Parse large SQL files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak SQL integrates seamlessly with:

- **Database Tools**: Build SQL query analyzers and optimizers
- **IDE Support**: Language server protocol compatibility for SQL
- **Migration Tools**: Analyze and transform database schemas
- **Query Builders**: Generate SQL from AST representations
- **Data Analysis**: Extract information from SQL queries

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete SQL statement parsing
- Query analysis and optimization
- Schema extraction and validation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-sql) or open [issues](https://github.com/ygg-lang/oaks/issues).