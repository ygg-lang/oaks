# Oak Javadoc Parser

[![Crates.io](https://img.shields.io/crates/v/oak-javadoc.svg)](https://crates.io/crates/oak-javadoc)
[![Documentation](https://docs.rs/oak-javadoc/badge.svg)](https://docs.rs/oak-javadoc)

High-performance incremental Javadoc parser for the oak ecosystem with flexible configuration, optimized for API documentation generation and code analysis.

## 🎯 Overview

Oak Javadoc is a robust parser for Java documentation comments (Javadoc), designed to handle complete Javadoc syntax including standard tags and custom extensions. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for documentation analysis and generation.

## ✨ Features

- **Complete Javadoc Syntax**: Supports all standard Javadoc tags and HTML elements
- **Custom Tag Support**: Handles custom Javadoc tags and extensions
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_javadoc::{Parser, JavadocLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
/**
 * Calculates the sum of two integers.
 * 
 * @param a The first integer
 * @param b The second integer
 * @return The sum of a and b
 * @throws IllegalArgumentException If either parameter is null
 * @since 1.0
 * @author John Doe
 */
public int add(int a, int b) {
    return a + b;
}
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed Javadoc comment successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Class Documentation
```rust
use oak_javadoc::{Parser, JavadocLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
/**
 * Represents a person with a name and age.
 * <p>
 * This class provides basic functionality for storing and retrieving
 * person information. It includes validation for age constraints.
 * 
 * @author Jane Smith
 * @version 1.2
 * @since 1.0
 */
public class Person {
    private String name;
    private int age;
    
    /**
     * Constructs a new Person with the specified name and age.
     * 
     * @param name The person's name, cannot be null
     * @param age The person's age, must be between 0 and 150
     * @throws IllegalArgumentException If name is null or age is out of range
     */
    public Person(String name, int age) {
        // Implementation...
    }
}
"#);

let result = parser.parse(&source);
println!("Parsed class documentation successfully.");
```

### Method Documentation with Examples
```rust
use oak_javadoc::{Parser, JavadocLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
/**
 * Formats a string using the specified template and arguments.
 * <p>
 * This method replaces placeholders in the template with the provided arguments.
 * Placeholders are specified in the format {0}, {1}, etc.
 * 
 * <pre>
 * String result = StringFormatter.format("Hello {0}, today is {1}", "Alice", "Monday");
 * // Returns: "Hello Alice, today is Monday"
 * </pre>
 * 
 * @param template The template string with placeholders
 * @param args The arguments to replace the placeholders
 * @return The formatted string
 * @throws IllegalArgumentException If template is null or number of arguments doesn't match placeholders
 * @see java.text.MessageFormat
 * @since 1.5
 */
public String format(String template, Object... args) {
    // Implementation...
}
"#);

let result = parser.parse(&source);
println!("Parsed method documentation with examples successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_javadoc::{Parser, JavadocLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("/** Simple comment */");
let result = parser.parse(&source);
// Token information is available in the parse result
```

### Error Handling
```rust
use oak_javadoc::{Parser, JavadocLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
/**
 * This comment has an unclosed tag
 * @param x This parameter is described
 * @return This return is not properly closed
public int broken() {
    return 0;
}
"#);

let result = parser.parse(&source);
if let Err(e) = result.result {
    println!("Parse error: {:?}", e);
}
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **Javadoc**: Root container for Javadoc comments
- **Description**: Main description text with HTML elements
- **BlockTag**: Block-level tags like @param, @return, @throws
- **InlineTag**: Inline tags like {@link}, {@code}, {@literal}
- **HtmlElement**: HTML elements within the documentation

## 📊 Performance

- **Streaming**: Parse large Javadoc files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak-javadoc integrates seamlessly with:

- **Documentation Generation**: Generating HTML docs from Javadoc comments
- **Static Analysis**: Analyzing code quality and documentation coverage
- **IDE Support**: Language server protocol compatibility for Java
- **API Extraction**: Extracting API specifications from source code
- **Code Completion**: Providing context-aware code completion

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete class documentation parsing
- Method and field documentation analysis
- Custom tag handling
- Integration with documentation generation tools

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-javadoc) or open [issues](https://github.com/ygg-lang/oaks/issues).