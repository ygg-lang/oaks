# Oak TypeScript Parser

[![Crates.io](https://img.shields.io/crates/v/oak-typescript.svg)](https://crates.io/crates/oak-typescript)
[![Documentation](https://docs.rs/oak-typescript/badge.svg)](https://docs.rs/oak-typescript)

High-performance incremental TypeScript parser for the oak ecosystem with flexible configuration, optimized for modern JavaScript development.

## 🎯 Overview

Oak TypeScript is a robust parser for TypeScript, designed to handle complete TypeScript syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for TypeScript analysis and tooling.

## ✨ Features

- **Complete TypeScript Syntax**: Supports all TypeScript features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_typescript::{Parser, TypeScriptLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
interface User {
    id: number;
    name: string;
    email?: string;
}

function greet(user: User): string {
    return `Hello, ${user.name}!`;
}
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed TypeScript code successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Interface Parsing
```rust
use oak_typescript::{Parser, TypeScriptLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
interface User {
    id: number;
    name: string;
    email?: string;
    roles: string[];
}
"#);

let result = parser.parse(&source);
println!("Parsed TypeScript interface successfully.");
```

### Function Parsing
```rust
use oak_typescript::{Parser, TypeScriptLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
function calculateTotal(items: Item[]): number {
    return items.reduce((sum, item) => sum + item.price, 0);
}
"#);

let result = parser.parse(&source);
println!("Parsed TypeScript function successfully.");
```

### Class Parsing
```rust
use oak_typescript::{Parser, TypeScriptLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
class UserService {
    private users: User[] = [];
    
    constructor(private apiClient: ApiClient) {}
    
    async getUser(id: number): Promise<User> {
        return this.apiClient.get(`/users/${id}`);
    }
}
"#);

let result = parser.parse(&source);
println!("Parsed TypeScript class successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_typescript::{Parser, TypeScriptLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("const x: number = 42;");
let result = parser.parse(&source);
// Token information is available in the parse result
```

### Error Handling
```rust
use oak_typescript::{Parser, TypeScriptLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
interface User {
    name: string
    age: number;
}
"#);

let result = parser.parse(&source);
if let Err(e) = result.result {
    println!("Parse error: {:?}", e);
}
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **SourceFile**: Root container for TypeScript source files
- **InterfaceDeclaration**: TypeScript interface definitions
- **ClassDeclaration**: TypeScript class definitions
- **FunctionDeclaration**: Function and method definitions
- **VariableStatement**: Variable declarations
- **Expression**: Various expression types
- **TypeAnnotation**: Type annotations and type references

## 📊 Performance

- **Streaming**: Parse large TypeScript files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak of typescript integrates seamlessly with:

- **Web Development**: Build TypeScript development tools and analyzers
- **IDE Support**: Language server protocol compatibility for TypeScript
- **Build Tools**: Integrate with bundlers and build pipelines
- **Code Analysis**: Analyze and understand TypeScript codebases
- **Documentation Tools**: Extract documentation from TypeScript source code

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete TypeScript source file parsing
- Interface and class analysis
- Type extraction and validation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-typescript) or open [issues](https://github.com/ygg-lang/oaks/issues).