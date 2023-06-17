# Oak Sass Parser

[![Crates.io](https://img.shields.io/crates/v/oak-sass.svg)](https://crates.io/crates/oak-sass)
[![Documentation](https://docs.rs/oak-sass/badge.svg)](https://docs.rs/oak-sass)


A high-performance Sass/SCSS parser for Rust, built with the Oak parser combinator framework. Parse stylesheets with comprehensive AST generation and error handling.

## Overview

Oak Sass provides robust parsing capabilities for Sass and SCSS stylesheet files, supporting variables, mixins, functions, nesting, and all major Sass constructs. Built on the Oak parser combinator framework, it delivers excellent performance and detailed error messages.

## Features

- ✅ **Complete Sass Support**: Parse variables, mixins, functions, nesting, and imports
- ✅ **Modern Rust API**: Type-safe parsing with comprehensive error handling
- ✅ **High Performance**: Built on the efficient Oak parser combinator framework
- ✅ **Rich AST**: Detailed Abstract Syntax Tree with source location tracking
- ✅ **Extensible**: Easy to extend for custom Sass dialects
- ✅ **Well Tested**: Comprehensive test suite with real-world examples

## Quick Start


## Parsing Examples

### Basic SCSS Parsing

```rust
use oak::{Parser, Language};
use oak_sass::SassLanguage;

fn main() {
    let source = r#"
        $primary-color: #3498db;
        $secondary-color: #2ecc71;
        $font-size: 16px;
        
        .button {
            background-color: $primary-color;
            color: white;
            padding: 10px 20px;
            border-radius: 5px;
            
            &:hover {
                background-color: darken($primary-color, 10%);
            }
            
            &.large {
                font-size: $font-size * 1.2;
                padding: 15px 30px;
            }
        }
    "#;
    
    let mut parser = Parser::<SassLanguage>::new();
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

### Advanced Mixins and Functions

```rust
use oak::{Parser, Language};
use oak_sass::SassLanguage;

fn main() {
    let source = r#"
        @mixin flex-center {
            display: flex;
            justify-content: center;
            align-items: center;
        }
        
        @function rem($pixels, $base: 16px) {
            @return $pixels / $base * 1rem;
        }
        
        @mixin responsive-font($min-size, $max-size, $min-width: 320px, $max-width: 1200px) {
            font-size: $min-size;
            
            @media screen and (min-width: $min-width) {
                font-size: calc(#{$min-size} + #{strip-unit($max-size - $min-size)} * 
                    ((100vw - #{$min-width}) / #{strip-unit($max-width - $min-width)}));
            }
            
            @media screen and (min-width: $max-width) {
                font-size: $max-size;
            }
        }
        
        .container {
            @include flex-center;
            min-height: 100vh;
            
            .content {
                @include responsive-font(14px, 24px);
                padding: rem(20px);
                max-width: rem(800px);
            }
        }
    "#;
    
    let mut parser = Parser::<SassLanguage>::new();
    match parser.parse(&source) {
        Ok(ast) => {
            println!("Advanced Sass parsed successfully!");
        }
        Err(error) => {
            eprintln!("Parse error: {}", error);
        }
    }
}
```

## Advanced Features

### Nested Imports

Oak Sass supports parsing nested imports:

```rust
let source = r#"
    @import 'variables';
    @import 'mixins';
    
    @import url('https://fonts.googleapis.com/css?family=Roboto');
"#;
```

### Control Directives

Parse control directives like @if, @for, @each, and @while:

```rust
let source = r#"
    @for $i from 1 through 12 {
        .col-#{$i} {
            width: percentage($i / 12);
        }
    }
    
    @each $color in red, green, blue {
        .bg-#{$color} {
            background-color: $color;
        }
    }
"#;
```

## AST Structure

The parser generates a rich AST with the following main node types:

- `SassFile` - Root node containing the entire file
- `Variable` - Variable declarations and references
- `Rule` - CSS rules with selectors and declarations
- `Mixin` - Mixin definitions and includes
- `Function` - Function definitions and calls
- `Import` - Import statements
- `Media` - Media queries and blocks
- `Expression` - Sass expressions and operations
- `AtRule` - At-rules like @extend, @debug, @warn

## Performance

Oak Sass is designed for high performance:

- **Zero-copy parsing** where possible
- **Streaming support** for large stylesheet files
- **Efficient memory usage** with minimal allocations
- **Fast error recovery** for better developer experience

## Integration

Oak Sass integrates seamlessly with the Oak ecosystem:

```rust
use oak::{Parser, Language};
use oak_sass::SassLanguage;

// Use with other Oak parsers
let mut parser = Parser::<SassLanguage>::new();
let result = parser.parse(sass_source);
```

## Examples

More examples can be found in the [examples directory](https://github.com/axodotdev/oak/tree/main/examples/oak-sass/examples):

- [Basic SCSS parsing](examples/basic.rs)
- [Variables and mixins](examples/variables_mixins.rs)
- [Nested selectors](examples/nesting.rs)
- [Functions and operations](examples/functions.rs)
- [Error handling](examples/error_handling.rs)

## Contributing

We welcome contributions! Please see our [Contributing Guide](https://github.com/axodotdev/oak/blob/main/CONTRIBUTING.md) for details.