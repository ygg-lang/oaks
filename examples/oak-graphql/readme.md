# Oak GraphQL Parser

[![Crates.io](https://img.shields.io/crates/v/oak-graphql.svg)](https://crates.io/crates/oak-graphql)
[![Documentation](https://docs.rs/oak-graphql/badge.svg)](https://docs.rs/oak-graphql)

High-performance incremental GraphQL parser for the oak ecosystem with flexible configuration, optimized for static analysis and code generation.

## 🎯 Overview

Oak GraphQL is a robust parser for GraphQL, designed to handle complete GraphQL syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for static analysis and code generation.

## ✨ Features

- **Complete GraphQL Syntax**: Supports all GraphQL features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_graphql::{Parser, GraphQLLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
query GetUser($id: ID!) {
    user(id: $id) {
        name
        email
        posts {
            title
            content
        }
    }
}
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed GraphQL successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Query Parsing
```rust
use oak_graphql::{Parser, GraphQLLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
query GetUsers($limit: Int, $offset: Int) {
    users(limit: $limit, offset: $offset) {
        id
        name
        email
        createdAt
    }
    totalUsers
}
"#);

let result = parser.parse(&source);
println!("Query parsed successfully.");
```

### Mutation Parsing
```rust
use oak_graphql::{Parser, GraphQLLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
mutation CreateUser($input: CreateUserInput!) {
    createUser(input: $input) {
        id
        name
        email
        createdAt
    }
}
"#);

let result = parser.parse(&source);
println!("Mutation parsed successfully.");
```

### Schema Parsing
```rust
use oak_graphql::{Parser, GraphQLLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
type User {
    id: ID!
    name: String!
    email: String!
    posts: [Post!]!
    createdAt: DateTime!
}

type Post {
    id: ID!
    title: String!
    content: String!
    author: User!
    createdAt: DateTime!
}

type Query {
    user(id: ID!): User
    users(limit: Int, offset: Int): [User!]!
    post(id: ID!): Post
    posts(limit: Int, offset: Int): [Post!]!
}
"#);

let result = parser.parse(&source);
println!("Schema parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_graphql::{Parser, GraphQLLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("query { user { name } }");
let result = parser.parse(&source);
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_graphql::{Parser, GraphQLLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
// Invalid GraphQL code example
query BrokenQuery {
    user {
        name
        email
        // Missing closing brace
    // Missing closing brace for query
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

- **GraphQLProgram**: Root container for GraphQL programs
- **Query**: GraphQL query definitions
- **Mutation**: GraphQL mutation definitions
- **Subscription**: GraphQL subscription definitions
- **Fragment**: GraphQL fragment definitions
- **Type**: GraphQL type definitions
- **Directive**: GraphQL directive definitions

## 📊 Performance

- **Streaming**: Parse large GraphQL files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak GraphQL integrates seamlessly with:

- **Static Analysis**: Code quality and security analysis
- **Code Generation**: Generating code from GraphQL AST
- **IDE Support**: Language server protocol compatibility
- **Refactoring**: Automated code refactoring
- **Documentation**: Generating documentation from GraphQL code

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete GraphQL program parsing
- Query, mutation, and subscription analysis
- Schema parsing and validation
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-graphql) or open [issues](https://github.com/ygg-lang/oaks/issues).