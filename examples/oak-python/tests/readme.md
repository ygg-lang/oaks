# 🧪 Python Parser Test Suite & Quality Assurance

`oak-python` features a comprehensive test suite to ensure stability across Python's clean and indentation-sensitive syntax, from simple scripts to complex async applications.

## 📊 Test Coverage

Our test suite is organized into several core levels to ensure full language compliance and structural accuracy:

### 1. Lexer Tests
Validates the correct identification of all Python keywords, operators, and literals, including support for f-strings, complex number formats, and indentation/dedentation tokens.
- **Directory**: `tests/lexer/`
- **Mechanism**: Snapshot-based verification comparing against expected token streams.

### 2. Parser Tests
Verifies the structural correctness of the Abstract Syntax Tree (AST), covering:
- **Scoping**: Correct identification of block scopes based on indentation levels.
- **Types**: Full support for PEP 484 type hints and modern type syntax.
- **Object-Oriented**: Classes, multiple inheritance, and method decorators.
- **Modern Features**: Structural pattern matching (`match/case`), `async/await`, and Walrus operator (`:=`).
- **Expressions**: Correct precedence and associativity for all Python operators, including power (`**`) and matrix multiplication (`@`).

### 3. Error Recovery
Specifically tests the parser's robustness when facing malformed Python code (e.g., indentation errors, missing colons, or unmatched brackets), ensuring it can continue parsing subsequent code and produce useful diagnostics.

### 4. Version Compliance
Validation against various Python versions (3.8, 3.10, 3.12) to ensure the parser correctly handles version-specific syntax and keyword changes.

## 🚀 Running Tests

You can run the full test suite using:

```bash
# Run standard Rust tests using cargo
cargo test -p oak-python

# Or use the project-wide task runner
wee test
```

## 📈 Contributing Tests
If you discover a Python edge case that doesn't parse correctly, we welcome contributions! Please add a new `.py` file to the `tests/` directory representing the case and submit a PR.
