# 🧪 Objective-C Parser Test Suite & Quality Assurance

`oak-objective-c` features a comprehensive test suite to ensure stability across Objective-C's unique syntax and its integration with C/C++.

## 📊 Test Coverage

Our test suite is organized into several core levels to ensure full language compliance and structural accuracy:

### 1. Lexer Tests
Validates the correct identification of all Objective-C keywords (including `@`-prefixed ones), operators, and literals, with specific focus on message selectors and mixed C/C++ tokens.
- **Directory**: `tests/lexer/`
- **Mechanism**: Snapshot-based verification comparing against expected token streams.

### 2. Parser Tests
Verifies the structural correctness of the Abstract Syntax Tree (AST), covering:
- **Object-Oriented**: Interfaces (`@interface`), implementations (`@implementation`), categories, and protocols.
- **Message Passing**: Proper parsing of simple and multi-argument message expressions (`[receiver message:arg1 label2:arg2]`).
- **Modern Features**: Properties (`@property`), synthesization (`@synthesize`), and Blocks (`^`).
- **Mixed Syntax**: Robust parsing of Objective-C code combined with C and C++ (for Objective-C++).
- **Memory Management**: Validation of ARC-specific keywords and manual retain/release patterns.
- **Expressions**: Correct precedence and associativity for all Objective-C and inherited C operators.

### 3. Error Recovery
Specifically tests the parser's robustness when facing malformed Objective-C code (e.g., missing `@end` keywords, unmatched brackets in message expressions, or invalid block syntax), ensuring it can continue parsing subsequent code and produce useful diagnostics.

### 4. Version & Runtime Compliance
Validation against various Objective-C versions (1.0 and 2.0) and common Apple-platform coding patterns to ensure the parser correctly handles environment-specific syntax.

## 🚀 Running Tests

You can run the full test suite using:

```bash
# Run standard Rust tests using cargo
cargo test -p oak-objective-c

# Or use the project-wide task runner
wee test
```

## 📈 Contributing Tests
If you discover an Objective-C edge case that doesn't parse correctly, we welcome contributions! Please add a new `.m` or `.mm` file to the `tests/` directory representing the case and submit a PR.
