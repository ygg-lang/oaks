# Oak Dockerfile Parser

[![Crates.io](https://img.shields.io/crates/v/oak-dockerfile.svg)](https://crates.io/crates/oak-dockerfile)
[![Documentation](https://docs.rs/oak-dockerfile/badge.svg)](https://docs.rs/oak-dockerfile)

High-performance incremental Dockerfile parser for the oak ecosystem with flexible configuration, optimized for container configuration and image building.

## 🎯 Overview

Oak Dockerfile is a robust parser for Dockerfile, designed to handle complete Dockerfile syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for container configuration and image building.

## ✨ Features

- **Complete Dockerfile Syntax**: Supports all Dockerfile features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_dockerfile::{Parser, DockerfileLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
FROM alpine:latest
RUN apk add --no-cache bash
COPY . /app
WORKDIR /app
CMD ["bash"]
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed Dockerfile successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Basic Dockerfile Parsing
```rust
use oak_dockerfile::{Parser, DockerfileLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
FROM node:14-alpine
WORKDIR /app
COPY package*.json ./
RUN npm ci --only=production
COPY . .
EXPOSE 3000
CMD ["npm", "start"]
"#);

let result = parser.parse(&source);
println!("Dockerfile parsed successfully.");
```

### Multi-stage Build Parsing
```rust
use oak_dockerfile::{Parser, DockerfileLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
# Build stage
FROM golang:1.19-alpine AS builder
WORKDIR /src
COPY go.mod go.sum ./
RUN go mod download
COPY . .
RUN CGO_ENABLED=0 go build -o /app

# Runtime stage
FROM alpine:latest
RUN apk --no-cache add ca-certificates
WORKDIR /root/
COPY --from=builder /app .
EXPOSE 8080
CMD ["./app"]
"#);

let result = parser.parse(&source);
println!("Multi-stage Dockerfile parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_dockerfile::{Parser, DockerfileLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("FROM alpine:latest");
let result = parser.parse(&source);
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_dockerfile::{Parser, DockerfileLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
FROM alpine:latest
RUN apk add --no-cache bash
COPY . /app
WORKDIR /app
CMD ["bash"
# Missing closing bracket
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

- **Dockerfile**: Root container for Dockerfile documents
- **Instruction**: Dockerfile instructions (FROM, RUN, COPY, etc.)
- **Argument**: Instruction arguments and parameters
- **Stage**: Build stages in multi-stage builds
- **Comment**: Dockerfile comments

## 📊 Performance

- **Streaming**: Parse large Dockerfiles without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak Dockerfile integrates seamlessly with:

- **Container Analysis**: Analyze Dockerfiles for security and best practices
- **CI/CD Pipelines**: Integrate with build and deployment workflows
- **IDE Support**: Language server protocol compatibility
- **Security Scanning**: Identify potential security issues in Dockerfiles
- **Optimization**: Suggest optimizations for Dockerfile structure

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete Dockerfile parsing
- Multi-stage build analysis
- Security vulnerability detection
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-dockerfile) or open [issues](https://github.com/ygg-lang/oaks/issues).