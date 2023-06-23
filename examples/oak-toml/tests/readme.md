# 🧪 TOML Parser Test Suite & Quality Assurance

`oak-toml` features a comprehensive test suite to ensure stability across various TOML features and edge cases.

## 📊 Test Coverage

Our test suite is organized into several core levels to ensure full language compliance and structural accuracy:

### 1. Lexer Tests
Validates the correct identification of all TOML tokens, including keys (simple, quoted, dotted), strings (basic, multiline, literal), numbers, and dates.
- **Directory**: `tests/lexer/`
- **Mechanism**: Snapshot-based verification comparing against expected token streams.

### 2. Parser Tests
Verifies the structural correctness of the Abstract Syntax Tree (AST), covering:
- **Tables**: Standard tables, nested tables, inline tables, and arrays of tables.
- **Key-Value Pairs**: Correct parsing of complex key structures and all TOML value types.
- **Data Types**: Native support for integers, floats, booleans, and RFC3339 date-times.

### 3. Error Recovery
Specifically tests the parser's robustness when facing malformed TOML (e.g., re-defined keys, unclosed strings, or invalid table headers), ensuring it can continue parsing subsequent sections and produce useful diagnostics.

### 4. Specification Compliance
Validation against the TOML specification to ensure that the parser correctly handles standard-specific rules and edge cases.

## 🚀 Running Tests

You can run the full test suite using:

```bash
# Run standard Rust tests using cargo
cargo test -p oak-toml

# Or use the project-wide task runner
wee test
```

## 📈 Contributing Tests
If you discover a TOML edge case or a valid configuration snippet that doesn't parse correctly, we welcome contributions! Please add a new `.toml` file to the `tests/` directory representing the case and submit a PR.
