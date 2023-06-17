# Oak Ruby Parser

[![Crates.io](https://img.shields.io/crates/v/oak-ruby.svg)](https://crates.io/crates/oak-ruby)
[![Documentation](https://docs.rs/oak-ruby/badge.svg)](https://docs.rs/oak-ruby)

High-performance incremental Ruby parser for the oak ecosystem with flexible configuration, optimized for static analysis and code generation.

## 🎯 Overview

Oak Ruby is a robust parser for Ruby, designed to handle complete Ruby syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for static analysis and code generation.

## ✨ Features

- **Complete Ruby Syntax**: Supports all Ruby features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_ruby::{Parser, RubyLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
puts "Hello, World!"

name = "Ruby"
puts "Welcome to #{name}!"
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed Ruby successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Method Parsing
```rust
use oak_ruby::{Parser, RubyLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
def add(a, b)
    return a + b
end

result = add(5, 3)
puts "Result: #{result}"
"#);

let result = parser.parse(&source);
println!("Method parsed successfully.");
```

### Class Parsing
```rust
use oak_ruby::{Parser, RubyLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
class Calculator
    def initialize
        @result = 0
    end
    
    def add(value)
        @result += value
        self
    end
    
    def get_result
        @result
    end
end
"#);

let result = parser.parse(&source);
println!("Class parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_ruby::{Parser, RubyLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("x = 42");
let result = parser.parse(&source);
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_ruby::{Parser, RubyLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
# Invalid Ruby code example
def broken_function(
    puts "Hello"
# Missing closing brace
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

- **RubyProgram**: Root container for Ruby programs
- **Function**: Ruby functions and methods
- **Class**: Ruby class definitions
- **Statement**: Various statement types including control flow
- **Expression**: Various expression types including operators
- **Variable**: Ruby variable constructs

## 📊 Performance

- **Streaming**: Parse large Ruby files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak Ruby integrates seamlessly with:

- **Static Analysis**: Code quality and security analysis
- **Code Generation**: Generating code from Ruby AST
- **IDE Support**: Language server protocol compatibility
- **Refactoring**: Automated code refactoring
- **Documentation**: Generating documentation from Ruby code

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete Ruby program parsing
- Method and class analysis
- Code transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-ruby) or open [issues](https://github.com/ygg-lang/oaks/issues).