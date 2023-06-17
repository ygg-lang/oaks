# Oak Lua Parser

[![Crates.io](https://img.shields.io/crates/v/oak-lua.svg)](https://crates.io/crates/oak-lua)
[![Documentation](https://docs.rs/oak-lua/badge.svg)](https://docs.rs/oak-lua)

High-performance incremental Lua parser for the oak ecosystem with flexible configuration, optimized for static analysis and code generation.

## 🎯 Overview

Oak Lua is a robust parser for Lua, designed to handle complete Lua syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for static analysis and code generation.

## ✨ Features

- **Complete Lua Syntax**: Supports all Lua features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_lua::{Parser, LuaLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
function greet(name)
    print("Hello, " .. name)
end

local message = "Welcome to Lua!"
greet(message)
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed Lua successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Function Parsing
```rust
use oak_lua::{Parser, LuaLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
function factorial(n)
    if n <= 1 then
        return 1
    else
        return n * factorial(n - 1)
    end
end
"#);

let result = parser.parse(&source);
println!("Function parsed successfully.");
```

### Table Parsing
```rust
use oak_lua::{Parser, LuaLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
local config = {
    name = "server",
    port = 8080,
    enabled = true,
    users = {"admin", "guest"}
}
"#);

let result = parser.parse(&source);
println!("Table parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_lua::{Parser, LuaLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("local x = 42");
let result = parser.parse(&source);
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_lua::{Parser, LuaLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
-- Invalid Lua code example
function broken_function(
    print("Hello"
-- Missing closing parenthesis and end
"#);

let result = parser.parse(&source);
if let Some(errors) = result.result.err() {
    println!("Parse errors found: {:?}", errors);
} else {
    println!("Parsed successfully.");
}
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **LuaProgram**: Root container for Lua programs
- **Function**: Lua functions and methods
- **Statement**: Various statement types including control flow
- **Expression**: Various expression types including operators
- **Table**: Lua table constructs

## 📊 Performance

- **Streaming**: Parse large Lua files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak Lua integrates seamlessly with:

- **Static Analysis**: Code quality and security analysis
- **Code Generation**: Generating code from Lua AST
- **IDE Support**: Language server protocol compatibility
- **Refactoring**: Automated code refactoring
- **Documentation**: Generating documentation from Lua code

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete Lua program parsing
- Function and table analysis
- Code transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-lua) or open [issues](https://github.com/ygg-lang/oaks/issues).