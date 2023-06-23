# 🧪 Swift Parser Test Suite

`oak-swift` employs a comprehensive testing methodology to ensure accurate parsing of Swift's sophisticated and rapidly evolving syntax.

## 📊 Test Coverage

### 1. Lexer Snapshot Tests
Ensures correct tokenization of:
- **String Interpolation**: Complex, multiline strings with nested expressions.
- **Custom Operators**: Handles Swift's unique ability to define custom prefix, infix, and postfix operators.
- **Numeric Literals**: Binary, octal, hexadecimal, and scientific notation with underscores for readability.
- **Keywords**: Full support for context-specific keywords (e.g., `set`, `get`, `willSet`).

### 2. Syntax Verification
Validates the structural integrity of the AST for:
- **Modern Concurrency**: `async`/`await`, `actors`, and structured concurrency patterns.
- **Generics**: Complex generic constraints, variadic generics, and opaque return types.
- **Result Builders**: Proper parsing of SwiftUI-style declarative blocks.
- **Property Wrappers**: Support for various property wrapper application styles.
- **Enums & Pattern Matching**: Exhaustive testing of associated values and complex `switch` cases.
- **Macros**: Support for both freestanding and attached macro syntax.

### 3. Error Recovery Tests
Ensures the parser remains stable when facing:
- Unclosed braces in deeply nested structures.
- Missing `await` keywords in asynchronous contexts.
- Syntax errors within SwiftUI view bodies.
- Incomplete generic parameter lists.

## 🚀 Running Tests

```bash
# Run all Swift parser tests
cargo test -p oak-swift

# Run specific integration tests
cargo test -p oak-swift --test parser_tests
```

## 📈 Quality Assurance
All contributions must pass the existing test suite and include new tests for any added features (especially new Swift Evolution syntax) or fixed bugs. We aim for high fidelity with the official Swift compiler's parsing behavior.
