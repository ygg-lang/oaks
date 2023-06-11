# Oaks - Parser Combinator Library Collection

[![Rust Version](https://img.shields.io/badge/rust-nightly-blue.svg)](https://www.rust-lang.org)

Oaks is a comprehensive collection of parser combinator libraries for Rust, providing robust parsing solutions for
multiple programming languages and data formats. Built on a solid foundation of parser combinators, Oaks offers both
high-level convenience and low-level control.

## 🚀 Features

- **Multi-Language Support**: 50+ programming languages and data formats
- **High Performance**: Zero-copy parsing with minimal allocations
- **Extensible**: Modular design allowing custom parsers and extensions
- **No-std Compatible**: Core library works without the standard library
- **Comprehensive Error Handling**: Detailed error messages with source locations
- **Incremental Parsing**: Support for partial and streaming parsing
- **Syntax Highlighting**: Built-in multi-language syntax highlighting
- **Pretty Printing**: Code formatting and pretty printing utilities

## 📦 Core Crates

| Crate              | Description                           | Status         |
|--------------------|---------------------------------------|----------------|
| `oak-core`         | Core parser combinator primitives     | ✅ Active       |
| `oak-highlight`    | Multi-language syntax highlighter     | ✅ Active       |
| `oak-pretty-print` | Code formatting and pretty printing   | ✅ Active       |
| `oak-visualize`    | AST visualization and graph rendering | 🔄 Development |
| `oaks`             | Main unified library                  | ✅ Active       |

## 🔧 Language Parsers

### System Programming

- `oak-c` - C language parser with preprocessor
- `oak-cpp` - C++ language parser
- `oak-rust` - Rust language parser
- `oak-zig` - Zig language parser
- `oak-go` - Go language parser
- `oak-d` - D language parser

### Web & Scripting

- `oak-javascript` - JavaScript/ECMAScript parser
- `oak-typescript` - TypeScript parser
- `oak-html` - HTML parser
- `oak-css` - CSS parser
- `oak-json` - JSON parser with streaming
- `oak-markdown` - Markdown parser with CommonMark

### Functional & JVM

- `oak-python` - Python language parser
- `oak-java` - Java language parser
- `oak-kotlin` - Kotlin language parser
- `oak-scala` - Scala language parser
- `oak-clojure` - Clojure parser
- `oak-fsharp` - F# parser

### Data & Config

- `oak-yaml` - YAML parser
- `oak-toml` - TOML parser
- `oak-ini` - INI parser
- `oak-csv` - CSV parser
- `oak-xml` - XML parser

### And many more... (50+ languages supported)

## 🛠️ Quick Start

Basic usage example:

```rust
use oak_core::{Language, Lexer};
use oak_rust::{RustLanguage, RustLexer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Tokenize Rust code
    let input = "fn main() { println!(\"Hello, World!\"); }";
    let lexer = RustLexer::new(input);
    let tokens: Vec<_> = lexer.collect();

    println!("Tokens: {:?}", tokens);
    Ok(())
}
```

## 🏗️ Architecture

Oaks follows a modular workspace architecture:

- **Core Libraries**: Fundamental parser combinator primitives in `projects/`
    - `oak-core`: Core parsing infrastructure and traits
    - `oak-highlight`: Multi-language syntax highlighting
    - `oak-pretty-print`: Code formatting utilities
    - `oak-visualize`: AST visualization (in development)
    - `oaks`: Unified API facade

- **Language Parsers**: Individual parser implementations in `examples/`
    - 50+ language-specific parsers
    - Each parser built on top of `oak-core`
    - Consistent API design across all languages

- **Development Tools**: Build and development utilities
    - Workspace-based development with Cargo
    - Comprehensive test suites for each parser
    - Documentation and examples

## 📁 Project Structure

```
oaks/
├── projects/              # Core libraries
│   ├── oak-core/         # Core parsing infrastructure
│   ├── oak-highlight/    # Syntax highlighting
│   ├── oak-pretty-print/ # Code formatting
│   ├── oak-visualize/    # AST visualization
│   └── oaks/             # Main unified library
├── examples/             # Language parsers (50+ languages)
│   ├── oak-rust/         # Rust parser
│   ├── oak-c/           # C parser
│   ├── oak-javascript/  # JavaScript parser
│   └── ...              # Many more languages
├── documents/            # Additional documentation
├── target/              # Build artifacts
└── Cargo.toml           # Workspace configuration
```

## 🔨 Build & Development

### Requirements

- Rust nightly toolchain (required for `new_range_api` feature)
- Cargo workspace support

### Building

```bash
# Build all crates
cargo build --release

# Build specific language parser
cargo build -p oak-rust

# Run tests
cargo test --release
```

### Development Scripts

```bash
# Format code
npm run fmt

# Generate documentation
npm run doc

# Run all tests
npm run test
```

## 📋 Examples

### Tokenizing Rust Code

```rust
use oak_rust::{RustLexer, RustLanguage};
use oak_core::{Lexer, Language};

let input = "fn main() { println!(\"Hello, World!\"); }";
let lexer = RustLexer::new(input);
let tokens: Vec<_ > = lexer.collect();
println!("Found {} tokens", tokens.len());
```

### Syntax Highlighting

```rust
use oak_highlight::{Highlighter, Theme};
use oak_rust::RustLanguage;

let highlighter = Highlighter::new(RustLanguage::new());
let highlighted = highlighter.highlight_to_html(code, Theme::Github);
```

### Pretty Printing

```rust
use oak_rust::RustLanguage;
use oak_pretty_print::Formatter;

let formatter = Formatter::new(RustLanguage::new());
let formatted = formatter.format(code) ?;
```

## 🚦 Development Status

Oaks is actively developed and maintained. Current status:

### ✅ Completed

- Core parser combinator framework (`oak-core`)
- Basic lexer infrastructure for all languages
- Syntax highlighting system (`oak-highlight`)
- Pretty printing framework (`oak-pretty-print`)
- Project structure for 50+ languages

### 🔄 In Development

- Full AST parsing for major languages (Rust, C, JavaScript, Python)
- Advanced error recovery mechanisms
- Incremental parsing capabilities
- Performance optimizations
- Documentation and examples

### 📋 Planned

- Complete AST implementations for all languages
- Language server protocol support
- Advanced code analysis features
- IDE integrations
- Streaming parser support

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to
discuss what you would like to change.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 🙏 Acknowledgments

- Built with love for the Rust community
- Inspired by parser combinator libraries like [nom](https://github.com/Geal/nom)
  and [combine](https://github.com/Marwes/combine)
- Thanks to all contributors who have helped shape this project

## 📚 Resources

- [Repository](https://github.com/oovm/pex)
- [Issue Tracker](https://github.com/oovm/pex/issues)
- Development documentation in each crate's `readme.md`
- Examples in the `examples/` directory

---

**Happy Parsing!** 🎉