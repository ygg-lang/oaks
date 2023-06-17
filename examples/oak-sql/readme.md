# Oak SQL Parser

[![Crates.io](https://img.shields.io/crates/v/oak-sql.svg)](https://crates.io/crates/oak-sql)
[![Documentation](https://docs.rs/oak-sql/badge.svg)](https://docs.rs/oak-sql)

High-performance incremental SQL parser for the oak ecosystem with flexible configuration, optimized for database query analysis and SQL processing.

## 🎯 Overview

Oak of sql is a robust parser for SQL, designed to handle complete SQL syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for database query analysis and SQL processing.

## ✨ Features

- **Complete SQL Syntax**: Supports all SQL features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_sql::SqlParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = SqlParser::new();
    let sql_query = r#"
        SELECT u.id, u.name, p.title
        FROM users u
        JOIN posts p ON u.id = p.user_id
        WHERE u.active = true
        ORDER BY u.created_at DESC
        LIMIT 10;
    "#;
    
    let statement = parser.parse_statement(sql_query)?;
    println!("Parsed SQL statement successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Select Statement Parsing
```rust
use oak_sql::{SqlParser, ast::SelectStatement};

let parser = SqlParser::new();
let sql_query = r#"
    SELECT id, name, email
    FROM customers
    WHERE country = 'USA' AND age > 21
    ORDER BY name ASC
    LIMIT 100;
"#;

let select = parser.parse_select(sql_query)?;
println!("Selected columns: {}", select.columns.len());
println!("From table: {}", select.from_table);
```

### Insert Statement Parsing
```rust
use oak_sql::{SqlParser, ast::InsertStatement};

let parser = SqlParser::new();
let insert_query = r#"
    INSERT INTO products (name, price, category)
    VALUES ('Laptop', 999.99, 'Electronics');
"#;

let insert = parser.parse_insert(insert_query)?;
println!("Target table: {}", insert.table);
println!("Values: {}", insert.values.len());
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_sql::{SqlParser, lexer::Token};

let parser = SqlParser::new();
let tokens = parser.tokenize("SELECT * FROM users WHERE id = 1;")?;
for token in tokens {
    println!("{:?}", token.kind);
}
```

### Error Handling
```rust
use oak_sql::SqlParser;

let parser = SqlParser::new();
let invalid_sql = r#"
    SELECT name, 
    FROM users
    WHERE id = 1;
"#;

match parser.parse_statement(invalid_sql) {
    Ok(statement) => println!("Parsed SQL statement successfully."),
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

Oak of sql integrates seamlessly with:

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