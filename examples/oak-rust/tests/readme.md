# 🧪 Rust Parser Test Suite & Quality Assurance

`oak-rust` features a comprehensive test suite to ensure stability across Rust's powerful and evolving syntax, from low-level systems code to high-level async applications.

## 📊 Test Coverage

Our test suite is organized into several core levels to ensure full language compliance and structural accuracy:

### 1. Lexer Tests
Validates the correct identification of all Rust keywords, operators, and literals, including support for raw strings, byte strings, and complex numeric suffixes.
- **Directory**: `tests/lexer/`
- **Mechanism**: Snapshot-based verification comparing against expected token streams.

### 2. Parser Tests
Verifies the structural correctness of the Abstract Syntax Tree (AST), covering:
- **Ownership & Lifetimes**: Explicit lifetime annotations, borrowing, and ownership transitions.
- **Generics & Traits**: Trait definitions, implementations, and complex generic bounds.
- **Macros**: Declarative `macro_rules!` and various forms of procedural macros.
- **Pattern Matching**: Complex `match` arms, destructuring, and pattern guards.
- **Modern Features**: `async/await`, `try` blocks, and structured bindings.
- **Expressions**: Correct precedence and associativity for all Rust operators.

### 3. Error Recovery
Specifically tests the parser's robustness when facing malformed Rust code (e.g., missing semicolons, unmatched braces, or incomplete generic parameters), ensuring it can continue parsing subsequent code and produce useful diagnostics.

### 4. Edition Compliance
Validation against various Rust editions (2015, 2018, 2021, 2024) to ensure the parser correctly handles edition-specific syntax and keyword changes.

## 🚀 Running Tests

You can run the full test suite using:

```bash
# Run standard Rust tests using cargo
cargo test -p oak-rust

# Or use the project-wide task runner
wee test
```

## 📈 Contributing Tests
If you discover a Rust edge case that doesn't parse correctly, we welcome contributions! Please add a new `.rs` file to the `tests/` directory representing the case and submit a PR.
