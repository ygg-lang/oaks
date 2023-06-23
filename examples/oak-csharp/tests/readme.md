# 🧪 C# Parser Test Suite

`oak-csharp` employs a rigorous testing methodology to ensure accurate parsing across the diverse and evolving C# ecosystem.

## 📊 Test Coverage

### 1. Lexer Snapshot Tests
Ensures correct tokenization of:
- **String Literals**: Verbatim strings (`@""`), interpolated strings (`$""`), and modern raw string literals (`"""..."""`).
- **Numeric Literals**: Binary, hexadecimal, and scientific notation with underscore support.
- **Keywords**: Full support for contextual keywords like `var`, `partial`, `yield`, and `global`.
- **Comments**: Standard C-style, C++-style, and XML documentation comments.

### 2. Syntax Verification
Validates the structural integrity of the AST for:
- **Modern Features**: Records, pattern matching (switch expressions, property patterns), init-only properties, and primary constructors.
- **Asynchronous Flow**: `async`/`await` patterns and `await foreach` constructs.
- **LINQ**: Query expressions with `from`, `where`, `select`, `group`, and `join` clauses.
- **OOP Structures**: Classes, Structs, Interfaces, Enums, and complex inheritance/implementation hierarchies.
- **Attributes**: Robust parsing of global, class-level, and member-level attributes.
- **Namespaces**: Support for file-scoped namespaces and nested namespace declarations.

### 3. Error Recovery Tests
Ensures the parser remains stable when facing:
- Unclosed braces in deeply nested class or method bodies.
- Syntax errors within complex LINQ query chains.
- Missing `await` keywords in asynchronous contexts.
- Incomplete record or primary constructor definitions.

## 🚀 Running Tests

```bash
# Run all C# parser tests
cargo test -p oak-csharp

# Run specific integration tests
cargo test -p oak-csharp --test parser_tests
```

## 📈 Quality Assurance
All contributions must pass the existing test suite and include new tests for any added features (especially new C# version syntax) or fixed bugs. We strive for high fidelity with the official Roslyn parser's behavior.
