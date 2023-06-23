# 🧪 Test Suite & Quality Assurance

`oak-actionscript` features a comprehensive test suite to ensure stability across a wide range of complex AS3 syntax scenarios.

## 📊 Test Coverage

Our test suite is organized into three core levels:

### 1. Lexer Tests
Validates the correct identification of all ActionScript 3.0 keywords, operators, literals (including multi-line strings), and regular expressions.
- **Directory**: `tests/lexer/`
- **Mechanism**: Snapshot-based verification comparing against expected token streams.

### 2. Parser Tests
Verifies the structural correctness of the Abstract Syntax Tree (AST), covering:
- Packages & Namespaces
- Class & Interface Inheritance
- E4X (ECMAScript for XML) syntax support
- Complex expression precedence

### 3. Error Recovery
Specifically tests the parser's robustness when facing malformed or incomplete code, ensuring it can continue parsing subsequent code and produce useful diagnostics.

## 🚀 Running Tests

You can run the full test suite using:

```bash
# Run standard Rust tests using cargo
cargo test

# Or use the built-in project tool (if installed)
wee test
```

## 📈 Contributing Tests
If you discover an edge case that doesn't parse correctly, we welcome contributions! Please add a new `.as` file to the `tests/` directory and submit a PR.
