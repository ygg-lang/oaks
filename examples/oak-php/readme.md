# Oak PHP Parser

[![Crates.io](https://img.shields.io/crates/v/oak-php.svg)](https://crates.io/crates/oak-php)
[![Documentation](https://docs.rs/oak-php/badge.svg)](https://docs.rs/oak-php)

High-performance incremental PHP parser for the oak ecosystem with flexible configuration, optimized for static analysis and code generation.

## 🎯 Overview

Oak PHP is a robust parser for PHP, designed to handle complete PHP syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for static analysis and code generation.

## ✨ Features

- **Complete PHP Syntax**: Supports all PHP features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_php::{PhpParser, PhpLanguage};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut session = ParseSession::<PhpLanguage>::default();
    let parser = PhpParser::new();
    let source = SourceText::new(r#"
<?php
echo "Hello, World!";

$name = "PHP";
echo "Welcome to $name!";
?>
    "#);
    
    let result = parser.parse(&[], &mut session);
    println!("Parsed PHP successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Function Parsing
```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_php::{PhpParser, PhpLanguage};

let mut session = ParseSession::<PhpLanguage>::default();
let parser = PhpParser::new();
let source = SourceText::new(r#"
<?php
function add($a, $b) {
    return $a + $b;
}

$result = add(5, 3);
echo "Result: $result";
?>
"#);

let result = parser.parse(&[], &mut session);
println!("Function parsed successfully.");
```

### Class Parsing
```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_php::{PhpParser, PhpLanguage};

let mut session = ParseSession::<PhpLanguage>::default();
let parser = PhpParser::new();
let source = SourceText::new(r#"
<?php
class Calculator {
    private $result;
    
    public function __construct() {
        $this->result = 0;
    }
    
    public function add($value) {
        $this->result += $value;
        return $this;
    }
    
    public function getResult() {
        return $this->result;
    }
}
?>
"#);

let result = parser.parse(&[], &mut session);
println!("Class parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_core::{Parser, SourceText, parser::session::ParseSession};
use oak_php::{PhpParser, PhpLanguage};

let mut session = ParseSession::<PhpLanguage>::default();
let parser = PhpParser::new();
let source = SourceText::new("<?php $x = 42; ?>");
let result = parser.parse(&[], &mut session);
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_php::{Parser, PhpLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
<?php
// Invalid PHP code example
function broken_function(
    echo "Hello"
// Missing closing brace
?>
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

- **PhpProgram**: Root container for PHP programs
- **Function**: PHP functions and methods
- **Class**: PHP class definitions
- **Statement**: Various statement types including control flow
- **Expression**: Various expression types including operators
- **Variable**: PHP variable constructs

## 📊 Performance

- **Streaming**: Parse large PHP files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak PHP integrates seamlessly with:

- **Static Analysis**: Code quality and security analysis
- **Code Generation**: Generating code from PHP AST
- **IDE Support**: Language server protocol compatibility
- **Refactoring**: Automated code refactoring
- **Documentation**: Generating documentation from PHP code

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete PHP program parsing
- Function and class analysis
- Code transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-php) or open [issues](https://github.com/ygg-lang/oaks/issues).