# Oak Perl Parser

[![Crates.io](https://img.shields.io/crates/v/oak-perl.svg)](https://crates.io/crates/oak-perl)
[![Documentation](https://docs.rs/oak-perl/badge.svg)](https://docs.rs/oak-perl)

High-performance incremental Perl parser for the oak ecosystem with flexible configuration, optimized for static analysis and code generation.

## 🎯 Overview

Oak Perl is a robust parser for Perl, designed to handle complete Perl syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for static analysis and code generation.

## ✨ Features

- **Complete Perl Syntax**: Supports all Perl features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_perl::{Parser, PerlLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
print "Hello, World!\n";
my $name = "Perl";
print "Welcome to $name!\n";
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed Perl successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Function Parsing
```rust
use oak_perl::{Parser, PerlLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
sub add {
    my ($a, $b) = @_;
    return $a + $b;
}
"#);

let result = parser.parse(&source);
println!("Function parsed successfully.");
```

### Regular Expression Parsing
```rust
use oak_perl::{Parser, PerlLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
if ($text =~ /pattern/) {
    print "Match found!\n";
}
"#);

let result = parser.parse(&source);
println!("Regular expression parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_perl::{Parser, PerlLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("my $x = 42;");
let result = parser.parse(&source);
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_perl::{Parser, PerlLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
# Invalid Perl code example
sub missing_brace {
    print "Hello"
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

- **PerlProgram**: Root container for Perl programs
- **Function**: Perl functions and subroutines
- **Statement**: Various statement types
- **Expression**: Various expression types
- **RegularExpression**: Perl regex patterns

## 📊 Performance

- **Streaming**: Parse large Perl files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak Perl integrates seamlessly with:

- **Static Analysis**: Code quality and security analysis
- **Code Generation**: Generating code from Perl AST
- **IDE Support**: Language server protocol compatibility
- **Refactoring**: Automated code refactoring
- **Documentation**: Generating documentation from Perl code

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete Perl program parsing
- Function and regex analysis
- Code transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-perl) or open [issues](https://github.com/ygg-lang/oaks/issues).