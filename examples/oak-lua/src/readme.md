# 🛠️ Lua Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-lua`.

## 🚦 Quick Start

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
oak-lua = { path = "..." }
```

### Basic Parsing Example

The following is a standard workflow for parsing Lua scripts, including support for tables, functions, and control flow:

```rust
use oak_lua::{LuaParser, SourceText, LuaLanguage};

fn main() {
    // 1. Prepare source code
    let code = r#"
        local function greet(name)
            local message = "Hello, " .. name
            print(message)
        end

        local user = {
            name = "Oak",
            age = 1
        }

        greet(user.name)
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser
    let config = LuaLanguage::new();
    let parser = LuaParser::new(&config);

    // 3. Execute parsing
    let result = parser.parse(&source);

    // 4. Handle results
    if result.is_success() {
        println!("Parsing successful! AST node count: {}", result.node_count());
    } else {
        eprintln!("Errors found during parsing.");
        for diag in result.diagnostics() {
            println!("[{}:{}] {}", diag.line, diag.column, diag.message);
        }
    }
}
```

## 🔍 Core API Usage

### 1. Syntax Tree Traversal
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract Lua specific constructs like function definitions, table constructors, local variable declarations, and loops.

### 2. Incremental Parsing
Lua scripts can grow large in game development. `oak-lua` supports sub-millisecond incremental updates:
```rust
// Re-parse only the modified section
let new_result = parser.reparse(&new_source, &old_result);
```

### 3. Error Recovery
The parser is designed for industrial-grade fault tolerance, recovering gracefully from missing `end` keywords or malformed table syntax to provide continuous feedback in IDEs.

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes Lua source text, supporting Lua-specific long strings (`[[ ... ]]`), long comments, and numeric literals.
- **Parser**: A high-performance recursive descent parser with Pratt parsing for expressions, handling Lua's operator precedence and table constructors.
- **AST**: A strongly-typed, lossless syntax tree that preserves all trivia (comments/whitespace) for refactoring and formatting tools.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/readme.md](tests/readme.md) for details on our snapshot-based testing.
