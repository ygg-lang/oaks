# Oak Rust - Rust Language Parser

Oak Rust is a high-performance Rust language parser built on the Oak framework, providing full syntax analysis, AST generation, syntax highlighting, and code formatting.

## 📋 Overview

Oak Rust parser focuses on the following core features:
- **High Performance**: Incremental parsing with minimal memory overhead.
- **Accuracy**: Full support for Rust's complex syntax rules.
- **Extensibility**: Modular design, easy to customize.
- **Error Recovery**: Gracefully handles syntax errors.
- **Syntax Highlighting**: Built-in highlighter supporting keywords, strings, comments, etc.
- **Code Formatting**: Follows Rust official code style guidelines.

## 🏗️ Architecture

### Core Components

- **[`RustParser`](parser/struct.RustParser.html)**: Main parser implementation, using Pratt parser for operator precedence.
- **[`RustLexer`](lexer/struct.RustLexer.html)**: Lexical analysis engine providing precise position tracking.
- **[`RustLanguage`](language/struct.RustLanguage.html)**: Language configuration and syntax rules.
- **[`RustBuilder`](builder/struct.RustBuilder.html)**: AST builder, converting parse trees to strongly-typed ASTs.
- **[`RustRoot`](ast/struct.RustRoot.html)**: AST root node, containing all parsed items.

### Optional Components

- **[`RustFormatter`](formatter/struct.RustFormatter.html)**: Rust code formatter (requires `oak-pretty-print` feature).
- **[`RustHighlighter`](highlighter/struct.RustHighlighter.html)**: Rust syntax highlighter (requires `oak-highlight` feature).

### AST Structure

The parser generates a strongly-typed AST containing the following main structures:

- **[`Item`](ast/enum.Item.html)**: Top-level items (functions, structs, enums, traits, impls, etc.).
- **[`Function`](ast/struct.Function.html)**: Function definitions, containing parameters and body.
- **[`Statement`](ast/enum.Statement.html)**: Various statement types (let bindings, expression statements, etc.).
- **[`Expr`](ast/enum.Expr.html)**: Expression types, covering all Rust expressions.
- **[`Type`](ast/enum.Type.html)**: Type representations (primitive types, references, generics, etc.).
- **[`Pattern`](ast/enum.Pattern.html)**: Pattern matching (identifiers, structs, tuples, etc.).
- **[`Identifier`](ast/struct.Identifier.html)**: Named identifiers with position information.

## 🔧 Usage Examples

### Basic Parsing
```rust,ignore
use oak_rust::{RustLanguage, RustParser};
use oak_core::language::Language;

let language = RustLanguage::new();
let parser = RustParser::new();

let source = r#"
fn main() {
    let x = 42;
    println!("Hello, world! x = {}", x);
}
"#;

let result = language.parse(source);
match result {
    Ok(ast) => println!("Parsing successful: {:?}", ast),
    Err(errors) => println!("Parsing error: {:?}", errors),
}
```

### Processing AST
```rust,ignore
use oak_rust::{RustLanguage, RustParser, ast::*};

let language = RustLanguage::new();
let parser = RustParser::new();
let source = "fn add(a: i32, b: i32) -> i32 { a + b }";

if let Ok(result) = language.parse(source) {
    if let Some(root) = result.root {
        println!("Parsed {} items", root.items.len());
        
        // Iterate through all items
        for item in &root.items {
            match item {
                Item::Function(func) => {
                    println!("Function: {}", func.name.name);
                    println!("Parameter count: {}", func.params.len());
                }
                Item::Struct(s) => {
                    println!("Struct: {}", s.name.name);
                }
                _ => println!("Other item type"),
            }
        }
    }
}
```

### Syntax Highlighting
```rust,ignore
use oak_rust::RustHighlighter;
use oak_highlight::highlighter::Highlighter;

let highlighter = RustHighlighter::new();
```
