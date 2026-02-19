# 🛠️ Kotlin Parser Developer Guide

Kotlin support for the Oak language framework.

This guide is designed to help you quickly get started with developing and integrating `oak-kotlin`.

## 🚦 Quick Start

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
oak-kotlin = { path = "..." }
```

### Basic Parsing Example

The following is a standard workflow for parsing modern Kotlin with data classes, coroutines, and lambdas:

```rust
use oak_kotlin::{KotlinParser, SourceText, Kotlin};
use oak_core::parser::DefaultParseCache;

fn main() {
    // 1. Prepare source code
    let code = r#"
        package com.example.oak

        import kotlinx.coroutines.*

        data class User(val id: String, val name: String)

        class Repository {
            private val users = mutableListOf<User>()

            suspend fun addUser(user: User) = withContext(Dispatchers.IO) {
                users.add(user)
                println("User ${user.name} added!")
            }
        }
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser
    let parser = KotlinParser;
    let mut cache = DefaultParseCache::default();

    // 3. Execute parsing
    let result = parser.parse(&source, &[], &mut cache);

    // 4. Handle results
    if result.is_success() {
        println!("Parsing successful! AST node count: {}", result.node_count());
    } else {
        eprintln!("Errors found during parsing.");
        for diag in result.diagnostics() {
            println!("[{}:{}] {}", diag.line, diag.column, diag.message);
        }
    }
}
```

## 🔍 Core API Usage

### 1. Syntax Tree Traversal
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract Kotlin specific constructs like data classes, sealed classes, suspend functions, lambdas, and property delegates.

### 2. Incremental Parsing
Kotlin projects (especially Android or KMP projects) can be massive. `oak-kotlin` supports sub-millisecond incremental updates:
```rust
// Re-parse only the modified section
let new_result = parser.parse(&new_source, &edits, &mut cache);
```

### 3. KMP and Multiplatform Support
The parser correctly handles Kotlin Multiplatform (KMP) specific syntax like `expect`/`actual` declarations, ensuring accurate trees across all targets.

## 📊 Code Analysis Examples

### 1. Analyzing Kotlin Data Classes

```rust
use oak_kotlin::{KotlinParser, SourceText, Kotlin};
use oak_core::parser::DefaultParseCache;

fn analyze_data_classes() {
    let code = r#"
        data class Person(
            val id: String,
            val name: String,
            val age: Int,
            val email: String?
        ) {
            fun fullName() = name
        }
    "#;
    
    let source = SourceText::new(code);
    let parser = KotlinParser;
    let mut cache = DefaultParseCache::default();
    let result = parser.parse(&source, &[], &mut cache);
    
    if result.is_success() {
        println!("Data class analysis:");
        println!("- Parsed successfully with {} nodes", result.node_count());
        // Here you would extract data class properties and methods
    }
}
```

### 2. Analyzing Coroutine Functions

```rust
use oak_kotlin::{KotlinParser, SourceText, Kotlin};
use oak_core::parser::DefaultParseCache;

fn analyze_coroutines() {
    let code = r#"
        class ApiService {
            private val client = HttpClient()
            
            suspend fun fetchData(url: String): Result<Data, ApiError> = withContext(Dispatchers.IO) {
                try {
                    val response = client.get(url)
                    val data = response.body<Data>()
                    Result.success(data)
                } catch (e: Exception) {
                    Result.failure(ApiError(e.message))
                }
            }
        }
    "#;
    
    let source = SourceText::new(code);
    let parser = KotlinParser;
    let mut cache = DefaultParseCache::default();
    let result = parser.parse(&source, &[], &mut cache);
    
    if result.is_success() {
        println!("Coroutine analysis:");
        println!("- Suspend function detected");
        // Here you would extract coroutine-specific information
    }
}
```

### 3. Analyzing Extension Functions

```rust
use oak_kotlin::{KotlinParser, SourceText, Kotlin};
use oak_core::parser::DefaultParseCache;

fn analyze_extension_functions() {
    let code = r#"
        fun String.capitalizeWords(): String {
            return split(" ")
                .map { it.capitalize() }
                .joinToString(" ")
        }
        
        fun List<Int>.sumEven(): Int {
            return filter { it % 2 == 0 }
                .sum()
        }
    "#;
    
    let source = SourceText::new(code);
    let parser = KotlinParser;
    let mut cache = DefaultParseCache::default();
    let result = parser.parse(&source, &[], &mut cache);
    
    if result.is_success() {
        println!("Extension function analysis:");
        println!("- String extension: capitalizeWords");
        println!("- List<Int> extension: sumEven");
        // Here you would extract extension function details
    }
}
```

### 4. Analyzing Sealed Classes and When Expressions

```rust
use oak_kotlin::{KotlinParser, SourceText, Kotlin};
use oak_core::parser::DefaultParseCache;

fn analyze_sealed_classes() {
    let code = r#"
        sealed class Result<T> {
            data class Success<T>(val data: T) : Result<T>()
            data class Error<T>(val message: String) : Result<T>()
        }
        
        fun <T> handleResult(result: Result<T>) {
            when (result) {
                is Result.Success -> println("Success: ${result.data}")
                is Result.Error -> println("Error: ${result.message}")
            }
        }
    "#;
    
    let source = SourceText::new(code);
    let parser = KotlinParser;
    let mut cache = DefaultParseCache::default();
    let result = parser.parse(&source, &[], &mut cache);
    
    if result.is_success() {
        println!("Sealed class analysis:");
        println!("- Sealed class Result with 2 subclasses");
        println!("- When expression with pattern matching");
        // Here you would extract sealed class hierarchy
    }
}
```

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes Kotlin source text, supporting complex string templates, backticked identifiers, and various numeric literal formats.
- **Parser**: A high-performance syntax analyzer that handles Kotlin's expressive grammar, including generics, coroutines, and modern language features.
- **AST**: A strongly-typed, lossless syntax tree that preserves all trivia (comments/whitespace) for refactoring and formatting tools.

## 🛠️ Performance & Reliability

- **High-Fidelity AST**: Retains all trivia (whitespace and comments), making it ideal for code formatting and refactoring tools.
- **Fault Tolerance**: Automatically recovers from syntax errors to provide as much information as possible from the rest of the file.
- **Memory Efficiency**: Leverages immutable data structures (Green Trees) for low-overhead tree management.
- **Incremental Updates**: Sub-millisecond parsing for large files with small changes, perfect for IDE integration.

## 📚 API Reference

### Core Components

- **KotlinParser**: The main parser implementation for Kotlin source code
- **Kotlin**: Language configuration and settings
- **KotlinLexer**: Tokenizer for Kotlin source text
- **KotlinToken**: Token types for Kotlin syntax
- **KotlinElement**: Element types for Kotlin AST nodes

### Common Use Cases

1. **IDE Integration**: Real-time syntax highlighting and error checking
2. **Static Analysis**: Code quality tools and linters
3. **Refactoring Tools**: Automated code transformations
4. **Documentation Generation**: Extracting code structure for documentation
5. **KMP Support**: Analyzing multiplatform codebases

## 🔧 Advanced Configuration

### Customizing Parser Behavior

```rust
use oak_kotlin::{KotlinParser, Kotlin};

fn configure_parser() {
    let language = Kotlin;
    let parser = KotlinParser;
    
    println!("Parser configured for Kotlin language");
    println!("Language name: {}", Kotlin::NAME);
}
```

## 🎯 Kotlin-Specific Features

The parser supports all major Kotlin features:

- **Data Classes**: Automatic generation of equals(), hashCode(), toString(), etc.
- **Sealed Classes**: Restricted class hierarchies for pattern matching
- **Coroutines**: Asynchronous programming with suspend functions
- **Extension Functions**: Adding functionality to existing types
- **Null Safety**: Safe handling of nullable types
- **Smart Casts**: Automatic type casting based on type checks
- **Lambda Expressions**: Concise function literals
- **Property Delegates**: Custom property behavior
- **Type Inference**: Automatic type deduction
- **Operator Overloading**: Custom operator implementations
