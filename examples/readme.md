# 🌳 Oak Language Parsers

A collection of high-performance, incremental parsers built on the Oak framework. Each parser delivers sub-millisecond performance with robust error recovery and full IDE integration support.

## 📚 Available Parsers

### System & Compiled Languages

| Parser | Description |
|--------|-------------|
| [oak-ada](./oak-ada) | Ada for safety-critical systems |
| [oak-c](./oak-c) | C for systems programming |
| [oak-cpp](./oak-cpp) | C++ with modern standard support |
| [oak-d](./oak-d) | D for systems programming |
| [oak-go](./oak-go) | Go for cloud-native development |
| [oak-nim](./oak-nim) | Nim with indentation support |
| [oak-rust](./oak-rust) | Rust for the Rust ecosystem |
| [oak-swift](./oak-swift) | Swift for Apple platforms |
| [oak-vlang](./oak-vlang) | V for simple systems programming |
| [oak-zig](./oak-zig) | Zig with comptime support |

### Web & Scripting Languages

| Parser | Description |
|--------|-------------|
| [oak-bash](./oak-bash) | Bash shell scripting |
| [oak-cmd](./oak-cmd) | Windows batch scripts |
| [oak-css](./oak-css) | CSS with modern features |
| [oak-dart](./oak-dart) | Dart for Flutter/Dart ecosystem |
| [oak-html](./oak-html) | HTML with HTML5 support |
| [oak-lua](./oak-lua) | Lua for game/embedded scripting |
| [oak-perl](./oak-perl) | Perl |
| [oak-php](./oak-php) | PHP with modern features |
| [oak-python](./oak-python) | Python with type hint support |
| [oak-ruby](./oak-ruby) | Ruby |
| [oak-sass](./oak-sass) | Sass |
| [oak-scss](./oak-scss) | SCSS |
| [oak-vue](./oak-vue) | Vue SFC |

### Data & Configuration

| Parser | Description |
|--------|-------------|
| [oak-csv](./oak-csv) | CSV |
| [oak-dsv](./oak-dsv) | Delimiter-separated values |
| [oak-ini](./oak-ini) | INI configuration |
| [oak-json](./oak-json) | JSON with JSON5 support |
| [oak-nix](./oak-nix) | Nix expressions |
| [oak-toml](./oak-toml) | TOML for Rust configs |
| [oak-tsv](./oak-tsv) | TSV |
| [oak-xml](./oak-xml) | XML |
| [oak-yaml](./oak-yaml) | YAML with anchor support |

### JVM & Functional Languages

| Parser | Description |
|--------|-------------|
| [oak-elm](./oak-elm) | Elm |
| [oak-java](./oak-java) | Java with modern features |
| [oak-kotlin](./oak-kotlin) | Kotlin |
| [oak-ocaml](./oak-ocaml) | OCaml |
| [oak-scala](./oak-scala) | Scala |

### WebAssembly & Low-Level

| Parser | Description |
|--------|-------------|
| [oak-wat](./oak-wat) | WebAssembly Text Format |
| [oak-wgsl](./oak-wgsl) | WebGPU Shading Language |
| [oak-hlsl](./oak-hlsl) | HLSL shaders |
| [oak-gsgl](./oak-gsgl) | GSGL shaders |
| [oak-msil](./oak-msil) | MSIL/CIL |

### Proof Assistants & Formal Methods

| Parser | Description |
|--------|-------------|
| [oak-coq](./oak-coq) | Coq proof assistant |
| [oak-lean](./oak-lean) | Lean theorem prover |

### Specialized Languages

| Parser | Description |
|--------|-------------|
| [oak-apl](./oak-apl) | APL array language |
| [oak-j](./oak-j) | J array language |
| [oak-koka](./oak-koka) | Koka effect handlers |
| [oak-r](./oak-r) | R statistical language |
| [oak-sql](./oak-sql) | SQL with multi-dialect support |

### Markup & Documentation

| Parser | Description |
|--------|-------------|
| [oak-dot](./oak-dot) | DOT graph description |
| [oak-tex](./oak-tex) | TeX/LaTeX |
| [oak-typst](./oak-typst) | Typst |

## 🏗️ Common Architecture

All Oak parsers follow a consistent architecture:

- **Green/Red Trees** — Efficient immutable syntax trees with lossless round-trips
- **Incremental Parsing** — Re-parse only changed portions
- **Error Recovery** — Continue parsing after errors for better IDE experience
- **Trivia Preservation** — Retain whitespace and comments for formatting

## 🚀 Adding a New Parser

1. Define `SyntaxKind` enum for tokens and nodes
2. Implement the `Language` trait from `oak-core`
3. Build the lexer and parser
4. Add to workspace `Cargo.toml`

See [oak-c](./oak-c), [oak-json](./oak-json), or [oak-python](./oak-python) for reference implementations.

## 🤝 Contributing

Contributions are welcome! When adding a new parser, follow the existing architecture pattern and include comprehensive tests.
