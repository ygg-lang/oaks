# 🧪 C++ Parser Test Suite & Quality Assurance

`oak-cpp` features a comprehensive test suite to ensure stability across the vast and complex landscape of C++, from foundational features to the latest C++23 standards.

## 📊 Test Coverage

Our test suite is organized into several core levels to ensure full language compliance and structural accuracy:

### 1. Lexer Tests
Validates the correct identification of all C++ keywords, operators, and literals, including support for user-defined literals and raw strings.
- **Directory**: `tests/lexer/`
- **Mechanism**: Snapshot-based verification comparing against expected token streams.

### 2. Parser Tests
Verifies the structural correctness of the Abstract Syntax Tree (AST), covering:
- **Object-Oriented**: Classes, structs, unions, inheritance, and access control.
- **Templates**: Function and class templates, variadic templates, and concepts.
- **Namespaces**: Nested namespaces, using-declarations, and inline namespaces.
- **Modern Features**: Lambdas, `auto`, structured bindings, and modules.
- **Expressions**: Correct precedence and associativity for all C++ operators, including those with complex syntax like `::` and `.*`.

### 3. Error Recovery
Specifically tests the parser's robustness when facing malformed C++ code (e.g., missing semicolons, unmatched template brackets), ensuring it can continue parsing subsequent code and produce useful diagnostics.

### 4. Standards Compliance
Validation against various C++ standards (C++11, C++17, C++20, C++23) to ensure the parser correctly handles standard-specific syntax and semantics.

## 🚀 Running Tests

You can run the full test suite using:

```bash
# Run standard Rust tests using cargo
cargo test -p oak-cpp

# Or use the project-wide task runner
wee test
```

## 📈 Contributing Tests
If you discover a C++ edge case that doesn't parse correctly, we welcome contributions! Please add a new `.cpp` file to the `tests/` directory representing the case and submit a PR.
