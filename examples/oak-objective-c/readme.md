# Oak Objective-C Parser

[![Crates.io](https://img.shields.io/crates/v/oak-objective-c.svg)](https://crates.io/crates/oak-objective-c)
[![Documentation](https://docs.rs/oak-objective-c/badge.svg)](https://docs.rs/oak-objective-c)

High-performance incremental Objective-C parser for the oak ecosystem with flexible configuration, optimized for static analysis and code generation.

## 🎯 Overview

Oak Objective-C is a robust parser for Objective-C, designed to handle complete Objective-C syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for static analysis and code generation.

## ✨ Features

- **Complete Objective-C Syntax**: Supports all Objective-C features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_objective_c::{Parser, ObjectiveCLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
#import <Foundation/Foundation.h>

@interface Person : NSObject {
    NSString *name;
    NSInteger age;
}

- (instancetype)initWithName:(NSString *)name age:(NSInteger)age;
- (void)greet;

@end

@implementation Person

- (instancetype)initWithName:(NSString *)name age:(NSInteger)age {
    self = [super init];
    if (self) {
        self.name = name;
        self.age = age;
    }
    return self;
}

- (void)greet {
    NSLog(@"Hello, I'm %@ and I'm %ld years old", self.name, (long)self.age);
}

@end

int main(int argc, const char * argv[]) {
    @autoreleasepool {
        Person *person = [[Person alloc] initWithName:@"John" age:30];
        [person greet];
    }
    return 0;
}
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed Objective-C successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Interface Parsing
```rust
use oak_objective_c::{Parser, ObjectiveCLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
@interface Calculator : NSObject

- (NSInteger)add:(NSInteger)a to:(NSInteger)b;
- (NSInteger)subtract:(NSInteger)a from:(NSInteger)b;
- (NSInteger)multiply:(NSInteger)a by:(NSInteger)b;

@end
"#);

let result = parser.parse(&source);
println!("Interface parsed successfully.");
```

### Implementation Parsing
```rust
use oak_objective_c::{Parser, ObjectiveCLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
@implementation Calculator

- (NSInteger)add:(NSInteger)a to:(NSInteger)b {
    return a + b;
}

- (NSInteger)subtract:(NSInteger)a from:(NSInteger)b {
    return b - a;
}

- (NSInteger)multiply:(NSInteger)a by:(NSInteger)b {
    return a * b;
}

@end
"#);

let result = parser.parse(&source);
println!("Implementation parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_objective_c::{Parser, ObjectiveCLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("NSString *message = @\"Hello World\";");
let result = parser.parse(&source);
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_objective_c::{Parser, ObjectiveCLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
// Invalid Objective-C code example
@interface BrokenClass : NSObject {
    NSString *name
    // Missing semicolon
}

- (void)brokenMethod {
    NSLog(@"Hello")
    // Missing semicolon
}

@end
"#);

let result = parser.parse(&source);
if let Some(errors) = result.result.err() {
    println!("Parse errors found: {:?}", errors);
} else {
    println!("Parsed successfully.");
}
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **ObjectiveCProgram**: Root container for Objective-C programs
- **Interface**: Objective-C interface definitions
- **Implementation**: Objective-C implementation definitions
- **Method**: Objective-C methods
- **Property**: Objective-C properties
- **Statement**: Various statement types including control flow
- **Expression**: Various expression types including operators

## 📊 Performance

- **Streaming**: Parse large Objective-C files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak Objective-C integrates seamlessly with:

- **Static Analysis**: Code quality and security analysis
- **Code Generation**: Generating code from Objective-C AST
- **IDE Support**: Language server protocol compatibility
- **Refactoring**: Automated code refactoring
- **Documentation**: Generating documentation from Objective-C code

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete Objective-C program parsing
- Interface and implementation analysis
- Code transformation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-objective-c) or open [issues](https://github.com/ygg-lang/oaks/issues).