# 🧪 PHP Parser Test Suite

`oak-php` employs a robust testing methodology to ensure accurate parsing across the vast and evolving PHP ecosystem.

## 📊 Test Coverage

### 1. Lexer Snapshot Tests
Ensures correct tokenization of:
- **Complex Strings**: Deeply nested variable interpolation in double quotes and heredocs.
- **Modern Literals**: Binary, octal (including PHP 8.1+ `0o` notation), and hexadecimal integers.
- **Heredoc/Nowdoc**: Full support for various closing marker styles and indentation.
- **Comments**: Standard C-style, C++-style, and Shell-style (`#`) comments.

### 2. Syntax Verification
Validates the structural integrity of the AST for:
- **PHP 8.x Features**: Attributes, promoted properties, enums, readonly classes, and nullsafe operators.
- **OOP Structures**: Classes, Interfaces, Traits, and complex inheritance hierarchies.
- **Namespace Logic**: Correct parsing of nested namespaces and grouped `use` statements.
- **Mixed HTML/PHP**: Robust handling of files where PHP is embedded within HTML.
- **Error Handling**: `try-catch-finally` blocks and modern `throw` expressions.

### 3. Error Recovery Tests
Ensures the parser remains functional when facing:
- Unclosed braces in class or function definitions.
- Syntax errors within complex expression chains.
- Missing semicolons or malformed attributes.

## 🚀 Running Tests

```bash
# Run all PHP parser tests
cargo test -p oak-php

# Run specific integration tests
cargo test -p oak-php --test parser_tests
```

## 📈 Quality Assurance
All contributions must pass the existing test suite and include new tests for any added features (especially new PHP version syntax) or fixed bugs. We aim for full parity with the official PHP parser's behavior.
