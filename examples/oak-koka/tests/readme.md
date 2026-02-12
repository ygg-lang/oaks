# 🧪 Koka Parser Test Suite

`oak-koka` employs a rigorous testing methodology to ensure accurate parsing across the diverse and evolving Koka ecosystem (JVM, Android, Native, and JS).

## 📊 Test Coverage

### 1. Lexer Snapshot Tests
Ensures correct tokenization of:
- **String Templates**: Handles complex `${...}` expressions and nested templates.
- **Backticked Identifiers**: Support for Koka's ` `identifier` ` syntax, common in testing and interoperability.
- **Numeric Literals**: Binary, hexadecimal, and scientific notation with underscore support.
- **Comments**: Standard C-style, C++-style, and nested comment support.

### 2. Syntax Verification
Validates the structural integrity of the AST for:
- **Modern Features**: Data classes, sealed classes/interfaces, context receivers, and promoted properties.
- **Functional Blocks**: Lambdas with receivers, trailing lambdas, and higher-order functions.
- **Concurrency**: `suspend` functions and coroutine-related syntax patterns.
- **Generics**: Reified type parameters, declaration-site variance (`in`/`out`), and complex constraints.
- **Multiplatform (KMP)**: Robust parsing of `expect` and `actual` declarations.
- **Delegation**: Proper mapping of property delegates (`by lazy`, etc.).

### 3. Error Recovery Tests
Ensures the parser remains stable when facing:
- Unclosed braces in deeply nested class or function bodies.
- Syntax errors within complex expression chains.
- Missing `suspend` keywords in asynchronous contexts.
- Incomplete generic parameter lists.

## 🚀 Running Tests

```bash
# Run all Koka parser tests
cargo test -p oak-koka

# Run specific integration tests
cargo test -p oak-koka --test parser_tests
```

## 📈 Quality Assurance
All contributions must pass the existing test suite and include new tests for any added features (especially new Koka version syntax) or fixed bugs. We strive for high fidelity with the official Koka compiler's parsing behavior.
