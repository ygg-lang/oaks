# Oak GLSL Parser

[![Crates.io](https://img.shields.io/crates/v/oak-gsgl.svg)](https://crates.io/crates/oak-gsgl)
[![Documentation](https://docs.rs/oak-gsgl/badge.svg)](https://docs.rs/oak-gsgl)

A high-performance GLSL (OpenGL Shading Language) parser for Rust, built with the Oak parser combinator framework. Parse shader programs with comprehensive AST generation and error handling.

## Overview

Oak GLSL provides robust parsing capabilities for GLSL shader files, supporting vertex, fragment, geometry, and compute shaders. Built on the Oak parser combinator framework, it delivers excellent performance and detailed error messages.

## Features

- ✅ **Complete GLSL Support**: Parse vertex, fragment, geometry, and compute shaders
- ✅ **Modern Rust API**: Type-safe parsing with comprehensive error handling
- ✅ **High Performance**: Built on the efficient Oak parser combinator framework
- ✅ **Rich AST**: Detailed Abstract Syntax Tree with source location tracking
- ✅ **Extensible**: Easy to extend for custom shader dialects
- ✅ **Well Tested**: Comprehensive test suite with real-world examples

## Quick Start

Add Oak GLSL to your `Cargo.toml`:

## 📋 Parsing Examples

### Basic Vertex Shader Parsing

```rust
use oak::{Parser, Language};
use oak_gsgl::GLSLLanguage;

fn main() {
    let source = r#"
        #version 330 core
        
        layout(location = 0) in vec3 aPos;
        layout(location = 1) in vec2 aTexCoord;
        
        out vec2 TexCoord;
        
        uniform mat4 model;
        uniform mat4 view;
        uniform mat4 projection;
        
        void main() {
            gl_Position = projection * view * model * vec4(aPos, 1.0);
            TexCoord = aTexCoord;
        }
    "#;
    
    let mut parser = Parser::<GLSLLanguage>::new();
    match parser.parse(&source) {
        Ok(ast) => {
            println!("Parsed AST: {:#?}", ast);
        }
        Err(error) => {
            eprintln!("Parse error: {}", error);
        }
    }
}
```

### Advanced Fragment Shader with Functions

```rust
use oak::{Parser, Language};
use oak_gsgl::GLSLLanguage;

fn main() {
    let source = r#"
        #version 450 core
        
        in vec2 TexCoord;
        in vec3 Normal;
        in vec3 FragPos;
        
        out vec4 FragColor;
        
        struct Material {
            vec3 ambient;
            vec3 diffuse;
            vec3 specular;
            float shininess;
        };
        
        struct Light {
            vec3 position;
            vec3 ambient;
            vec3 diffuse;
            vec3 specular;
        };
        
        uniform Material material;
        uniform Light light;
        uniform vec3 viewPos;
        uniform sampler2D texture1;
        
        vec3 calculatePhong(vec3 normal, vec3 fragPos, vec3 viewDir) {
            vec3 lightDir = normalize(light.position - fragPos);
            
            // Ambient
            vec3 ambient = light.ambient * material.ambient;
            
            // Diffuse
            float diff = max(dot(normal, lightDir), 0.0);
            vec3 diffuse = light.diffuse * (diff * material.diffuse);
            
            // Specular
            vec3 reflectDir = reflect(-lightDir, normal);
            float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
            vec3 specular = light.specular * (spec * material.specular);
            
            return ambient + diffuse + specular;
        }
        
        void main() {
            vec3 norm = normalize(Normal);
            vec3 viewDir = normalize(viewPos - FragPos);
            
            vec3 result = calculatePhong(norm, FragPos, viewDir);
            vec3 texColor = texture(texture1, TexCoord).rgb;
            
            FragColor = vec4(result * texColor, 1.0);
        }
    "#;
    
    let mut parser = Parser::<GLSLLanguage>::new();
    match parser.parse(&source) {
        Ok(ast) => {
            println!("Advanced shader parsed successfully!");
        }
        Err(error) => {
            eprintln!("Parse error: {}", error);
        }
    }
}
```

## Advanced Features

### Compute Shader Parsing

Oak GLSL supports parsing compute shaders:

```rust
let source = r#"
    #version 430
    
    layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;
    
    layout(std430, binding = 0) buffer InputBuffer {
        float data[];
    } inputBuffer;
    
    layout(std430, binding = 1) buffer OutputBuffer {
        float result[];
    } outputBuffer;
    
    uniform float multiplier;
    
    void main() {
        uint index = gl_GlobalInvocationID.x;
        if (index < inputBuffer.data.length()) {
            outputBuffer.result[index] = inputBuffer.data[index] * multiplier;
        }
    }
"#;
```

### Geometry Shader Support

Parse geometry shaders with input/output layout qualifiers:

```rust
let source = r#"
    #version 330 core
    
    layout(points) in;
    layout(line_strip, max_vertices = 2) out;
    
    uniform mat4 projection;
    uniform float time;
    
    void main() {
        vec4 pos = gl_in[0].gl_Position;
        
        gl_Position = projection * (pos + vec4(-0.1, 0.0, 0.0, 0.0));
        EmitVertex();
        
        gl_Position = projection * (pos + vec4(0.1, 0.0, 0.0, 0.0));
        EmitVertex();
        
        EndPrimitive();
    }
"#;
```

### Preprocessor Directives

Handle GLSL preprocessor directives:

```rust
let source = r#"
    #version 450 core
    
    #define MAX_LIGHTS 16
    #define PI 3.14159265359
    
    #ifdef USE_NORMAL_MAPPING
        #extension GL_OES_standard_derivatives : enable
    #endif
    
    #if defined(USE_SHADOWS) && defined(USE_PCF)
        #define SHADOW_FILTER_SIZE 3
    #endif
    
    // Shader code continues...
"#;
```

## AST Structure

The parser generates a rich AST with the following main node types:

- `GLSLFile` - Root node containing the entire shader
- `VersionDirective` - GLSL version specification
- `ExtensionDirective` - Extension directives
- `PreprocessorDirective` - Preprocessor commands
- `FunctionDefinition` - Function definitions
- `StructDefinition` - Struct type definitions
- `VariableDeclaration` - Variable declarations
- `TypeQualifier` - Type qualifiers (in, out, uniform, etc.)
- `Expression` - Mathematical and logical expressions
- `Statement` - Control flow statements

## Performance

Oak GLSL is designed for high performance:

- **Zero-copy parsing** where possible
- **Streaming support** for large shader files
- **Efficient memory usage** with minimal allocations
- **Fast error recovery** for better developer experience

## Integration

Oak GLSL integrates seamlessly with the Oak ecosystem:

```rust
use oak::{Parser, Language};
use oak_gsgl::GLSLLanguage;

// Use with other Oak parsers
let mut parser = Parser::<GLSLLanguage>::new();
let result = parser.parse(glsl_source);
```

## Examples

More examples can be found in the [examples directory](https://github.com/axodotdev/oak/tree/main/examples/oak-gsgl/examples):

- [Basic vertex shader](examples/vertex_shader.rs)
- [Fragment shader with lighting](examples/fragment_shader.rs)
- [Compute shader](examples/compute_shader.rs)
- [Geometry shader](examples/geometry_shader.rs)
- [Preprocessor directives](examples/preprocessor.rs)

## Contributing

We welcome contributions! Please see our [Contributing Guide](https://github.com/axodotdev/oak/blob/main/CONTRIBUTING.md) for details.