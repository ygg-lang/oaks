# 🧪 Zig Parser Test Suite

`oak-zig` utilizes a multi-layered testing strategy to ensure high fidelity parsing across various Zig versions and language features.

## 📊 Test Coverage

### 1. Lexer Snapshot Tests
Ensures tokens are correctly identified, including Zig-specific edge cases:
- **String Literals**: Multi-line strings, character escapes, and byte strings.
- **Keywords & Built-ins**: Handling of `@import`, `@as`, and other compiler intrinsics.
- **Numeric Literals**: Underscores in numbers and various bases (hex, binary, octal).

### 2. Syntax Verification
Validates the structural integrity of the AST for:
- **Comptime Logic**: Verification of `comptime` blocks, functions, and variables.
- **Error Handling**: Proper parsing of `try`, `catch`, `errdefer`, and error sets.
- **Memory Management**: Support for `allocator` patterns and pointer syntax.
- **Generics**: Handling of type parameters and generic struct/function definitions.

### 3. Error Recovery Tests
Verifies that the parser remains stable when encountering:
- Missing semicolons in block statements.
- Incomplete struct or enum definitions.
- Syntax errors inside `comptime` expressions.

## 🚀 Running Tests

```bash
# Run all Zig parser tests
cargo test -p oak-zig

# Run specific lexer tests
cargo test -p oak-zig --test lexer_tests
```

## 📈 Quality Standards
Every pull request must pass the full test suite and maintain snapshot stability. For new language features or bug fixes, corresponding tests should be added to the `tests/` directory.
