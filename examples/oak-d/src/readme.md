# Oak-D: D Language Lexer for Oak Ecosystem

Oak-D is a high-performance incremental lexer for the D programming language, designed as part of the oak parsing framework. It provides comprehensive tokenization of D language source code with support for both D1 and D2 language features.

## Features

- **Complete D Language Support**: Handles all D language constructs including keywords, operators, literals, and comments
- **Incremental Parsing**: Designed for efficient incremental reparsing in IDE environments
- **Flexible Configuration**: Supports different language feature sets (D1, D2, minimal configurations)
- **Performance Optimized**: Built for high-throughput parsing of large codebases
- **Comprehensive Token Types**: Detailed token categorization for accurate syntax analysis

## Usage

### Basic Usage

```rust
use oak_d::{DLexer, DLanguage};
use oak_core::{Lexer, SourceText, parser::session::ParseSession};

// Create a language configuration
let language = DLanguage::standard(); // or DLanguage::minimal() for basic features

// Create a lexer instance
let lexer = DLexer::new(&language);
let mut session = ParseSession::default();

// Parse source code
let source = SourceText::new("module example; void main() { writeln(\"Hello, D!\"); }");
let output = lexer.lex(&source, &[], &mut session);

// Process tokens
if let Ok(tokens) = output.result {
    for token in tokens.iter() {
        println!("{:?}: {:?}", token.kind, token.span);
    }
}
```

### Language Configuration

Oak-D provides several language configurations:

```rust
use oak_d::DLanguage;

// Standard D with all features enabled
let standard = DLanguage::standard();

// Minimal D with only core features
let minimal = DLanguage::minimal();

// Custom configuration
let custom = DLanguage {
    d2_features: true,    // Enable D2-specific features
    inline_asm: true,     // Allow inline assembly
    contracts: true,      // Enable contract programming
};
```

### Integration with Oak Ecosystem

Oak-D is designed to work seamlessly with the oak parsing framework:

```rust
use oak_core::{Parser, source::SourceText, parser::session::ParseSession};
use oak_d::{DLanguage, DLexer, parser::DParser};

// Create a language configuration
let language = DLanguage::standard();

// Create a parsing session
let mut session = ParseSession::<DLanguage>::default();

// Create a parser
let parser = DParser::new(&language);

// Parse source code
let source = SourceText::new("module example;");
let result = parser.parse(&source, &[], &mut session);
```

## Token Types

Oak-D provides comprehensive tokenization of D language constructs:

### Keywords
- Module declarations: `module`, `import`, `package`
- Access modifiers: `public`, `private`, `protected`, `export`
- Storage classes: `static`, `final`, `const`, `immutable`, `shared`
- Type modifiers: `auto`, `alias`, `typedef`
- Control flow: `if`, `else`, `while`, `for`, `foreach`, `do`, `switch`
- Exception handling: `try`, `catch`, `finally`, `throw`
- Object-oriented: `class`, `struct`, `interface`, `union`, `enum`
- Functions: `function`, `delegate`, `return`
- Memory management: `new`, `delete`
- Contract programming: `invariant`, `in`, `out`, `body`
- Metaprogramming: `template`, `mixin`, `static`, `typeof`, `typeid`
- Attributes: `pure`, `nothrow`, `safe`, `trusted`, `system`, `nogc`, `property`
- Built-in types: `void`, `bool`, `byte`, `ubyte`, `short`, `ushort`, `int`, `uint`, `long`, `ulong`, `cent`, `ucent`, `float`, `double`, `real`, `ifloat`, `idouble`, `ireal`, `cfloat`, `cdouble`, `creal`, `char`, `wchar`, `dchar`, `string`, `wstring`, `dstring`

### Operators
- Arithmetic: `+`, `-`, `*`, `/`, `%`
- Assignment: `=`, `+=`, `-=`, `*=`, `/=`, `%=`
- Bitwise: `&`, `|`, `^`, `~`, `<<`, `>>`, `>>>`
- Comparison: `==`, `!=`, `<`, `<=`, `>`, `>=`, `is`, `!is`
- Logical: `&&`, `||`, `!`
- Other: `++`, `--`, `?`, `$`, `@`

### Literals
- Integer literals: `123`, `0x1A`, `0b1010`
- Floating-point literals: `123.45`, `1.23e-4`, `12.34f`
- String literals: `"Hello"`, `'c'`, `r"raw"`
- Character literals: `'a'`, `'\n'`, `'\u00A9'`

### Comments
- Line comments: `// This is a comment`
- Block comments: `/* This is a block comment */`
- Nested comments: `/+ This is a +/ nested comment +/`
- Documentation comments: `/// DDoc comment`, `/** DDoc comment */`

## Testing

Oak-D includes comprehensive tests for the lexer:

```bash
# Run all tests
cargo test

# Run lexer tests specifically
cargo test test_d_lexer
```

## Architecture

Oak-D is built on the oak-core framework and follows its design principles:

- **Modular Design**: Separate modules for lexer, language definition, and token types
- **Incremental Processing**: Designed for efficient incremental reparsing
- **Extensible**: Easy to extend with additional language features
- **Performance Focused**: Optimized for high-throughput parsing

## Integration Examples

### IDE Integration

```rust,ignore
use oak_d::{DLexer, DLanguage};
use oak_core::incremental::{IncrementalParser, ChangeSet};

// Create an incremental parser
let mut parser = IncrementalParser::new(DLanguage::standard());

// Initial parse
let initial_result = parser.parse_initial(source);

// Apply incremental changes
let changes = ChangeSet::from_diff(old_source, new_source);
let updated_result = parser.parse_incremental(changes);
```

### Highlighting Support

Oak-D integrates with `oak-highlight` for syntax highlighting:

```rust,ignore
use oak_d::{DLexer, DLanguage};
use oak_core::{lexer::Lexer, source::SourceText, parser::session::ParseSession};
use oak_highlight::{OakHighlighter, Theme};

let language = DLanguage::standard();
let lexer = DLexer::new(&language);
let mut session = ParseSession::<DLanguage>::default();
let source = SourceText::new("module example;");
let output = lexer.lex(&source, &[], &mut session);
```

## Performance Considerations

- Oak-D is optimized for performance with large codebases
- Uses efficient string handling and memory management
- Incremental parsing minimizes reprocessing of unchanged code
- Token caching improves performance for repeated parsing operations

## Future Enhancements

- Full parser implementation for complete syntax tree generation
- Error recovery mechanisms for robust parsing of invalid code
- Semantic analysis integration
- Additional D language features as they are introduced