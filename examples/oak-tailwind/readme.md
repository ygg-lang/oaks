# Oak Twig Parser

[![Crates.io](https://img.shields.io/crates/v/oak-twig.svg)](https://crates.io/crates/oak-twig)
[![Documentation](https://docs.rs/oak-twig/badge.svg)](https://docs.rs/oak-twig)

A high-performance Twig template parser for Rust, built with the Oak parser combinator framework. Parse Twig templates with comprehensive AST generation and error handling.

## Overview

Oak Twig provides robust parsing capabilities for Twig template files, supporting variables, filters, functions, tags, and all major Twig constructs. Built on the Oak parser combinator framework, it delivers excellent performance and detailed error messages.

## Features

- ✅ **Complete Twig Support**: Parse variables, filters, functions, tags, and blocks
- ✅ **Modern Rust API**: Type-safe parsing with comprehensive error handling
- ✅ **High Performance**: Built on the efficient Oak parser combinator framework
- ✅ **Rich AST**: Detailed Abstract Syntax Tree with source location tracking
- ✅ **Extensible**: Easy to extend for custom Twig dialects
- ✅ **Well Tested**: Comprehensive test suite with real-world examples

## Quick Start

Add Oak Twig to your `Cargo.toml`:

```toml
[dependencies]
oak = "0.1.0"
oak-twig = "0.1.0"
```

## Parsing Examples

### Basic Template Parsing

```rust
use oak::{Parser, Language};
use oak_twig::TwigLanguage;

fn main() {
    let source = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>{{ title|e }}</title>
        </head>
        <body>
            <h1>{{ heading }}</h1>
            <ul>
                {% for item in items %}
                    <li>{{ item.name }} - {{ item.price|currency }}</li>
                {% endfor %}
            </ul>
            <p>Total: {{ items|length }} items</p>
        </body>
        </html>
    "#;
    
    let mut parser = Parser::<TwigLanguage>::new();
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

### Advanced Template with Inheritance

```rust
use oak::{Parser, Language};
use oak_twig::TwigLanguage;

fn main() {
    let source = r#"
        {% extends "base.html" %}
        
        {% block title %}Product Listing{% endblock %}
        
        {% block content %}
            <div class="products">
                <h1>{{ category.name }}</h1>
                
                {% if products is not empty %}
                    <div class="product-grid">
                        {% for product in products %}
                            <div class="product-card">
                                {% if product.image %}
                                    <img src="{{ product.image.url }}" alt="{{ product.image.alt }}">
                                {% endif %}
                                <h3>{{ product.name }}</h3>
                                <p class="price">{{ product.price|number_format(2, '.', ',') }}</p>
                                <p class="description">{{ product.description|truncate(100) }}</p>
                                
                                {% if product.in_stock %}
                                    <button data-id="{{ product.id }}">Add to Cart</button>
                                {% else %}
                                    <span class="out-of-stock">Out of Stock</span>
                                {% endif %}
                            </div>
                        {% endfor %}
                    </div>
                    
                    {% include 'pagination.html' with {'current': current_page, 'total': total_pages} %}
                {% else %}
                    <p>No products found in this category.</p>
                {% endif %}
            </div>
        {% endblock %}
    "#;
    
    let mut parser = Parser::<TwigLanguage>::new();
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

### Custom Functions and Filters

Oak Twig supports parsing custom functions and filters:

```rust
let source = r#"
    {% set price = calculate_tax(product.price, tax_rate) %}
    {% set discounted = apply_discount(price, discount) %}
    
    <p>Original: {{ product.price|currency }}</p>
    <p>With tax: {{ price|currency }}</p>
    <p>Final: {{ discounted|currency|default('N/A') }}</p>
    
    {% set categories = product.categories|filter(category => category.active) %}
    {% set tags = product.tags|map(tag => tag.name)|join(', ') %}
"#;
```

### Macros

Parse Twig macros:

```rust
let source = r#"
    {% macro input(name, value, type = 'text', size = 20) %}
        <input type="{{ type }}" name="{{ name }}" value="{{ value|e }}" size="{{ size }}" />
    {% endmacro %}
    
    {% macro textarea(name, value, rows = 10, cols = 40) %}
        <textarea name="{{ name }}" rows="{{ rows }}" cols="{{ cols }}">{{ value|e }}</textarea>
    {% endmacro %}
    
    {{ input('username', user.name) }}
    {{ textarea('bio', user.bio, rows=5) }}
"#;
```

### Conditional Logic

Parse complex conditional logic:

```rust
let source = r#"
    {% if product.price > 100 and product.category == 'electronics' %}
        <span class="badge premium">Premium Product</span>
    {% elseif product.sale_price %}
        <span class="badge sale">On Sale - Save {{ product.price - product.sale_price|round }}</span>
    {% else %}
        <span class="badge standard">Standard Product</span>
    {% endif %}
    
    {% set class = 'product-card' %}
    {% if product.featured %}
        {% set class = class ~ ' featured' %}
    {% endif %}
    {% if product.new %}
        {% set class = class ~ ' new' %}
    {% endif %}
    
    <div class="{{ class }}">
        <!-- product content -->
    </div>
"#;
```

## AST Structure

The parser generates a rich AST with the following main node types:

- `TwigFile` - Root node containing the entire template
- `Text` - Static text content
- `Variable` - Variable references like {{ name }}
- `Filter` - Filter applications like {{ name|upper }}
- `Function` - Function calls like {{ date(format='Y-m-d') }}
- `Tag` - Twig tags like {% for %}, {% if %}, etc.
- `Block` - Template blocks like {% block content %}
- `Comment` - Twig comments {# comment #}
- `Expression` - Complex expressions and operators

## Performance

Oak Twig is designed for high performance:

- **Zero-copy parsing** where possible
- **Streaming support** for large template files
- **Efficient memory usage** with minimal allocations
- **Fast error recovery** for better developer experience

## Integration

Oak Twig integrates seamlessly with the Oak ecosystem:

```rust
use oak::{Parser, Language};
use oak_twig::TwigLanguage;

// Use with other Oak parsers
let mut parser = Parser::<TwigLanguage>::new();
let result = parser.parse(twig_source);
```

## Examples

More examples can be found in the [examples directory](https://github.com/axodotdev/oak/tree/main/examples/oak-twig/examples):

- [Basic templates](examples/basic.rs)
- [Template inheritance](examples/inheritance.rs)
- [Macros](examples/macros.rs)
- [Filters and functions](examples/filters.rs)
- [Error handling](examples/error_handling.rs)

## Contributing

We welcome contributions! Please see our [Contributing Guide](https://github.com/axodotdev/oak/blob/main/CONTRIBUTING.md) for details.