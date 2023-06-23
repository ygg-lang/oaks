# 🧪 Ruby Parser Test Suite

`oak-ruby` employs a rigorous testing methodology to ensure accurate parsing of Ruby's diverse and often complex syntax patterns.

## 📊 Test Coverage

### 1. Lexer Snapshot Tests
Ensures correct tokenization of:
- **String Interpolation**: Deeply nested `#{...}` structures.
- **Heredocs**: Handles all variants (standard, indented `<<-`, and squiggly `<<~`).
- **Percent Literals**: Full range of `%q`, `%Q`, `%w`, `%W`, `%i`, `%I`, `%r`, `%x`, and `%s`.
- **Regular Expressions**: Precise tokenization of complex regex patterns.

### 2. Syntax Verification
Validates the structural integrity of the AST for:
- **Metaprogramming**: Handles code that heavily uses `define_method`, `attr_accessor`, and other dynamic constructs.
- **Blocks & Procs**: Correct parsing of block arguments and local variable scoping within blocks.
- **Exception Handling**: Robust parsing of `begin-rescue-ensure-end` blocks.
- **Pattern Matching**: Support for modern Ruby 3.x pattern matching syntax.
- **Keyword Arguments**: Precise handling of complex parameter lists and double-splat operators.

### 3. Error Recovery Tests
Ensures the parser remains functional when facing:
- Unclosed blocks or strings.
- Syntax errors in Rails-style DSLs.
- Incomplete class or method definitions.

## 🚀 Running Tests

```bash
# Run all Ruby parser tests
cargo test -p oak-ruby

# Run specific integration tests
cargo test -p oak-ruby --test parser_tests
```

## 📈 Quality Assurance
All contributions must pass the existing test suite and include new tests for any added features or fixed bugs. We strive for 100% compatibility with standard Ruby (MRI) parsing behavior.
