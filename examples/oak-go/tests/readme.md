# 🧪 Go Parser Test Suite & Quality Assurance

`oak-go` features a robust test suite to ensure stability across Go's clean and efficient syntax, from simple scripts to complex microservices.

## 📊 Test Coverage

Our test suite is organized into several core levels to ensure full language compliance and structural accuracy:

### 1. Lexer Tests
Validates the correct identification of all Go keywords, operators, and literals, including support for backtick strings and complex number formats.
- **Directory**: `tests/lexer/`
- **Mechanism**: Snapshot-based verification comparing against expected token streams.

### 2. Parser Tests
Verifies the structural correctness of the Abstract Syntax Tree (AST), covering:
- **Declarations**: Packages, imports, structs, and interfaces.
- **Concurrency**: Goroutines, channels, and select statements.
- **Methods**: Function signatures, method receivers, and multiple return values.
- **Generics**: Type parameters and constraints (for Go 1.18+).
- **Control Flow**: `if` with initialization, `for-range` loops, and `defer/panic/recover`.

### 3. Error Recovery
Specifically tests the parser's robustness when facing malformed Go code (e.g., missing return types, unmatched braces), ensuring it can continue parsing subsequent code and produce useful diagnostics.

### 4. Version Compliance
Validation against various Go versions to ensure the parser correctly handles version-specific syntax, especially the introduction of generics in Go 1.18.

## 🚀 Running Tests

You can run the full test suite using:

```bash
# Run standard Rust tests using cargo
cargo test -p oak-go

# Or use the project-wide task runner
wee test
```

## 📈 Contributing Tests
If you discover a Go edge case that doesn't parse correctly, we welcome contributions! Please add a new `.go` file to the `tests/` directory representing the case and submit a PR.
