# 🧪 CSS Parser Test Suite & Quality Assurance

`oak-css` features a comprehensive test suite to ensure stability across modern CSS modules and diverse browser-specific syntax.

## 📊 Test Coverage

Our test suite is organized into several core levels to ensure full language compliance and structural accuracy:

### 1. Lexer Tests
Validates the correct identification of all CSS tokens, including identifiers, numbers with units, strings, and special characters used in complex selectors.
- **Directory**: `tests/lexer/`
- **Mechanism**: Snapshot-based verification comparing against expected token streams.

### 2. Parser Tests
Verifies the structural correctness of the Abstract Syntax Tree (AST), covering:
- **Selectors**: Simple, compound, and complex selectors, including pseudo-classes and pseudo-elements.
- **Declarations**: Correct mapping of property names and values, including complex functions like `calc()`.
- **At-Rules**: Validation of `@media`, `@keyframes`, `@import`, and other standard at-rules.
- **Variables**: Robust parsing of CSS Custom Properties and their usage via `var()`.
- **Nesting**: Proper handling of the CSS Nesting module syntax.
- **Modern Layouts**: Support for Grid, Flexbox, and other modern layout property sets.

### 3. Error Recovery
Specifically tests the parser's robustness when facing malformed CSS (e.g., missing semicolons, unmatched braces, or invalid selector syntax), ensuring it can continue parsing subsequent rules and produce useful diagnostics.

### 4. Browser & Feature Compliance
Validation against various CSS specifications and common browser-specific hacks to ensure the parser correctly handles the real-world web design landscape.

## 🚀 Running Tests

You can run the full test suite using:

```bash
# Run standard Rust tests using cargo
cargo test -p oak-css

# Or use the project-wide task runner
wee test
```

## 📈 Contributing Tests
If you discover a CSS edge case or a modern property that doesn't parse correctly, we welcome contributions! Please add a new `.css` file to the `tests/` directory representing the case and submit a PR.
