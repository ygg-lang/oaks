# Oak Dart Parser

[![Crates.io](https://img.shields.io/crates/v/oak-dart.svg)](https://crates.io/crates/oak-dart)
[![Documentation](https://docs.rs/oak-dart/badge.svg)](https://docs.rs/oak-dart)

High-performance incremental Dart parser for the oak ecosystem with flexible configuration, optimized for code analysis and compilation.

## 🎯 Overview

Oak Dart is a robust parser for Dart, designed to handle complete Dart syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for static analysis and code generation.

## ✨ Features

- **Complete Dart Syntax**: Supports all Dart features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_dart::{Parser, DartLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
import 'dart:io';

void main() {
  print('Hello, Dart!');
  
  var numbers = [1, 2, 3, 4, 5];
  numbers.forEach((number) {
    print(number * 2);
  });
}
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed Dart program successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Class Parsing
```rust
use oak_dart::{Parser, DartLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
class Calculator {
  double _result = 0.0;
  
  double add(double a, double b) {
    _result = a + b;
    return _result;
  }
  
  double subtract(double a, double b) {
    _result = a - b;
    return _result;
  }
  
  double get result => _result;
}
    "#);

let result = parser.parse(&source);
println!("Parsed Dart class successfully.");
```

### Function Parsing
```rust
use oak_dart::{Parser, DartLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
String greet(String name, {String greeting = 'Hello'}) {
  return '$greeting, $name!';
}

int fibonacci(int n) {
  if (n <= 1) return n;
  return fibonacci(n - 1) + fibonacci(n - 2);
}
    "#);

let result = parser.parse(&source);
println!("Parsed Dart functions successfully.");
```

### Async/Await Parsing
```rust
use oak_dart::{Parser, DartLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
import 'dart:convert';
import 'package:http/http.dart' as http;

Future<Map<String, dynamic>> fetchData(String url) async {
  final response = await http.get(Uri.parse(url));
  
  if (response.statusCode == 200) {
    return jsonDecode(response.body) as Map<String, dynamic>;
  } else {
    throw Exception('Failed to load data');
  }
}

void main() async {
  try {
    final data = await fetchData('https://api.example.com/data');
    print('Data loaded: $data');
  } catch (e) {
    print('Error: $e');
  }
}
    "#);

let result = parser.parse(&source);
println!("Parsed Dart async code successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_dart::{Parser, DartLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("void main() { print('Hello'); }");
let result = parser.parse(&source);
// Token information is available in the parse result
```

### Error Handling
```rust
use oak_dart::{Parser, DartLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
void main() {
  print('Hello, Dart!'
  // Missing closing parenthesis
}
    "#);

let result = parser.parse(&source);
if let Err(e) = result.result {
    println!("Parse error: {:?}", e);
}
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **CompilationUnit**: Root container for Dart programs
- **ClassDeclaration**: Class definitions
- **FunctionDeclaration**: Function declarations and definitions
- **MethodDeclaration**: Method definitions within classes
- **VariableDeclaration**: Variable declarations
- **Expression**: Various expression types
- **Statement**: Control flow and other statements

## 📊 Performance

- **Streaming**: Parse large Dart files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak Dart integrates seamlessly with:

- **Compilers**: Front-end for Dart compilers
- **Static Analysis Tools**: Code quality and security analysis
- **IDE Support**: Language server protocol compatibility
- **Code Generation**: Generating code from AST

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete Dart program parsing
- Class and function analysis
- Code transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-dart) or open [issues](https://github.com/ygg-lang/oaks/issues).