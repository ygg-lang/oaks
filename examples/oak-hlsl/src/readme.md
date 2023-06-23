# 🛠️ HLSL Parser Developer Guide

This guide is designed to help you quickly get started with developing and integrating `oak-hlsl`.

## 🚦 Quick Start

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
oak-hlsl = { path = "..." }
```

### Basic Parsing Example

The following is a standard workflow for parsing an HLSL shader:

```rust
use oak_hlsl::{Parser, SourceText};

fn main() {
    // 1. Prepare shader source code
    let code = r#"
        struct VS_INPUT {
            float4 Pos : POSITION;
            float2 Tex : TEXCOORD0;
        };

        struct VS_OUTPUT {
            float4 Pos : SV_POSITION;
            float2 Tex : TEXCOORD0;
        };

        float4x4 WorldViewProj;

        VS_OUTPUT main(VS_INPUT input) {
            VS_OUTPUT output;
            output.Pos = mul(input.Pos, WorldViewProj);
            output.Tex = input.Tex;
            return output;
        }
    "#;
    let source = SourceText::new(code);

    // 2. Initialize parser
    let parser = Parser::new();

    // 3. Execute parsing
    let result = parser.parse(&source);

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
After a successful parse, you can use the built-in visitor pattern or manually traverse the Green/Red Tree to extract HLSL constructs like shader entry points, constant buffer declarations, and resource bindings.

### 2. Incremental Parsing
Shader code often undergoes frequent small edits during development. `oak-hlsl` supports sub-millisecond incremental updates:
```rust
// Re-parse only the modified section
let new_result = parser.reparse(&new_source, &old_result);
```

### 3. Error Recovery
The parser is designed for industrial-grade fault tolerance, recovering gracefully from missing semicolons or malformed shader attributes to provide continuous feedback in IDEs.

## 🏗️ Architecture Overview

- **Lexer**: Tokenizes HLSL source text, supporting shader-specific keywords, semantics, and numeric literals with optional suffixes.
- **Parser**: A high-performance recursive descent parser with Pratt parsing for expressions, handling HLSL's operator precedence and complex vector/matrix math.
- **AST**: A strongly-typed, lossless syntax tree that preserves all trivia (comments/whitespace) for refactoring and formatting tools.

## 🔗 Advanced Resources

- **Full Examples**: Check the [examples/](examples/) folder in the project root.
- **API Documentation**: Run `cargo doc --open` for detailed type definitions.
- **Test Cases**: See [tests/readme.md](tests/readme.md) for details on our snapshot-based testing.
