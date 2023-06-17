# Oak Stylus Parser

[![Crates.io](https://img.shields.io/crates/v/oak-stylus.svg)](https://crates.io/crates/oak-stylus)
[![Documentation](https://docs.rs/oak-stylus/badge.svg)](https://docs.rs/oak-stylus)

A high-performance Stylus CSS preprocessor parser for Rust, built with the Oak parser combinator framework. Parse Stylus stylesheets with comprehensive AST generation and error handling.

## Overview

Oak Stylus provides robust parsing capabilities for Stylus stylesheets, supporting variables, mixins, functions, conditionals, and all major Stylus constructs. Built on the Oak parser combinator framework, it delivers excellent performance and detailed error messages.

## Features

- ✅ **Complete Stylus Support**: Parse variables, mixins, functions, and conditionals
- ✅ **Modern Rust API**: Type-safe parsing with comprehensive error handling
- ✅ **High Performance**: Built on the efficient Oak parser combinator framework
- ✅ **Rich AST**: Detailed Abstract Syntax Tree with source location tracking
- ✅ **Extensible**: Easy to extend for custom Stylus dialects
- ✅ **Well Tested**: Comprehensive test suite with real-world examples

## 🚀 Quick Start

## 📋 Parsing Examples

### Basic Stylus Stylesheet Parsing

```rust
use oak::{Parser, Language};
use oak_stylus::StylusLanguage;

fn main() {
    let source = r#"
        // Variables
        primary-color = #3498db
        secondary-color = #2ecc71
        font-size = 16px
        
        // Base styles
        body
            font-size font-size
            font-family 'Helvetica Neue', Arial, sans-serif
            color #333
            background-color #f8f9fa
            margin 0
            padding 0
        
        // Component styles
        .button
            display inline-block
            padding 10px 20px
            background-color primary-color
            color white
            border-radius 4px
            text-decoration none
            transition background-color 0.3s ease
            
            &:hover
                background-color darken(primary-color, 10%)
            
            &.secondary
                background-color secondary-color
                
                &:hover
                    background-color darken(secondary-color, 10%)
        
        // Responsive styles
        @media (max-width: 768px)
            body
                font-size 14px
            
            .button
                padding 8px 16px
                font-size 14px
    "#;
    
    let mut parser = Parser::<StylusLanguage>::new();
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

### Advanced Stylus with Mixins and Functions

```rust
use oak::{Parser, Language};
use oak_stylus::StylusLanguage;

fn main() {
    let source = r#"
        // Utility functions
        border-radius(n)
            -webkit-border-radius n
            -moz-border-radius n
            border-radius n
        
        box-shadow()
            -webkit-box-shadow arguments
            -moz-box-shadow arguments
            box-shadow arguments
        
        // Mixin for flexbox
        flex-center()
            display flex
            justify-content center
            align-items center
        
        // Color functions
        primary = #3498db
        secondary = #e74c3c
        
        // Component library
        .card
            background-color white
            border-radius 8px
            box-shadow 0 2px 4px rgba(0,0,0,0.1)
            padding 20px
            margin 10px
            transition transform 0.2s ease
            
            &:hover
                transform translateY(-2px)
                box-shadow 0 4px 8px rgba(0,0,0,0.15)
        
        .navbar
            background-color primary
            color white
            padding 1rem 0
            
            .nav-container
                max-width 1200px
                margin 0 auto
                padding 0 20px
                flex-center()
                
            .nav-brand
                font-size 1.5rem
                font-weight bold
                text-decoration none
                color white
                
            .nav-links
                list-style none
                margin 0
                padding 0
                display flex
                gap 2rem
                
                li
                    a
                        color white
                        text-decoration none
                        padding 0.5rem 1rem
                        border-radius 4px
                        transition background-color 0.3s ease
                        
                        &:hover
                            background-color rgba(255,255,255,0.1)
        
        // Grid system
        .grid
            display grid
            grid-template-columns repeat(auto-fit, minmax(300px, 1fr))
            gap 20px
            padding 20px
        
        // Responsive design
        for size in (mobile tablet desktop)
            .{size}-hide
                if size == mobile
                    @media (max-width: 767px)
                        display none
                else if size == tablet
                    @media (min-width: 768px) and (max-width: 1023px)
                        display none
                else if size == desktop
                    @media (min-width: 1024px)
                        display none
    "#;
    
    let mut parser = Parser::<StylusLanguage>::new();
    match parser.parse(&source) {
        Ok(ast) => {
            println!("Advanced stylesheet parsed successfully!");
        }
        Err(error) => {
            eprintln!("Parse error: {}", error);
        }
    }
}
```

## Advanced Features

### Conditional Logic

Oak Stylus supports parsing conditional statements:

```rust
let source = r#"
    theme = dark
    
    body
        if theme == dark
            background-color #1a1a1a
            color #ffffff
        else
            background-color #ffffff
            color #333333
    
    // Conditional mixins
    responsive-padding(size)
        padding size
        
        if size > 20px
            @media (max-width: 768px)
                padding size * 0.5
        else
            @media (max-width: 768px)
                padding size * 0.8
"#;
```

### Iterations and Loops

Parse loops and iterations:

```rust
let source = r#"
    // Generate utility classes
    for size in (xs sm md lg xl)
        .margin-{size}
            margin lookup('spacing-' + size)
        
        .padding-{size}
            padding lookup('spacing-' + size)
    
    // Generate color variants
    colors = {
        primary: #3498db,
        secondary: #2ecc71,
        danger: #e74c3c,
        warning: #f39c12
    }
    
    for name, color in colors
        .btn-{name}
            background-color color
            color white
            
            &:hover
                background-color darken(color, 10%)
"#;
```

### Interpolation and String Operations

Handle string interpolation:

```rust
let source = r#"
    base-font-size = 16px
    scale-ratio = 1.25
    
    // Generate heading sizes
    for i in (1..6)
        h{i}
            font-size base-font-size * pow(scale-ratio, 6 - i)
            line-height 1.2
            margin-bottom 0.5em
    
    // Dynamic class names
    component = 'button'
    state = 'hover'
    
    .{component}
        &.{state}
            background-color lighten(primary-color, 20%)
"#;
```

## AST Structure

The parser generates a rich AST with the following main node types:

- `StylusFile` - Root node containing the entire stylesheet
- `Rule` - CSS rules with selectors and declarations
- `Selector` - CSS selectors (class, id, element, pseudo)
- `Declaration` - Property-value pairs
- `Variable` - Stylus variables
- `Function` - Function definitions
- `Mixin` - Mixin definitions
- `Conditional` - If/else statements
- `Iteration` - For loops
- `Comment` - Single-line and multi-line comments

## Performance

Oak Stylus is designed for high performance:

- **Zero-copy parsing** where possible
- **Streaming support** for large stylesheets
- **Efficient memory usage** with minimal allocations
- **Fast error recovery** for better developer experience

## Integration

Oak Stylus integrates seamlessly with the Oak ecosystem:

```rust
use oak::{Parser, Language};
use oak_stylus::StylusLanguage;

// Use with other Oak parsers
let mut parser = Parser::<StylusLanguage>::new();
let result = parser.parse(stylus_source);
```

## Examples

More examples can be found in the [examples directory](https://github.com/axodotdev/oak/tree/main/examples/oak-stylus/examples):

- [Basic stylesheets](examples/basic.rs)
- [Mixins and functions](examples/mixins.rs)
- [Responsive design](examples/responsive.rs)
- [Color functions](examples/colors.rs)
- [Error handling](examples/error_handling.rs)

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-stylus) or open [issues](https://github.com/ygg-lang/oaks/issues).