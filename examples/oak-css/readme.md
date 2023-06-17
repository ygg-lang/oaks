# Oak CSS Parser

[![Crates.io](https://img.shields.io/crates/v/oak-css.svg)](https://crates.io/crates/oak-css)
[![Documentation](https://docs.rs/oak-css/badge.svg)](https://docs.rs/oak-css)

High-performance incremental CSS parser for the oak ecosystem with flexible configuration, optimized for web development and styling analysis.

## 🎯 Overview

Oak-css is a robust parser for CSS, designed to handle complete CSS syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for web development and styling analysis.

## ✨ Features

- **Complete CSS Syntax**: Supports all CSS features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_css::CssParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = CssParser::new();
    let css_content = r#"
body {
    margin: 0;
    padding: 0;
    font-family: Arial, sans-serif;
}

.container {
    max-width: 1200px;
    margin: 0 auto;
}
    "#;
    
    let stylesheet = parser.parse_stylesheet(css_content)?;
    println!("Parsed CSS stylesheet successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Rule Parsing
```rust
use oak_css::{CssParser, ast::Rule};

let parser = CssParser::new();
let css_content = "h1 { color: red; font-size: 2em; font-weight: bold; }";

let rule = parser.parse_rule(css_content)?;
println!("Selector: {}", rule.selector);
println!("Declarations: {}", rule.declarations.len());
```

### Selector Parsing
```rust
use oak_css::{CssParser, ast::Selector};

let parser = CssParser::new();
let selector = parser.parse_selector(".container .item:hover > .child")?;
println!("Selector complexity: {}", selector.complexity());
println!("Specificity: {:?}", selector.specificity());
```

### Declaration Parsing
```rust
use oak_css::{CssParser, ast::Declaration};

let parser = CssParser::new();
let declaration = parser.parse_declaration("margin: 10px 5px 20px 15px")?;
println!("Property: {}", declaration.property);
println!("Value tokens: {}", declaration.value.len());
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_css::{CssParser, lexer::Token};

let parser = CssParser::new();
let tokens = parser.tokenize(".class { color: #ff0000; }")?;
for token in tokens {
    println!("{:?}", token.kind);
}
```

### Error Handling
```rust
use oak_css::CssParser;

let parser = CssParser::new();
let invalid_css = r#"
body {
    color: red
    font-size: 16px;  // Missing semicolon
    margin: 10px
}
"#;

match parser.parse_stylesheet(invalid_css) {
    Ok(stylesheet) => println!("Parsed CSS stylesheet successfully."),
    Err(e) => {
        println!("Parse error at line {} column {}: {}", 
            e.line(), e.column(), e.message());
        if let Some(context) = e.context() {
            println!("Error context: {}", context);
        }
    }
}
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **Stylesheet**: Root container for CSS rules
- **Rule**: CSS rules with selectors and declarations
- **Selector**: CSS selectors with specificity calculation
- **Declaration**: Property-value pairs
- **Function**: CSS functions like `rgb()`, `calc()`, etc.
- **AtRule**: CSS at-rules like `@media`, `@import`, etc.

## 📊 Performance

- **Streaming**: Parse large CSS files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak-css integrates seamlessly with:

- **Web Development**: Parse CSS for web applications
- **Styling Analysis**: Analyze CSS for optimization and linting
- **Build Tools**: Integrate with CSS processing pipelines
- **IDE Support**: Language server protocol compatibility
- **Preprocessors**: Handle CSS-like preprocessor syntax

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete CSS stylesheet parsing
- Selector analysis and specificity calculation
- Property validation and transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-css) or open [issues](https://github.com/ygg-lang/oaks/issues).