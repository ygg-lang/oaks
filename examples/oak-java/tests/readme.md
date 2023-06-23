# 🧪 Java Parser Test Suite & Quality Assurance

`oak-java` features a comprehensive test suite to ensure stability across Java's structured and evolving syntax, from legacy enterprise code to modern Java 21+ features.

## 📊 Test Coverage

Our test suite is organized into several core levels to ensure full language compliance and structural accuracy:

### 1. Lexer Tests
Validates the correct identification of all Java keywords, operators, and literals, including support for Unicode identifiers and complex numeric literals.
- **Directory**: `tests/lexer/`
- **Mechanism**: Snapshot-based verification comparing against expected token streams.

### 2. Parser Tests
Verifies the structural correctness of the Abstract Syntax Tree (AST), covering:
- **Object-Oriented**: Classes, interfaces, inheritance, and access modifiers.
- **Modern Features**: Records, sealed classes, and pattern matching for `switch`.
- **Generics**: Complex generic type declarations and bounds.
- **Annotations**: Parsing of built-in and custom annotations on various elements.
- **Control Flow**: `if-else`, `for/while` loops, and modern `switch` expressions.
- **Expressions**: Correct precedence and associativity for all Java operators.

### 3. Error Recovery
Specifically tests the parser's robustness when facing malformed Java code (e.g., missing semicolons, unmatched braces, or incomplete record declarations), ensuring it can continue parsing subsequent code and produce useful diagnostics.

### 4. Version Compliance
Validation against various Java versions (8, 11, 17, 21) to ensure the parser correctly handles version-specific syntax and keyword changes.

## 🚀 Running Tests

You can run the full test suite using:

```bash
# Run standard Rust tests using cargo
cargo test -p oak-java

# Or use the project-wide task runner
wee test
```

## 📈 Contributing Tests
If you discover a Java edge case that doesn't parse correctly, we welcome contributions! Please add a new `.java` file to the `tests/` directory representing the case and submit a PR.
