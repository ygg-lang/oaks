# 📖 C Parser User Guide

C support for the Oak language framework.

This guide helps you integrate `oak-c` into your project and perform common parsing tasks efficiently.

## 🚀 Quick Start

### Basic Parsing Example

The following is a standard workflow for parsing a C function:

```rust
use oak_c::{CParser, language::CLanguage};
use oak_core::{SourceText, parser::Parser, source::TextEdit, parser::ParseSession};

fn main() {
    // 1. Prepare source code
    let code = r#"
        #include <stdio.h>
        
        int main() {
            printf("Hello, Oak!\n");
            return 0;
        }
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser
    let config = CLanguage::default();
    let parser = CParser::new(&config);
    let mut cache = ParseSession::default();

    // 3. Execute parsing
    let result = parser.parse(&source, &[], &mut cache);

    // 4. Handle results
    if result.diagnostics.is_empty() {
        println!("Parsing successful!");
    } else {
        eprintln!("Errors found during parsing.");
    }
}
```

## 🔍 Core Functionality

### 1. Syntax Tree Traversal
After a successful parse, use the built-in visitor pattern or manually traverse the Green/Red Tree to extract C-specific constructs like function definitions, struct members, or preprocessor directives.

### 2. Incremental Parsing
Optimize performance by only re-parsing changed sections:
```rust
use oak_c::{CParser, language::CLanguage};
use oak_core::{SourceText, parser::Parser, source::TextEdit, parser::ParseSession};

fn main() {
    // Initial parsing
    let code = r#"
        int main() {
            return 0;
        }
    "#;
    let source = SourceText::new(code);
    let config = CLanguage::default();
    let parser = CParser::new(&config);
    let mut cache = ParseSession::default();
    let old_result = parser.parse(&source, &[], &mut cache);

    // Updated code
    let new_code = r#"
        int main() {
            printf("Hello!");
            return 0;
        }
    "#;
    let new_source = SourceText::new(new_code);
    
    // Re-parsing (simplified example)
    let new_result = parser.parse(&new_source, &[], &mut cache);
}
```

### 3. Diagnostics & Error Recovery
`oak-c` provides detailed error contexts tailored for C developers:
```rust
use oak_c::{CParser, language::CLanguage};
use oak_core::{SourceText, parser::Parser, source::TextEdit, parser::ParseSession};

fn main() {
    let code = r#"
        int main() {
            return;
        }
    "#;
    let source = SourceText::new(code);
    let config = CLanguage::default();
    let parser = CParser::new(&config);
    let mut cache = ParseSession::default();
    let result = parser.parse(&source, &[], &mut cache);

    for error in &result.diagnostics {
        println!("Error: {}", error);
    }
}
```

## 🛠️ Performance & Reliability

- **High-Fidelity AST**: Retains all trivia (whitespace and comments), making it ideal for code formatting and refactoring tools.
- **Fault Tolerance**: Automatically recovers from syntax errors to provide as much information as possible from the rest of the file.
- **Memory Efficiency**: Leverages immutable data structures (Green Trees) for low-overhead tree management.
