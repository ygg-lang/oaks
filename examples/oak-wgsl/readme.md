# Oak WGSL Parser

[![Crates.io](https://img.shields.io/crates/v/oak-wgsl.svg)](https://crates.io/crates/oak-wgsl)
[![Documentation](https://docs.rs/oak-wgsl/badge.svg)](https://docs.rs/oak-wgsl)

High-performance incremental WGSL parser for the oak ecosystem with flexible configuration, optimized for WebGPU graphics programming and shader development.

## 🎯 Overview

Oak WGSL is a robust parser for WebGPU Shading Language (WGSL), designed to handle complete WGSL syntax including modern shader features and compute capabilities. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for WGSL analysis and tooling.

## ✨ Features

- **Complete WGSL Syntax**: Supports all WGSL features including modern specifications
- **Shader Stage Support**: Handles vertex, fragment, and compute shader stages
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_wgsl::{Parser, WgslLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    let x = f32(i32(in_vertex_index) - 1);
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
    return vec4<f32>(x, y, 0.0, 1.0);
}
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed WGSL shader successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Fragment Shader Parsing
```rust
use oak_wgsl::{Parser, WgslLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
@fragment
fn fs_main(@location(0) frag_color: vec4<f32>) -> @location(0) vec4<f32> {
    return frag_color;
}
"#);

let result = parser.parse(&source);
println!("Parsed WGSL fragment shader successfully.");
```

### Compute Shader Parsing
```rust
use oak_wgsl::{Parser, WgslLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
@group(0) @binding(0)
var<storage, read_write> data: array<f32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    if (index >= arrayLength(&data)) {
        return;
    }
    data[index] = data[index] * 2.0;
}
"#);

let result = parser.parse(&source);
println!("Parsed WGSL compute shader successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_wgsl::{Parser, WgslLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("@vertex fn main() -> @builtin(position) vec4<f32> { return vec4<f32>(0.0); }");
let result = parser.parse(&source);
// Token information is available in the parse result
```

### Error Handling
```rust
use oak_wgsl::{Parser, WgslLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
@vertex
fn broken_shader() -> vec4<f32> {
    let x: f32 = "not a number"; // Type mismatch
    return x; // Missing vector construction
}
"#);

let result = parser.parse(&source);
if let Err(e) = result.result {
    println!("Parse error: {:?}", e);
}
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **Shader**: Root container for WGSL shaders
- **Function**: Shader functions with attributes
- **Variable**: Variable declarations with bindings
- **Statements**: Assignment, if, loop, return statements
- **Expressions**: Binary, unary, function call expressions
- **Types**: Vector, matrix, array, and custom types

## 📊 Performance

- **Streaming**: Parse large WGSL files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak-wgsl integrates seamlessly with:

- **Static Analysis**: Code quality and security analysis
- **Shader Transpilation**: Converting between shading languages
- **IDE Support**: Language server protocol compatibility
- **Shader Validation**: Checking shader correctness before compilation
- **Documentation**: Generating documentation from WGSL code

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete WGSL shader parsing
- Vertex and fragment shader analysis
- Compute shader processing
- Integration with graphics pipelines

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-wgsl) or open [issues](https://github.com/ygg-lang/oaks/issues).