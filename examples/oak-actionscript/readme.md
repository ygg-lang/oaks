# Oak ActionScript Parser

[![Crates.io](https://img.shields.io/crates/v/oak-actionscript.svg)](https://crates.io/crates/oak-actionscript)
[![Documentation](https://docs.rs/oak-actionscript/badge.svg)](https://docs.rs/oak-actionscript)

High-performance incremental ActionScript parser for the oak ecosystem with flexible configuration, optimized for Adobe Flash and Apache Flex development.

## 🎯 Overview

Oak ActionScript is a robust parser for ActionScript 3.0, designed to handle both legacy Flash and modern Flex applications. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for ActionScript analysis and tooling.

## ✨ Features

- **Complete AS3 Syntax**: Supports all ActionScript 3.0 features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_actionscript::{Parser, ActionScriptLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
package com.example {
    public class Main {
        public function Main() {
            trace("Hello, ActionScript!");
        }
    }
}
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed ActionScript package successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Class Parsing
```rust
use oak_actionscript::{Parser, ActionScriptLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
public class MovieClip extends flash.display.MovieClip {
    private var frameCount:int;
    
    public function MovieClip() {
        frameCount = 0;
    }
    
    public function play():void {
        super.play();
    }
}
"#);

let result = parser.parse(&source);
println!("Parsed ActionScript class successfully.");
```

### Function Parsing
```rust
use oak_actionscript::{Parser, ActionScriptLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
public function calculateArea(width:Number, height:Number):Number {
    return width * height;
}
"#);

let result = parser.parse(&source);
println!("Parsed ActionScript function successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_actionscript::{Parser, ActionScriptLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("public function test():void {}");
let result = parser.parse(&source);
// Token information is available in the parse result
```

### Error Handling
```rust
use oak_actionscript::{Parser, ActionScriptLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
package {
    public class Test {
        public function test( {
            trace("Missing closing parenthesis");
        }
    }
}
"#);

let result = parser.parse(&source);
if let Err(e) = result.result {
    println!("Parse error: {:?}", e);
}
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **Package**: ActionScript package declarations
- **ClassDefinition**: Class definitions with inheritance
- **FunctionDefinition**: Function and method definitions
- **VariableDeclaration**: Variable declarations with type annotations
- **Expression**: Expressions including arithmetic, logical, and function calls
- **Statement**: Control flow statements and assignments
- **TypeAnnotation**: Type specifications and annotations

## 📊 Performance

- **Streaming**: Parse large ActionScript files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak of actionscript integrates seamlessly with:

- **Flash Development**: Build Flash applications and games
- **Code Analysis**: Static analysis and refactoring tools
- **IDE Support**: Language server protocol compatibility for ActionScript
- **Migration Tools**: Convert ActionScript to other languages
- **Educational Tools**: Build programming language learning environments

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete ActionScript class parsing
- Flash API integration
- Event handling and animation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-actionscript) or open [issues](https://github.com/ygg-lang/oaks/issues).