# 🧪 JSON Parser Test Suite & Quality Assurance

`oak-json` features a comprehensive test suite to ensure stability across standard JSON and various common extensions like JSON5.

## 📊 Test Coverage

Our test suite is organized into several core levels to ensure full language compliance and structural accuracy:

### 1. Lexer Tests
Validates the correct identification of all JSON tokens, including strings (with escape sequences), numbers (integers, floats, and scientific notation), and structural characters.
- **Directory**: `tests/lexer/`
- **Mechanism**: Snapshot-based verification comparing against expected token streams.

### 2. Parser Tests
Verifies the structural correctness of the Abstract Syntax Tree (AST), covering:
- **Core Structures**: Objects, Arrays, and deeply nested combinations.
- **Value Types**: Strings, Numbers, Booleans, and Null.
- **Extensions**: Trailing commas, bare keys, single quotes, and comments (when configured via `JsonLanguage`).

### 3. Error Recovery
Specifically tests the parser's robustness when facing malformed JSON code (e.g., missing colons, unmatched braces, or invalid values), ensuring it can continue parsing subsequent data and produce useful diagnostics.

### 4. JSON5 Compliance
Validation against the JSON5 specification to ensure that extended features are handled correctly when enabled in the configuration.

## 🚀 Running Tests

You can run the full test suite using:

```bash
# Run standard Rust tests using cargo
cargo test -p oak-json

# Or use the project-wide task runner
wee test
```

## 📈 Contributing Tests
If you discover a JSON edge case or a valid JSON5 snippet that doesn't parse correctly, we welcome contributions! Please add a new `.json` (or `.json5`) file to the `tests/` directory representing the case and submit a PR.
