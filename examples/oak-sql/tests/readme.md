# 🧪 SQL Parser Test Suite & Quality Assurance

`oak-sql` features a comprehensive test suite to ensure stability across multiple SQL dialects and complex query structures.

## 📊 Test Coverage

Our test suite is organized into several core levels to ensure full language compliance and structural accuracy:

### 1. Lexer Tests
Validates the correct identification of all SQL keywords, operators, and literals, including support for dialect-specific quoting styles and string escaping.
- **Directory**: `tests/lexer/`
- **Mechanism**: Snapshot-based verification comparing against expected token streams.

### 2. Parser Tests
Verifies the structural correctness of the Abstract Syntax Tree (AST), covering:
- **DML**: `SELECT` (including Joins, CTEs, Window Functions), `INSERT`, `UPDATE`, `DELETE`.
- **DDL**: `CREATE`, `ALTER`, `DROP` for tables, indexes, and views across different dialects.
- **Dialect Specifics**: MySQL-specific syntax (e.g., `ON DUPLICATE KEY UPDATE`), PostgreSQL-specific types and operators, and SQLite's unique features.
- **Procedural Logic**: Parsing of stored procedures and triggers (where supported by the dialect).
- **Expressions**: Correct precedence and associativity for all SQL operators.

### 3. Error Recovery
Specifically tests the parser's robustness when facing malformed SQL code (e.g., missing commas in `SELECT` lists, unmatched parentheses in subqueries, or incomplete DDL statements), ensuring it can continue parsing subsequent code and produce useful diagnostics.

### 4. Dialect Compliance
Validation against various SQL dialects (ANSI, MySQL, PostgreSQL, SQLite) to ensure the parser correctly handles dialect-specific syntax and keywords.

## 🚀 Running Tests

You can run the full test suite using:

```bash
# Run standard Rust tests using cargo
cargo test -p oak-sql

# Or use the project-wide task runner
wee test
```

## 📈 Contributing Tests
If you discover a SQL edge case or a dialect-specific query that doesn't parse correctly, we welcome contributions! Please add a new `.sql` file to the `tests/` directory representing the case and submit a PR.
