# 🧪 C Parser Test Suite & Quality Assurance

`oak-c` features a comprehensive test suite to ensure stability across a wide range of complex C syntax scenarios, from legacy system code to modern C23 features.

## 📊 Test Coverage

Our test suite is organized into several core levels to ensure full language compliance:

### 1. Lexer Tests
Validates the correct identification of all C keywords, operators, and complex literals (including octal, hex, and floating-point constants).
- **Directory**: `tests/lexer/`
- **Mechanism**: Snapshot-based verification comparing against expected token streams.

### 2. Parser Tests
Verifies the structural correctness of the Abstract Syntax Tree (AST), covering:
- **Declarations**: Nested pointers, function pointers, and complex array declarators.
- **Types**: Structs, unions, enums, and typedefs.
- **Control Flow**: Complex `if-else` chains, `for/while` loops, and `switch` statements with `goto` labels.
- **Expressions**: Correct precedence and associativity for all C operators.

### 3. Error Recovery
Specifically tests the parser's robustness when facing malformed C code (e.g., missing semicolons, unmatched braces), ensuring it can continue parsing subsequent code and produce useful diagnostics.

### 4. Standards Compliance
Validation against various C standards (C89, C99, C11) to ensure the parser correctly handles standard-specific features.

## 🚀 Running Tests

You can run the full test suite using:

```bash
# Run standard Rust tests using cargo
cargo test -p oak-c

# Or use the project-wide task runner
wee test
```

## 📈 Contributing Tests
If you discover a C edge case that doesn't parse correctly, we welcome contributions! Please add a new `.c` file to the `tests/` directory representing the case and submit a PR.
