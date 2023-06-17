# Oak Handlebars Parser

[![Crates.io](https://img.shields.io/crates/v/oak-handlebars.svg)](https://crates.io/crates/oak-handlebars)
[![Documentation](https://docs.rs/oak-handlebars/badge.svg)](https://docs.rs/oak-handlebars)

A high-performance Handlebars template parser for Rust, built with the Oak parser combinator framework. Parse Handlebars templates with comprehensive AST generation and error handling.

## Overview

Oak Handlebars provides robust parsing capabilities for Handlebars template files, supporting variables, helpers, partials, block helpers, and all major Handlebars constructs. Built on the Oak parser combinator framework, it delivers excellent performance and detailed error messages.

## Features

- ✅ **Complete Handlebars Support**: Parse variables, helpers, partials, and block helpers
- ✅ **Modern Rust API**: Type-safe parsing with comprehensive error handling
- ✅ **High Performance**: Built on the efficient Oak parser combinator framework
- ✅ **Rich AST**: Detailed Abstract Syntax Tree with source location tracking
- ✅ **Extensible**: Easy to extend for custom Handlebars dialects
- ✅ **Well Tested**: Comprehensive test suite with real-world examples

## Quick Start


## Parsing Examples

### Basic Template Parsing

```rust
use oak::{Parser, Language};
use oak_handlebars::HandlebarsLanguage;

fn main() {
    let source = r#"
        <div class="user-profile">
            <h1>{{user.name}}</h1>
            <p>{{user.bio}}</p>
            <ul>
                {{#each user.skills}}
                    <li>{{this}}</li>
                {{/each}}
            </ul>
        </div>
    "#;
    
    let mut parser = Parser::<HandlebarsLanguage>::new();
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

### Advanced Template with Partials and Helpers

```rust
use oak::{Parser, Language};
use oak_handlebars::HandlebarsLanguage;

fn main() {
    let source = r#"
        {{> header title="My Blog"}}
        
        <main>
            {{#if posts}}
                <h1>Recent Posts</h1>
                {{#each posts}}
                    <article>
                        <h2><a href="/posts/{{id}}">{{title}}</a></h2>
                        <p class="meta">
                            By {{author.name}} on {{formatDate date}}
                        </p>
                        <div class="excerpt">
                            {{truncate content 200}}
                        </div>
                        <a href="/posts/{{id}}">Read more...</a>
                    </article>
                {{/each}}
                
                {{#if showPagination}}
                    {{> pagination current=currentPage total=totalPages}}
                {{/if}}
            {{else}}
                <p>No posts found.</p>
            {{/if}}
        </main>
        
        {{> footer}}
    "#;
    
    let mut parser = Parser::<HandlebarsLanguage>::new();
    match parser.parse(&source) {
        Ok(ast) => {
            println!("Advanced template parsed successfully!");
        }
        Err(error) => {
            eprintln!("Parse error: {}", error);
        }
    }
}
```

## Advanced Features

### Custom Helpers

Oak Handlebars supports parsing custom helper definitions:

```rust
let source = r#"
    {{#uppercase}}
        hello world
    {{/uppercase}}
    
    {{#repeat 3}}
        <p>Item {{@index}}</p>
    {{/repeat}}
"#;
```

### Whitespace Control

Parse templates with whitespace control:

```rust
let source = r#"
    <ul>
        {{~#each items~}}
            <li>{{name}}</li>
        {{~/each~}}
    </ul>
"#;
```

### Subexpressions

Parse complex subexpressions:

```rust
let source = r#"
    {{#each (filter posts "published")}}
        <article>{{title}}</article>
    {{/each}}
    
    {{#if (and user.isAdmin (gt posts.length 0))}}
        <div>Admin controls here</div>
    {{/if}}
"#;
```

## AST Structure

The parser generates a rich AST with the following main node types:

- `HandlebarsFile` - Root node containing the entire template
- `Text` - Static text content
- `Variable` - Variable references like {{name}}
- `Helper` - Helper calls like {{helper arg}}
- `BlockHelper` - Block helpers like {{#each}}...{{/each}}
- `Partial` - Partial includes like {{> partial}}
- `Comment` - Handlebars comments {{!-- comment --}}
- `Subexpression` - Nested subexpressions

## Performance

Oak Handlebars is designed for high performance:

- **Zero-copy parsing** where possible
- **Streaming support** for large template files
- **Efficient memory usage** with minimal allocations
- **Fast error recovery** for better developer experience

## Integration

Oak Handlebars integrates seamlessly with the Oak ecosystem:

```rust
use oak::{Parser, Language};
use oak_handlebars::HandlebarsLanguage;

// Use with other Oak parsers
let mut parser = Parser::<HandlebarsLanguage>::new();
let result = parser.parse(handlebars_source);
```

## Examples

More examples can be found in the [examples directory](https://github.com/axodotdev/oak/tree/main/examples/oak-handlebars/examples):

- [Basic templates](examples/basic.rs)
- [Block helpers](examples/block_helpers.rs)
- [Partials](examples/partials.rs)
- [Custom helpers](examples/custom_helpers.rs)
- [Error handling](examples/error_handling.rs)

## Contributing

We welcome contributions! Please see our [Contributing Guide](https://github.com/axodotdev/oak/blob/main/CONTRIBUTING.md) for details.