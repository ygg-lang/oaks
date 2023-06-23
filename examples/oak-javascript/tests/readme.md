# 🧪 JavaScript Parser Test Suite & Quality Assurance

`oak-javascript` features a comprehensive test suite to ensure stability across JavaScript's dynamic and rapidly evolving syntax, from simple scripts to complex React applications.

## 📊 Test Coverage

Our test suite is organized into several core levels to ensure full language compliance and structural accuracy:

### 1. Lexer Tests
Validates the correct identification of all JavaScript keywords, operators, and literals, including support for template strings, regex literals, and JSX-specific tokens.
- **Directory**: `tests/lexer/`
- **Mechanism**: Snapshot-based verification comparing against expected token streams.

### 2. Parser Tests
Verifies the structural correctness of the Abstract Syntax Tree (AST), covering:
- **Modern ECMAScript**: Classes, arrow functions, async/await, and destructuring.
- **JSX**: Correct parsing of JSX elements, attributes, and embedded expressions.
- **Modules**: Proper handling of ESM (`import`/`export`) and legacy CommonJS.
- **Scoping**: Correct identification of block and function scopes.
- **ASI**: Validation of Automatic Semicolon Insertion rules across various edge cases.
- **Expressions**: Correct precedence and associativity for all JavaScript operators.

### 3. Error Recovery
Specifically tests the parser's robustness when facing malformed JavaScript code (e.g., missing closing braces, incomplete JSX tags, or syntax errors in async blocks), ensuring it can continue parsing subsequent code and produce useful diagnostics.

### 4. Version & Feature Compliance
Validation against various ECMAScript versions and feature sets (e.g., with or without JSX enabled) to ensure the parser correctly handles environment-specific syntax.

## 🚀 Running Tests

You can run the full test suite using:

```bash
# Run standard Rust tests using cargo
cargo test -p oak-javascript

# Or use the project-wide task runner
wee test
```

## 📈 Contributing Tests
If you discover a JavaScript edge case that doesn't parse correctly, we welcome contributions! Please add a new `.js` or `.jsx` file to the `tests/` directory representing the case and submit a PR.
