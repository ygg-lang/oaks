# 🧪 YAML Parser Test Suite & Quality Assurance

`oak-yaml` features a comprehensive test suite to ensure stability across complex YAML documents and features.

## 📊 Test Coverage

Our test suite is organized into several core levels to ensure full language compliance and structural accuracy:

### 1. Lexer Tests
Validates the correct identification of all YAML tokens, including indentation markers, block/flow style indicators, and various scalar formats.
- **Directory**: `tests/lexer/`
- **Mechanism**: Snapshot-based verification comparing against expected token streams.

### 2. Parser Tests
Verifies the structural correctness of the Abstract Syntax Tree (AST), covering:
- **Mappings & Sequences**: Nested structures in both block and flow styles.
- **Scalars**: Proper parsing of plain, single-quoted, double-quoted, literal, and folded scalars.
- **Anchors & Aliases**: Correct identification and structural mapping of references.
- **Document Markers**: Handling of `---` and `...` document separators.

### 3. Error Recovery
Specifically tests the parser's robustness when facing malformed YAML (e.g., inconsistent indentation, unmatched brackets in flow style), ensuring it can continue parsing subsequent sections and produce useful diagnostics.

### 4. Whitespace Sensitivity
Intensive testing of indentation rules to ensure the parser correctly identifies block levels and hierarchical relationships.

## 🚀 Running Tests

You can run the full test suite using:

```bash
# Run standard Rust tests using cargo
cargo test -p oak-yaml

# Or use the project-wide task runner
wee test
```

## 📈 Contributing Tests
If you discover a YAML edge case or a complex document that doesn't parse correctly, we welcome contributions! Please add a new `.yaml` file to the `tests/` directory representing the case and submit a PR.
