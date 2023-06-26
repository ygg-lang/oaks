# 📖 Java Parser User Guide

Java support for the Oak language framework.

This guide helps you integrate `oak-java` into your project and perform common parsing tasks efficiently.

## 🚀 Quick Start

### Basic Parsing Example

The following is a standard workflow for parsing a Java class with modern features like Records and Annotations:

```rust
use oak_java::{JavaParser, SourceText, JavaLanguage};

fn main() {
    // 1. Prepare source code
    let code = r#"
        package com.example;
        
        import java.util.List;
        
        /**
         * Represents a user in the system.
         */
        @Entity
        public record User(String name, int age) {
            public void greet() {
                System.out.println("Hello, " + name);
            }
        }
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser
    let config = JavaLanguage::new();
    let parser = JavaParser::new(&config);

    // 3. Execute parsing
    let result = parser.parse(&source);

    // 4. Handle results
    if result.is_success() {
        println!("Parsing successful! AST node count: {}", result.node_count());
    } else {
        eprintln!("Errors found during parsing.");
    }
}
```

## 🔍 Core Functionality

### 1. Syntax Tree Traversal
After a successful parse, use the built-in visitor pattern or manually traverse the Green/Red Tree to extract Java-specific constructs like class/record definitions, annotations, or complex method bodies.

### 2. Incremental Parsing
Optimize performance by only re-parsing changed sections:
```rust
// Assuming you have an old parse result 'old_result' and new source text 'new_source'
let new_result = parser.reparse(&new_source, &old_result);
```

### 3. Diagnostics & Error Recovery
`oak-java` provides detailed error contexts tailored for Java developers:
```rust
for diag in result.diagnostics() {
    println!("[{}:{}] {}", diag.line, diag.column, diag.message);
}
```

## 📊 Code Analysis Examples

### 1. Extracting Class Information

```rust
use oak_java::{JavaParser, SourceText, JavaLanguage};

fn analyze_class_structure() {
    let code = r#"
        public class Calculator {
            private int result;
            
            public int add(int a, int b) {
                result = a + b;
                return result;
            }
            
            public int subtract(int a, int b) {
                result = a - b;
                return result;
            }
        }
    "#;
    
    let source = SourceText::new(code);
    let config = JavaLanguage::new();
    let parser = JavaParser::new(&config);
    let result = parser.parse(&source);
    
    if result.is_success() {
        println!("Class structure analysis:");
        println!("- File contains {} nodes", result.node_count());
        // Here you would implement AST traversal to extract class details
    }
}
```

### 2. Analyzing Method Signatures

```rust
use oak_java::{JavaParser, SourceText, JavaLanguage};

fn analyze_method_signatures() {
    let code = r#"
        public interface Repository {
            User findById(long id);
            List<User> findAll();
            void save(User user);
            void delete(long id);
        }
    "#;
    
    let source = SourceText::new(code);
    let config = JavaLanguage::new();
    let parser = JavaParser::new(&config);
    let result = parser.parse(&source);
    
    if result.is_success() {
        println!("Method signature analysis:");
        println!("- Interface parsed successfully");
        // Here you would extract and analyze method signatures
    }
}
```

### 3. Detecting Code Smells

```rust
use oak_java::{JavaParser, SourceText, JavaLanguage};

fn detect_code_smells() {
    let code = r#"
        public class DataProcessor {
            public void processData(List<String> data) {
                for (int i = 0; i < data.size(); i++) {
                    String item = data.get(i);
                    System.out.println(item);
                }
            }
        }
    "#;
    
    let source = SourceText::new(code);
    let config = JavaLanguage::new();
    let parser = JavaParser::new(&config);
    let result = parser.parse(&source);
    
    if result.is_success() {
        println!("Code smell detection:");
        println!("- Potential issues: traditional for loop could be replaced with for-each");
        // Here you would implement more sophisticated code smell detection
    }
}
```

## 🛠️ Performance & Reliability

- **High-Fidelity AST**: Retains all trivia (whitespace and comments), making it ideal for code formatting and refactoring tools.
- **Fault Tolerance**: Automatically recovers from syntax errors to provide as much information as possible from the rest of the file.
- **Memory Efficiency**: Leverages immutable data structures (Green Trees) for low-overhead tree management.
- **Incremental Updates**: Sub-millisecond parsing for large files with small changes, perfect for IDE integration.

## 📚 API Reference

### Core Components

- **JavaParser**: The main parser implementation for Java source code
- **JavaLanguage**: Language configuration and settings
- **JavaLexer**: Tokenizer for Java source text
- **JavaBuilder**: AST builder for constructing Java-specific syntax trees

### Common Use Cases

1. **IDE Integration**: Real-time syntax highlighting and error checking
2. **Static Analysis**: Code quality tools and linters
3. **Refactoring Tools**: Automated code transformations
4. **Documentation Generation**: Extracting code structure for documentation
5. **Code Migration**: Assisting in codebase migrations between Java versions

## 🔧 Advanced Configuration

### Customizing Parser Behavior

```rust
use oak_java::{JavaParser, JavaLanguage};

fn configure_parser() {
    let mut config = JavaLanguage::new();
    // Here you would set configuration options if available
    let parser = JavaParser::new(&config);
    
    println!("Parser configured with custom settings");
}
```


