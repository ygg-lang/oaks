# Oak HLSL Parser

[![Crates.io](https://img.shields.io/crates/v/oak-hlsl.svg)](https://crates.io/crates/oak-hlsl)
[![Documentation](https://docs.rs/oak-hlsl/badge.svg)](https://docs.rs/oak-hlsl)

A high-performance High Level Shading Language (HLSL) parser for Rust, built with the Oak parser combinator framework. Parse DirectX shader code with comprehensive AST generation and error handling.

## Overview

Oak HLSL provides robust parsing capabilities for High Level Shading Language files, supporting vertex shaders, pixel shaders, compute shaders, and all major HLSL constructs. Built on the Oak parser combinator framework, it delivers excellent performance and detailed error messages.

## Features

- ✅ **Complete HLSL Support**: Parse shaders, functions, structs, constants, and semantics
- ✅ **Modern Rust API**: Type-safe parsing with comprehensive error handling
- ✅ **High Performance**: Built on the efficient Oak parser combinator framework
- ✅ **Rich AST**: Detailed Abstract Syntax Tree with source location tracking
- ✅ **Extensible**: Easy to extend for custom HLSL dialects
- ✅ **Well Tested**: Comprehensive test suite with real-world examples

## Quick Start

## Parsing Examples

### Basic Vertex Shader Parsing

```rust
use oak::{Parser, Language};
use oak_hlsl::HLSLLanguage;

fn main() {
    let source = r#"
        struct VS_INPUT {
            float4 position : POSITION;
            float2 texcoord : TEXCOORD0;
            float3 normal : NORMAL;
        };
        
        struct VS_OUTPUT {
            float4 position : SV_POSITION;
            float2 texcoord : TEXCOORD0;
            float3 normal : NORMAL;
        };
        
        VS_OUTPUT main(VS_INPUT input) {
            VS_OUTPUT output;
            output.position = mul(float4(input.position.xyz, 1.0), WorldViewProj);
            output.texcoord = input.texcoord;
            output.normal = mul(input.normal, (float3x3)WorldInverseTranspose);
            return output;
        }
    "#;
    
    let mut parser = Parser::<HLSLLanguage>::new();
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

### Complex Pixel Shader with Textures

```rust
use oak::{Parser, Language};
use oak_hlsl::HLSLLanguage;

fn main() {
    let source = r#"
        Texture2D gDiffuseTexture : register(t0);
        Texture2D gNormalTexture : register(t1);
        SamplerState gDiffuseSampler : register(s0);
        
        cbuffer PerFrameCB : register(b0) {
            float4 gLightDirection;
            float4 gLightColor;
            float4 gAmbientColor;
        };
        
        struct PS_INPUT {
            float4 position : SV_POSITION;
            float2 texcoord : TEXCOORD0;
            float3 normal : NORMAL;
            float3 tangent : TANGENT;
            float3 bitangent : BITANGENT;
        };
        
        float4 main(PS_INPUT input) : SV_TARGET {
            float4 diffuseColor = gDiffuseTexture.Sample(gDiffuseSampler, input.texcoord);
            float3 normalMap = gNormalTexture.Sample(gDiffuseSampler, input.texcoord).xyz * 2.0 - 1.0;
            
            float3 N = normalize(input.normal);
            float3 L = normalize(gLightDirection.xyz);
            float NdotL = max(dot(N, L), 0.0);
            
            float3 ambient = gAmbientColor.rgb * diffuseColor.rgb;
            float3 diffuse = gLightColor.rgb * diffuseColor.rgb * NdotL;
            
            return float4(ambient + diffuse, diffuseColor.a);
        }
    "#;
    
    let mut parser = Parser::<HLSLLanguage>::new();
    match parser.parse(&source) {
        Ok(ast) => {
            println!("Pixel shader parsed successfully!");
        }
        Err(error) => {
            eprintln!("Parse error: {}", error);
        }
    }
}
```

## Advanced Features

### Compute Shaders

Oak HLSL supports parsing compute shaders with thread group semantics:

```rust
let source = r#"
    RWTexture2D<float4> gOutput : register(u0);
    
    [numthreads(8, 8, 1)]
    void main(uint3 id : SV_DispatchThreadID) {
        float4 color = float4(id.x / 255.0, id.y / 255.0, 0.5, 1.0);
        gOutput[id.xy] = color;
    }
"#;
```

### Techniques and Passes

Parse effect files with techniques and passes:

```rust
let source = r#"
    technique11 Main {
        pass P0 {
            SetVertexShader(CompileShader(vs_5_0, VSMain()));
            SetPixelShader(CompileShader(ps_5_0, PSMain()));
            SetGeometryShader(NULL);
        }
    }
"#;
```

## AST Structure

The parser generates a rich AST with the following main node types:

- `HLSLFile` - Root node containing the entire file
- `Struct` - Structure definitions with semantics
- `Function` - Shader functions with return semantics
- `Variable` - Variable declarations with types and semantics
- `Technique` - Technique blocks with passes
- `Pass` - Render pass definitions
- `Expression` - HLSL expressions and operators
- `Statement` - Control flow and assignment statements

## Performance

Oak HLSL is designed for high performance:

- **Zero-copy parsing** where possible
- **Streaming support** for large shader files
- **Efficient memory usage** with minimal allocations
- **Fast error recovery** for better developer experience

## Integration

Oak HLSL integrates seamlessly with the Oak ecosystem:

```rust
use oak::{Parser, Language};
use oak_hlsl::HLSLLanguage;

// Use with other Oak parsers
let mut parser = Parser::<HLSLLanguage>::new();
let result = parser.parse(hlsl_source);
```

## Examples

More examples can be found in the [examples directory](https://github.com/axodotdev/oak/tree/main/examples/oak-hlsl/examples):

- [Vertex shader parsing](examples/vertex_shader.rs)
- [Pixel shader parsing](examples/pixel_shader.rs)
- [Compute shader parsing](examples/compute_shader.rs)
- [Effect file parsing](examples/effect_file.rs)
- [Error handling](examples/error_handling.rs)

## Contributing

We welcome contributions! Please see our [Contributing Guide](https://github.com/axodotdev/oak/blob/main/CONTRIBUTING.md) for details.