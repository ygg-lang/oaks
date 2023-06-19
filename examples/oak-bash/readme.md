# Oak Bash Parser

[![Crates.io](https://img.shields.io/crates/v/oak-bash.svg)](https://crates.io/crates/oak-bash)
[![Documentation](https://docs.rs/oak-bash/badge.svg)](https://docs.rs/oak-bash)

High-performance incremental Bash parser for the oak ecosystem with flexible configuration, optimized for script analysis and automation.

## 🎯 Overview

Oak Bash is a robust parser for Bash, designed to handle complete Bash syntax including modern features like conditionals, loops, and functions. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for script analysis and automation.

## ✨ Features

- **Complete Bash Syntax**: Supports all Bash features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_bash::{Parser, BashLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
        #!/bin/bash
        NAME="World"
        echo "Hello, $NAME!"
        if [ "$NAME" == "World" ]; then
            echo "It's a small world."
        fi
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed Bash script successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Script Parsing
```rust
use oak_bash::{Parser, BashLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
    #!/bin/bash
    echo "Hello"
    "#);

let result = parser.parse(&source);
println!("Parsed Bash script successfully.");
```

### Command Parsing
```rust
use oak_bash::{Parser, BashLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
    ls -la /tmp
    grep "error" /var/log/syslog
    "#);

let result = parser.parse(&source);
println!("Parsed Bash commands successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_bash::{Parser, BashLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("echo \"Hello World\"");
let result = parser.parse(&source);
// Token information is available in the parse result
```

### Error Handling
```rust
use oak_bash::{Parser, BashLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
    if [ -f /tmp/file ]; then
        echo "File exists"
    else
        echo "File does not exist"
    fi_invalid
    "#);

let result = parser.parse(&source);
if let Err(e) = result.result {
    println!("Parse error: {:?}", e);
}
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **Script**: Root container for Bash scripts
- **Command**: Simple commands, pipelines, conditionals, loops
- **Expression**: Arithmetic, string, file test expressions
- **Word**: Literal words, variables, command substitutions

## 📊 Performance

- **Streaming**: Parse large Bash files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak Bash integrates seamlessly with:

- **Shell Scripting Tools**: Integration with linters, formatters
- **Automation**: Parsing scripts for automated tasks
- **IDE Support**: Language server protocol compatibility
- **Security Analysis**: Analyzing scripts for vulnerabilities

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete Bash script parsing
- Command and expression analysis
- Code transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-bash) or open [issues](https://github.com/ygg-lang/oaks/issues).