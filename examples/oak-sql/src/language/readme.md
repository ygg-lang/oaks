# SQL Language Definition

This module contains the metadata and configuration options for the SQL language within the Oak framework.

## ⚙️ Configuration

The `SqlLanguage` struct defines how the parser and lexer should behave to accommodate various SQL dialects and standards:

```rust
pub struct SqlLanguage {
    /// Whether it is case sensitive.
    pub case_sensitive: bool,
    /// Whether to allow double-quoted identifiers.
    pub quoted_identifiers: bool,
    /// Whether to allow backtick identifiers.
    pub backtick_identifiers: bool,
    /// Whether to allow bracket identifiers.
    pub bracket_identifiers: bool,
}
```

### Predefined Configurations

`SqlLanguage` provides several helper methods to create configurations for common SQL dialects:

- **`standard()`**: Standard SQL configuration (Case-insensitive, double-quoted identifiers).
- **`mysql()`**: MySQL-style configuration (Backtick identifiers enabled).
- **`postgresql()`**: PostgreSQL-style configuration (Double-quoted identifiers).
- **`sqlserver()`**: SQL Server-style configuration (Bracket identifiers enabled).

## 🧩 Oak Integration

By implementing the `Language` trait, this module provides the glue that connects:
- **`TokenType`**: Defined in the `lexer` module, covering all SQL keywords, operators, and dialect-specific literals.
- **`ElementType`**: Defined in the `parser` module, representing SQL-specific syntax structures like `SELECT` statements, `JOIN` clauses, and DDL commands.
- **`TypedRoot`**: Defined in the `ast` module, providing a strongly-typed view of the SQL source file.

This centralization allows the Oak framework to handle generic tasks like incremental parsing and LSP support while remaining deeply aware of SQL's unique syntax and diverse dialect ecosystem.
