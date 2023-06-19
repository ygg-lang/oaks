# Oak Elixir Parser

[![Crates.io](https://img.shields.io/crates/v/oak-elixir.svg)](https://crates.io/crates/oak-elixir)
[![Documentation](https://docs.rs/oak-elixir/badge.svg)](https://docs.rs/oak-elixir)

High-performance incremental Elixir parser for the oak ecosystem with flexible configuration, optimized for static analysis and code generation.

## 🎯 Overview

Oak Elixir is a robust parser for Elixir, designed to handle complete Elixir syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for static analysis and code generation.

## ✨ Features

- **Complete Elixir Syntax**: Supports all Elixir features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_elixir::{Parser, ElixirLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
defmodule HelloWorld do
  def greet(name) do
    IO.puts("Hello, #{name}!")
  end
end

HelloWorld.greet("Elixir")
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed Elixir successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Function Parsing
```rust
use oak_elixir::{Parser, ElixirLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
defmodule Math do
  def add(a, b), do: a + b
  
  def factorial(0), do: 1
  def factorial(n) when n > 0, do: n * factorial(n - 1)
  
  def main do
    result = factorial(5)
    IO.puts("Factorial of 5 is: #{result}")
  end
end

Math.main()
"#);

let result = parser.parse(&source);
println!("Function parsed successfully.");
```

### Struct Parsing
```rust
use oak_elixir::{Parser, ElixirLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
defmodule Person do
  defstruct name: nil, age: nil
  
  def greet(%Person{name: name, age: age}) do
    IO.puts("Hello, I'm #{name} and I'm #{age} years old")
  end
  
  def have_birthday(person = %Person{age: age}) do
    %{person | age: age + 1}
  end
end

person = %Person{name: "Alice", age: 25}
Person.greet(person)
updated_person = Person.have_birthday(person)
Person.greet(updated_person)
"#);

let result = parser.parse(&source);
println!("Struct parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_elixir::{Parser, ElixirLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("x = 42");
let result = parser.parse(&source);
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_elixir::{Parser, ElixirLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
# Invalid Elixir code example
defmodule BrokenModule do
  def broken_function do
    IO.puts("Hello"
    # Missing closing parenthesis
  end
end
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

- **ElixirProgram**: Root container for Elixir programs
- **Module**: Elixir module definitions
- **Function**: Elixir functions and methods
- **Struct**: Elixir struct definitions
- **Statement**: Various statement types including control flow
- **Expression**: Various expression types including operators
- **Pattern**: Pattern matching constructs

## 📊 Performance

- **Streaming**: Parse large Elixir files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak Elixir integrates seamlessly with:

- **Static Analysis**: Code quality and security analysis
- **Code Generation**: Generating code from Elixir AST
- **IDE Support**: Language server protocol compatibility
- **Refactoring**: Automated code refactoring
- **Documentation**: Generating documentation from Elixir code

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete Elixir program parsing
- Module and function analysis
- Code transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-elixir) or open [issues](https://github.com/ygg-lang/oaks/issues).