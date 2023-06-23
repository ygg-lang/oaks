# 🧪 HTML Parser Test Suite & Quality Assurance

`oak-html` features a robust test suite to ensure stability across HTML5 standards and the diverse world of real-world "tag soup."

## 📊 Test Coverage

Our test suite is organized into several core levels to ensure full language compliance and structural accuracy:

### 1. Lexer Tests
Validates the correct identification of all HTML tags, attribute names, attribute values, text nodes, and comments, including support for entities and special characters.
- **Directory**: `tests/lexer/`
- **Mechanism**: Snapshot-based verification comparing against expected token streams.

### 2. Parser Tests
Verifies the structural correctness of the Abstract Syntax Tree (AST), covering:
- **Hierarchical Structure**: Deeply nested elements and correct parent-child relationships.
- **Attributes**: Correct mapping of standard attributes, boolean attributes, and data attributes.
- **Void Elements**: Proper handling of elements like `<img>`, `<br>`, and `<hr>` that don't require closing tags.
- **Foreign Elements**: Validation of SVG and MathML embedding within HTML.
- **Modern Features**: Support for modern HTML5 elements like `<main>`, `<section>`, and `<article>`.
- **Text & Comments**: Accurate capturing of text nodes and comments within the tree.

### 3. Error Recovery
Specifically tests the parser's robustness when facing malformed HTML (e.g., unclosed tags, mismatched tags, or "tag soup" structures), ensuring it can continue parsing subsequent code and produce useful diagnostics while maintaining a valid tree.

### 4. Standards Compliance
Validation against HTML5 standards to ensure the parser correctly handles the latest web features and optional tag rules.

## 🚀 Running Tests

You can run the full test suite using:

```bash
# Run standard Rust tests using cargo
cargo test -p oak-html

# Or use the project-wide task runner
wee test
```

## 📈 Contributing Tests
If you discover an HTML edge case or a particularly messy "tag soup" that doesn't parse correctly, we welcome contributions! Please add a new `.html` file to the `tests/` directory representing the case and submit a PR.
