# Oaks - Parser Framework for Rust

[![Rust Version](https://img.shields.io/badge/rust-nightly-blue.svg)](https://www.rust-lang.org)

Oaks is a modular parser framework for Rust that provides a unified approach to building language parsers. Built on the oak-core foundation, Oaks offers a comprehensive set of tools for lexical analysis, parsing, and syntax tree manipulation.

## 🚀 Features

- **Modular Architecture**: Decouples the core parsing engine from language-specific logic. Implement new languages by defining `TokenType` and `ElementType` without touching the core infrastructure.
- **Lossless Green/Red Tree**: Implements a Rowan-style architecture. **Green Trees** are immutable and interned for memory efficiency, while **Red Trees** provide a parent-aware, position-aware view for effortless traversal.
- **Structural Sharing**: Modifications to the tree use `Arc`-based sharing. Only the modified nodes and their direct ancestors are recreated, making transformations and refactorings extremely memory-efficient.
- **Error Recovery**: The parser can recover from syntax errors to produce a partial but valid tree, ensuring that features like highlighting and autocompletion remain functional during active editing.
- **Incremental Parsing**: By utilizing an `IncrementalCache`, the framework only re-parses the changed portions of the source code, enabling lightning-fast updates for large files in IDE environments.
- **Source Mapping**: Absolute position information is dynamically computed from Red Nodes, providing a stable and accurate foundation for Source Maps and diagnostic reporting.

## 🛠️ Core Capabilities

### Formatter
Implemented by manipulating `Trivia` (whitespace, line breaks, comments) within the `GreenNode` structure. It ensures 100% lossless code formatting by reconstructing the source text while preserving or adjusting non-functional tokens.

### Linter
Utilizes the `Visitor` pattern to traverse the `RedTree`. By leveraging absolute `span` information computed from red nodes, it performs efficient static analysis and provides precise diagnostic locations for coding standard violations.

### Highlighter
Supports dual-mode highlighting: fast Lexer-based highlighting using the raw token stream, and precise Parser-based highlighting that utilizes the full syntax tree to distinguish between semantic categories like function calls, types, and variables.

### Transformer
Powered by the `Transformer` trait, it enables high-performance code refactoring. It leverages the **Structural Sharing** property of the Red-Green tree architecture; only modified nodes and their parent paths are recreated as new `GreenNodes`, while unchanged subtrees are efficiently reused via `Arc`.

## 📦 Core Components

| Component          | Description                           | Status         |
|--------------------|---------------------------------------|----------------|
| `oak-core`         | Core parsing infrastructure and traits| ✅ Active       |
| `oak-highlight`    | Multi-language syntax highlighter     | ✅ Active       |
| `oak-pretty-print` | Code formatting and pretty printing   | ✅ Active       |
| `oak-visualize`    | AST visualization and graph rendering | 🔄 Development |
| `oaks`             | Main unified library                  | ✅ Active       |

## 🔧 Language Parsers

### System Programming

- `oak-c` - C language parser with preprocessor
- `oak-rust` - Rust language parser
- `oak-zig` - Zig language parser
- `oak-go` - Go language parser

### Web & Scripting

- `oak-javascript` - JavaScript/ECMAScript parser
- `oak-html` - HTML parser
- `oak-css` - CSS parser
- `oak-json` - JSON parser with JSON5 support
- `oak-markdown` - Markdown parser with CommonMark

### Functional & JVM

- `oak-python` - Python language parser
- `oak-java` - Java language parser
- `oak-kotlin` - Kotlin language parser
- `oak-scala` - Scala language parser

### Data & Config

- `oak-yaml` - YAML parser
- `oak-toml` - TOML parser
- `oak-ini` - INI parser
- `oak-csv` - CSV parser
- `oak-xml` - XML parser

### And many more...

## 🛠️ Quick Start

Basic usage example with oak-c:

```rust
use oak_core::{SourceText, Parser};
use oak_c::{CLanguage, CParser};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create source text from input
    let source = SourceText::new("int main() { return 0; }");
    
    // Parse the source code
    let parser = CParser::new(CLanguage::default());
    let result = parser.parse(&source);
    
    // Handle the result
    match result {
        Ok(tree) => println!("Parsed successfully: {:?}", tree),
        Err(errors) => println!("Parse errors: {:?}", errors),
    }
    
    Ok(())
}
```

## 🏗️ Architecture

Oaks is designed from the ground up to be a **High-Performance LSP Foundation**. Its architecture solves the most challenging problems in building modern IDE support:

### The LSP Powerhouse
- **Native LSP Type Support**: `SourceText` provides built-in, zero-cost conversion between UTF-8 byte offsets and LSP-standard `Line/Character` positions.
- **Resilient Analysis**: The framework's **Error Recovery** ensures that your Language Server remains responsive even when the user's code is in an invalid state.
- **HMR-Ready**: Sub-millisecond **Incremental Parsing** means your LSP can provide instant feedback on every keystroke, even in multi-megabyte files.
- **Refactoring Engine**: The `Transformer` trait combined with **Structural Sharing** allows for complex code actions (like "Rename" or "Extract Method") to be implemented with high performance and 100% comment preservation.

### Semantic Integration Ready
While Oaks focuses on high-performance syntax analysis, it is designed to be the perfect foundation for semantic analysis:
- **Semantic Hints**: The `TokenType` and `ElementType` traits provide built-in hooks like `is_definition`, `is_reference`, and `is_scope_boundary`, allowing external semantic engines to instantly understand the tree's logical structure.
- **Stable Identifiers**: Red nodes provide stable pointers that semantic analyzers can use for symbol indexing and cross-referencing.
- **Parent-Aware Navigation**: The `RedTree` allows semantic checkers to easily bubble up from a usage to its scope or declaration.
- **Typed IR**: The high-level **Typed AST** layer serves as a clean, serializable Intermediate Representation (IR) that external type-checkers and symbol solvers can consume without knowing the details of the red-green tree.
- **Framework Agnostic**: Oaks is unopinionated about how you handle semantics, making it trivial to integrate with databases (like Salsa) or graph-based analysis engines.

### Core Framework Concepts
- **Language Trait**: A unified interface to plug in any grammar.
- **Green Tree**: The "What" — immutable, shared, and extremely compact.
- **Red Tree**: The "Where" — a lightweight, parent-aware view for easy tree walking.
- **Visitor & Transformer**: Standardized patterns for both read-only analysis (Linter) and read-write mutations (Refactoring).

### Language Implementations

Each language parser (e.g., `oak-c`, `oak-json`) follows a consistent pattern:

1. **SyntaxKind Enum**: Defines all possible syntax elements
2. **Language Struct**: Implements the Language trait
3. **Lexer Struct**: Implements tokenization for the language
4. **AST Definitions**: Optional typed AST structures

### Project Structure

```
oaks/
├── projects/              # Core libraries
│   ├── oak-core/         # Core parsing infrastructure
│   │   ├── src/
│   │   │   ├── lib.rs    # Main API exports
│   │   │   ├── language/ # Language trait
│   │   │   ├── lexer/    # Lexer trait and utilities
│   │   │   ├── parser/   # Parser trait and Pratt parser
│   │   │   ├── source/   # SourceText implementation
│   │   │   ├── tree/     # Green/Red tree implementation
│   │   │   └── visitor/  # Tree visitor pattern
│   │   └── Cargo.toml
│   ├── oak-highlight/    # Syntax highlighting
│   ├── oak-pretty-print/ # Code formatting
│   ├── oak-visualize/    # AST visualization
│   └── oaks/             # Main unified library
├── examples/             # Language parsers
│   ├── oak-c/           # C parser example
│   │   ├── src/
│   │   │   ├── lib.rs   # Public API
│   │   │   ├── language/ # Language implementation
│   │   │   ├── lexer/   # Lexer implementation
│   │   │   └── kind/    # SyntaxKind definitions
│   │   └── Cargo.toml
│   ├── oak-json/        # JSON parser example
│   │   ├── src/
│   │   │   ├── lib.rs   # Public API
│   │   │   ├── language/ # Language implementation
│   │   │   ├── lexer/   # Lexer implementation
│   │   │   └── kind/    # SyntaxKind definitions
│   │   └── Cargo.toml
│   └── ...              # Other language parsers
└── Cargo.toml           # Workspace configuration
```

## 🔨 Build & Development

### Requirements

- Rust nightly toolchain (required for `new_range_api` feature)
- Cargo workspace support

## 📋 Examples

### Parsing C Code

```rust
use oak_core::{SourceText, Parser};
use oak_c::{CLanguage, CParser};

fn parse_c_function() -> Result<(), Box<dyn std::error::Error>> {
    let source = SourceText::new("
        int factorial(int n) {
            if (n <= 1) return 1;
            return n * factorial(n - 1);
        }
    ");
    
    let parser = CParser::new(CLanguage::default());
    let result = parser.parse(&source)?;
    
    println!("Parsed C function successfully");
    Ok(())
}
```

### Parsing JSON with Configuration

```rust
use oak_core::{SourceText, Parser};
use oak_json::{JsonLanguage, JsonParser};

fn parse_json5() -> Result<(), Box<dyn std::error::Error>> {
    // JSON5 allows comments, trailing commas, and more
    let source = SourceText::new(r#"
    {
        // This is a comment
        "name": "Oaks Parser",
        "version": "1.0.0",  // trailing comma
    }
    "#);
    
    // Use JSON5 configuration
    let language = JsonLanguage::json5();
    let parser = JsonParser::new(language);
    let result = parser.parse(&source)?;
    
    println!("Parsed JSON5 successfully");
    Ok(())
}
```

### Incremental Parsing

```rust
use oak_core::{SourceText, Parser, TextEdit};
use oak_json::{JsonLanguage, JsonParser};

fn incremental_parsing() -> Result<(), Box<dyn std::error::Error>> {
    let mut source = SourceText::new(r#"{"name": "Oaks"}"#);
    
    let language = JsonLanguage::standard();
    let parser = JsonParser::new(language);
    
    // Initial parse
    let result = parser.parse(&source)?;
    
    // Apply edits
    let edits = vec![TextEdit {
        span: 7..7,  // After "name":
        text: ", \"version\": \"1.0\"".to_string(),
    }];
    
    let min_offset = source.apply_edits(&edits);
    
    // Incremental reparse (only affected part)
    let incremental_result = parser.parse_incremental(&source, min_offset)?;
    
    println!("Incremental parsing successful");
    Ok(())
}
```

### Syntax Highlighting

```rust
use oak_highlight::{Highlighter, Theme};
use oak_c::CLanguage;

fn syntax_highlighting() -> Result<(), Box<dyn std::error::Error>> {
    let code = r#"
    #include <stdio.h>
    
    int main() {
        printf("Hello, World!\n");
        return 0;
    }
    "#;
    
    let language = CLanguage::default();
    let highlighter = Highlighter::new(language);
    let highlighted = highlighter.highlight_to_html(code, Theme::Github);
    
    println!("Highlighted HTML: {}", highlighted);
    Ok(())
}
```

## 🚦 Development Status

Oaks is actively developed and maintained. Current status:

### ✅ Completed

- Core parser framework (`oak-core`)
  - Language trait and infrastructure
  - Lexer and Parser traits
  - Green/Red tree system
  - SourceText with line/column tracking
  - Error recovery mechanisms
- Basic language implementations
  - C parser with full syntax support
  - JSON parser with JSON5 support
  - Other language parsers in various stages
- Syntax highlighting system (`oak-highlight`)
- Pretty printing framework (`oak-pretty-print`)
- Incremental parsing support
- Native `async trait` support (removed `async-trait` dependency)

### 🔄 In Development

- Complete AST implementations for major languages
- Advanced error recovery mechanisms
- Performance optimizations
- Comprehensive test coverage
- Documentation and examples

### 📋 Planned

- Language server protocol support
- Advanced code analysis features
- IDE integrations
- Additional language parsers

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Adding a New Language Parser

To add a new language parser to Oaks:

1. Create a new directory in `examples/` following the pattern `oak-{language}`
2. Implement the required components:
   - `SyntaxKind` enum in `src/kind/`
   - `Language` implementation in `src/language/`
   - `Lexer` implementation in `src/lexer/`
   - Optional: AST definitions in `src/ast/`
3. Add your parser to the workspace in the root `Cargo.toml`
4. Add documentation and examples

For reference implementations, see `oak-c` and `oak-json`.

## 📚 Resources

- [Repository](https://github.com/oovm/pex)
- [Issue Tracker](https://github.com/oovm/pex/issues)
- Development documentation in each crate's `readme.md`
- Examples in the `examples/` directory

---

**Happy Parsing!** 🎉