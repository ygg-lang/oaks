# 🧪 XML Parser Test Suite & Quality Assurance

`oak-xml` features a comprehensive test suite to ensure stability across complex XML documents and various markup standards.

## 📊 Test Coverage

Our test suite is organized into several core levels to ensure full language compliance and structural accuracy:

### 1. Lexer Tests
Validates the correct identification of all XML tokens, including tag delimiters, attribute names/values, character data, and entities.
- **Directory**: `tests/lexer/`
- **Mechanism**: Snapshot-based verification comparing against expected token streams.

### 2. Parser Tests
Verifies the structural correctness of the Abstract Syntax Tree (AST), covering:
- **Elements**: Proper parsing of start, end, and self-closing tags, along with nested structures.
- **Attributes**: Correct mapping of attribute-value pairs within elements.
- **Content**: Handling of text content, CDATA sections, and entities.
- **Namespaces**: Validation of namespace declarations and prefix usage.

### 3. Error Recovery
Specifically tests the parser's robustness when facing malformed XML (e.g., unclosed tags, mismatched tag names, or invalid attribute quoting), ensuring it can continue parsing subsequent elements and produce useful diagnostics.

### 4. Serialization (Serde) Tests
Ensures that the XML structures can be correctly serialized and deserialized using `serde`, enabling seamless integration with data processing workflows.

## 🚀 Running Tests

You can run the full test suite using:

```bash
# Run standard Rust tests using cargo
cargo test -p oak-xml

# Or use the project-wide task runner
wee test
```

## 📈 Contributing Tests
If you discover an XML edge case or a complex document that doesn't parse correctly, we welcome contributions! Please add a new `.xml` file to the `tests/` directory representing the case and submit a PR.
