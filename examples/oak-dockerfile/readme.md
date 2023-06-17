# Oak Dockerfile Parser

## Overview

`Oak of docker` is a powerful and efficient parser for Dockerfiles, built using the `oak` parser combinator library. It provides a robust solution for parsing Dockerfile syntax, enabling various applications such as static analysis of Docker images, security auditing, and automated Dockerfile generation.

## Features

- **Comprehensive Dockerfile Grammar**: Supports all standard Dockerfile instructions and syntax.
- **High Performance**: Leverages `oak`'s optimized parsing techniques for speed.
- **Abstract Syntax Tree (AST)**: Generates a detailed and easy-to-navigate AST representing the Dockerfile structure.
- **Error Handling**: Provides meaningful error messages for better debugging of malformed Dockerfiles.
- **Extensible**: Easily extendable to support custom Dockerfile directives or extensions.

## Quick Start

To use `Oak of docker` in your Rust project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
Oak of docker = "0.1.0" # Replace with the latest version
oak = "0.1.0" # Replace with the latest version
```

## Parsing Examples

Here's a simple example demonstrating how to parse a Dockerfile content:

```rust
use pex_docker::dockerfile_parser;

fn main() {
    let input = r#"
FROM alpine:latest
RUN apk add --no-cache bash
COPY . /app
WORKDIR /app
CMD ["bash"]
"#;
    match dockerfile_parser::parse(input) {
        Ok(ast) => {
            println!("Successfully parsed Dockerfile:\n{:#?}", ast);
        }
        Err(err) => {
            eprintln!("Failed to parse Dockerfile: {}", err);
        }
    }
}
```

## Advanced Features

### Customizing the Parser

The `oak` library allows for flexible customization of the parser. You can modify the grammar rules or add new ones to suit your specific needs, such as supporting experimental Dockerfile features. Refer to the `oak` documentation for more details on parser customization.

### Error Recovery

`Oak of docker` can be extended with error recovery mechanisms to handle malformed Dockerfiles gracefully, allowing for partial parsing and better resilience in real-world scenarios.

## AST Structure

The generated AST for Dockerfiles provides a hierarchical representation of the instructions. For instance, a `FROM` instruction might result in an AST structure similar to this:

```rust
// Simplified AST representation for:
// FROM alpine:latest
pex_docker::ast::Node::Instruction {
    command: "FROM".to_string(),
    arguments: vec![
        "alpine:latest".to_string(),
    ],
}
```

## Performance

`Oak of docker` is designed for performance. Benchmarks show efficient parsing of large Dockerfiles. Optimizations include memoization, efficient backtracking, and direct AST construction.

## Integration

`Oak of docker` can be integrated into various tools and applications:

- **Dockerfile Linters**: Analyze Dockerfiles for best practices and potential issues.
- **Security Scanners**: Identify vulnerabilities in Docker image builds.
- **CI/CD Pipelines**: Automate Dockerfile validation and generation.

## Examples

Explore the `examples` directory within the `oak-docker` project for more usage examples and demonstrations of specific Dockerfile parsing features.

## Contributing

Contributions to `Oak of docker` are welcome! If you find a bug or have a feature request, please open an issue on the GitHub repository. For major changes, please open a discussion first.