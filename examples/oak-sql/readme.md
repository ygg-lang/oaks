# Oak SQL Parser

[![Crates.io](https://img.shields.io/crates/v/oak-sql.svg)](https://crates.io/crates/oak-sql)
[![Documentation](https://docs.rs/oak-sql/badge.svg)](https://docs.rs/oak-sql)

High-performance incremental SQL parser for the oak ecosystem with flexible configuration, optimized for database query analysis and SQL processing.

## 🎯 Overview

Oak SQL is a robust parser for SQL, designed to handle complete SQL syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for database query analysis and SQL processing.

## ✨ Features

- **Complete SQL Syntax**: Supports all SQL features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_sql::{Parser, SqlLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
        SELECT u.id, u.name, p.title
        FROM users u
        JOIN posts p ON u.id = p.user_id
        WHERE u.active = true
        ORDER BY u.created_at DESC
        LIMIT 10;
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed SQL successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Select Statement Parsing
```rust
use oak_sql::{Parser, SqlLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
    SELECT id, name, email
    FROM customers
    WHERE country = 'USA' AND age > 21
    ORDER BY name ASC
    LIMIT 100;
"#);

let result = parser.parse(&source);
println!("Select statement parsed successfully.");
```

### Insert Statement Parsing
```rust
use oak_sql::{Parser, SqlLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
    INSERT INTO products (name, price, category)
    VALUES ('Laptop', 999.99, 'Electronics');
"#);

let result = parser.parse(&source);
println!("Insert statement parsed successfully.");
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