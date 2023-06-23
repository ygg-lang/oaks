# 🧪 HLSL Parser Test Suite

`oak-hlsl` utilizes a multi-layered testing strategy to ensure high fidelity parsing across various Shader Models and HLSL syntax versions.

## 📊 Test Coverage

### 1. Lexer Snapshot Tests
Ensures tokens are correctly identified, including graphics-specific edge cases:
- **Shader Semantics**: `POSITION`, `SV_TARGET`, `COLOR0`, etc.
- **Numeric Suffixes**: Handling of `f`, `h`, `u`, `l` suffixes for literals.
- **Preprocessor Directives**: Basic support for `#define`, `#include`, and conditional compilation.

### 2. Syntax Verification
Validates the structural integrity of the AST for:
- **Shader Entry Points**: Correct parsing of function signatures with input/output structures.
- **Resource Bindings**: Verification of `cbuffer`, `tbuffer`, and resource register bindings (`: register(t0)`).
- **Intrinsic Functions**: Proper handling of HLSL built-in functions like `mul()`, `dot()`, `lerp()`, etc.
- **Matrix/Vector Operations**: Ensuring correct precedence for swizzling and component-wise math.

### 3. Error Recovery Tests
Verifies that the parser remains stable when encountering:
- Missing semicolons in shader bodies.
- Incomplete buffer definitions.
- Syntax errors inside shader stages.

## 🚀 Running Tests

```bash
# Run all HLSL parser tests
cargo test -p oak-hlsl

# Run specific lexer tests
cargo test -p oak-hlsl --test lexer_tests
```

## 📈 Quality Standards
Every pull request must pass the full test suite and maintain snapshot stability. For new language features or bug fixes, corresponding tests should be added to the `tests/` directory.
