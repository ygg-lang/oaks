# Oak Regex Parser

[![Crates.io](https://img.shields.io/crates/v/oak-regex.svg)](https://crates.io/crates/oak-regex)
[![Documentation](https://docs.rs/oak-regex/badge.svg)](https://docs.rs/oak-regex)

High-performance incremental regular expression parser for the oak ecosystem with flexible configuration, optimized for pattern matching and text processing.

## 🎯 Overview

Oak Regex is a robust parser for regular expressions, designed to handle complete regex syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for pattern matching and text processing.

## ✨ Features

- **Complete Regex Syntax**: Supports all regex features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_regex::{Parser, RegexLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
        [a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed regex successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Email Pattern Parsing
```rust
use oak_regex::{Parser, RegexLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
    [a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}
"#);

let result = parser.parse(&source);
println!("Email pattern parsed successfully.");
```

### Complex Pattern Parsing
```rust
use oak_regex::{Parser, RegexLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
    (?:(?<=\s)|^)(?:https?:\/\/)?(?:www\.)?[a-zA-Z0-9-]+\.[a-zA-Z]{2,}(?:\/\S*)?(?=\s|$)
"#);

let result = parser.parse(&source);
println!("URL pattern parsed successfully.");
```

### Group and Alternation Parsing
```rust
use oak_regex::{Parser, RegexLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
    (?:cat|dog|bird)(?:\s+(?:is|are)\s+(?:brown|black|white))?
"#);

let result = parser.parse(&source);
println!("Group and alternation parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_regex::{Parser, RegexLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"\d{3}-\d{2}-\d{4}"#);
let result = parser.parse(&source);
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_regex::{Parser, RegexLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
    [a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}
    // Unclosed character class
    [a-z
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

- **RegexRoot**: Root container for regular expressions
- **Alternation**: Alternation expressions (|)
- **Group**: Grouping expressions (capturing and non-capturing)
- **Quantifier**: Quantifiers (?, *, +, {n,m})
- **CharacterClass**: Character classes and ranges
- **Assertion**: Assertions (^, $, \b, \B, lookaround)
- **Literal**: Literal characters and escaped sequences

## 📊 Performance

- **Streaming**: Parse large regex patterns without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak Regex integrates seamlessly with:

- **Text Processing**: Pattern matching and text extraction
- **Validation**: Input validation and sanitization
- **IDE Support**: Syntax highlighting and autocompletion for regex
- **Code Generation**: Generating code from regex patterns
- **Static Analysis**: Analyzing regex patterns for optimization

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete regex pattern parsing
- Complex pattern analysis
- Pattern optimization
- Integration with text processing workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-regex) or open [issues](https://github.com/ygg-lang/oaks/issues).