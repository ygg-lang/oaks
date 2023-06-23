# 🧪 TypeScript Parser Test Suite & Quality Assurance

`oak-typescript` features a comprehensive test suite to ensure stability across TypeScript's complex type system and evolving syntax, from small utilities to massive React/TSX applications.

## 📊 Test Coverage

Our test suite is organized into several core levels to ensure full language compliance and structural accuracy:

### 1. Lexer Tests
Validates the correct identification of all TypeScript keywords, operators, and literals, including support for type-only keywords, decorators, and TSX-specific tokens.
- **Directory**: `tests/lexer/`
- **Mechanism**: Snapshot-based verification comparing against expected token streams.

### 2. Parser Tests
Verifies the structural correctness of the Abstract Syntax Tree (AST), covering:
- **Type System**: Interfaces, type aliases, enums, generics, and union/intersection types.
- **TSX**: Correct parsing of TSX elements, attributes, and type-aware embedded expressions.
- **Decorators**: Validation of both legacy and modern decorator syntax.
- **Modern Features**: `satisfies` operator, `const` type parameters, and `as const` assertions.
- **Modules**: Proper handling of ESM (`import type`, `export type`) and namespaces.
- **Expressions**: Correct precedence and associativity for all TS/JS operators.

### 3. Error Recovery
Specifically tests the parser's robustness when facing malformed TypeScript code (e.g., incomplete type declarations, unmatched angle brackets in generics, or syntax errors in TSX), ensuring it can continue parsing subsequent code and produce useful diagnostics.

### 4. Version & Feature Compliance
Validation against various TypeScript versions and feature sets (e.g., with or without TSX enabled) to ensure the parser correctly handles environment-specific syntax.

## 🚀 Running Tests

You can run the full test suite using:

```bash
# Run standard Rust tests using cargo
cargo test -p oak-typescript

# Or use the project-wide task runner
wee test
```

## 📈 Contributing Tests
If you discover a TypeScript edge case that doesn't parse correctly, we welcome contributions! Please add a new `.ts` or `.tsx` file to the `tests/` directory representing the case and submit a PR.
