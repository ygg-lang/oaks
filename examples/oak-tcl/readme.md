# Oak Tcl Parser

[![Crates.io](https://img.shields.io/crates/v/oak-tcl.svg)](https://crates.io/crates/oak-tcl)
[![Documentation](https://docs.rs/oak-tcl/badge.svg)](https://docs.rs/oak-tcl)

A high-performance Tcl parser for Rust, built with the Oak parser combinator framework. Parse Tool Command Language scripts with comprehensive AST generation and error handling.

## Overview

Oak Tcl provides robust parsing capabilities for Tcl script files, supporting commands, procedures, control structures, and all major Tcl constructs. Built on the Oak parser combinator framework, it delivers excellent performance and detailed error messages.

## Features

- ✅ **Complete Tcl Support**: Parse commands, procedures, control structures, and variables
- ✅ **Modern Rust API**: Type-safe parsing with comprehensive error handling
- ✅ **High Performance**: Built on the efficient Oak parser combinator framework
- ✅ **Rich AST**: Detailed Abstract Syntax Tree with source location tracking
- ✅ **Extensible**: Easy to extend for custom Tcl dialects
- ✅ **Well Tested**: Comprehensive test suite with real-world examples

## Quick Start

## Parsing Examples

### Basic Command Parsing

```rust
use oak::{Parser, Language};
use oak_tcl::TclLanguage;

fn main() {
    let source = r#"
        set name "World"
        puts "Hello, $name!"
        
        set numbers {1 2 3 4 5}
        set sum 0
        foreach num $numbers {
            set sum [expr {$sum + $num}]
        }
        puts "Sum: $sum"
    "#;
    
    let mut parser = Parser::<TclLanguage>::new();
    match parser.parse(&source) {
        Ok(ast) => {
            println!("Parsed AST: {:#?}", ast);
        }
        Err(error) => {
            eprintln!("Parse error: {}", error);
        }
    }
}
```

### Procedure Definition

```rust
use oak::{Parser, Language};
use oak_tcl::TclLanguage;

fn main() {
    let source = r#"
        proc factorial {n} {
            if {$n <= 1} {
                return 1
            } else {
                return [expr {$n * [factorial [expr {$n - 1}]]}]
            }
        }
        
        proc greet {name {greeting "Hello"}} {
            return "$greeting, $name!"
        }
        
        puts [factorial 5]
        puts [greet "World"]
        puts [greet "World" "Hi"]
    "#;
    
    let mut parser = Parser::<TclLanguage>::new();
    match parser.parse(&source) {
        Ok(ast) => {
            println!("Procedures parsed successfully!");
        }
        Err(error) => {
            eprintln!("Parse error: {}", error);
        }
    }
}
```

## Advanced Features

### Lists and Arrays

Oak Tcl supports parsing complex list operations:

```rust
let source = r#"
    set mylist [list a b c d e]
    set element [lindex $mylist 2]
    set length [llength $mylist]
    set reversed [lreverse $mylist]
    
    array set colors {
        red   #FF0000
        green #00FF00
        blue  #0000FF
    }
    puts $colors(red)
"#;
```

### Regular Expressions

Parse regexp operations:

```rust
let source = r#"
    set text "Hello World 123"
    set pattern {\d+}
    if {[regexp $pattern $text match]} {
        puts "Found number: $match"
    }
    
    set result [regsub $pattern $text "XXX"]
    puts "Modified: $result"
"#;
```

## AST Structure

The parser generates a rich AST with the following main node types:

- `TclFile` - Root node containing the entire file
- `Command` - Tcl commands with arguments
- `Procedure` - Procedure definitions with parameters and body
- `Variable` - Variable references and assignments
- `Expression` - Expressions in square brackets
- `List` - List literals and operations
- `ControlFlow` - if, while, for, foreach statements
- `String` - String literals with interpolation

## Performance

Oak Tcl is designed for high performance:

- **Zero-copy parsing** where possible
- **Streaming support** for large script files
- **Efficient memory usage** with minimal allocations
- **Fast error recovery** for better developer experience

## Integration

Oak Tcl integrates seamlessly with the Oak ecosystem:

```rust
use oak::{Parser, Language};
use oak_tcl::TclLanguage;

// Use with other Oak parsers
let mut parser = Parser::<TclLanguage>::new();
let result = parser.parse(tcl_source);
```

## Examples

More examples can be found in the [examples directory](https://github.com/axodotdev/oak/tree/main/examples/oak-tcl/examples):

- [Basic commands](examples/commands.rs)
- [Procedures](examples/procedures.rs)
- [Control structures](examples/control.rs)
- [Lists and arrays](examples/lists.rs)
- [Error handling](examples/error_handling.rs)

## Contributing

We welcome contributions! Please see our [Contributing Guide](https://github.com/axodotdev/oak/blob/main/CONTRIBUTING.md) for details.