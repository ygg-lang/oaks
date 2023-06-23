# 🛠️ SQL Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-sql`.

## 🚦 Quick Start

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
oak-sql = { path = "..." }
```

### Basic Parsing Example

The following is a standard workflow for parsing a complex SQL query with Joins and CTEs:

```rust
use oak_sql::{SqlParser, SourceText, SqlLanguage};

fn main() {
    // 1. Prepare source code (PostgreSQL dialect)
    let code = r#"
        WITH regional_sales AS (
            SELECT region, SUM(amount) AS total_sales
            FROM orders
            GROUP BY region
        )
        SELECT product_name, total_sales
        FROM regional_sales
        JOIN products ON products.region = regional_sales.region
        WHERE total_sales > 10000
        ORDER BY total_sales DESC;
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser with specific dialect configuration
    let config = SqlLanguage::postgresql();
    let parser = SqlParser::new(&config);

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
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract SQL-specific constructs like CTE definitions, Join conditions, `WHERE` clauses, or DDL table structures.

### 2. Incremental Parsing
No need to re-parse an entire multi-megabyte SQL dump when small changes occur:
```rust
// Assuming you have an old parse result 'old_result' and new source text 'new_source'
let new_result = parser.reparse(&new_source, &old_result);
```

### 3. Diagnostics
`oak-sql` provides rich error contexts specifically tailored for database developers, handling dialect-specific syntax quirks and providing clear feedback on malformed queries:
```rust
for diag in result.diagnostics() {
    println!("[{}:{}] {}", diag.line, diag.column, diag.message);
}
```

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes SQL source text into a stream of tokens, handling keywords, operators, quoted identifiers, and dialect-specific literals.
- **Parser**: Syntax analyzer based on the Pratt parsing algorithm to handle SQL's complex expression precedence, multi-dialect support, and deeply nested query structures.
- **AST**: A strongly-typed syntax abstraction layer designed for high-performance SQL analysis tools, query formatters, and IDEs.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/](tests/) for handling of various SQL dialects and edge cases.
