# Oak Verilog Parser

[![Crates.io](https://img.shields.io/crates/v/oak-verilog.svg)](https://crates.io/crates/oak-verilog)
[![Documentation](https://docs.rs/oak-verilog/badge.svg)](https://docs.rs/oak-verilog)

High-performance incremental Verilog parser for the oak ecosystem with flexible configuration, optimized for hardware description and verification.

## 🎯 Overview

Oak Verilog is a robust parser for Verilog, designed to handle complete Verilog syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for hardware description and verification.

## ✨ Features

- **Complete Verilog Syntax**: Supports all Verilog features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_verilog::{Parser, VerilogLanguage, SourceText};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = Parser::new();
    let source = SourceText::new(r#"
module counter (
    input wire clk,
    input wire reset,
    output reg [3:0] count
);
    always @(posedge clk or posedge reset) begin
        if (reset)
            count <= 4'b0000;
        else
            count <= count + 1;
    end
endmodule
    "#);
    
    let result = parser.parse(&source);
    println!("Parsed Verilog successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Module Parsing
```rust
use oak_verilog::{Parser, VerilogLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
module full_adder (
    input a, b, cin,
    output sum, cout
);
    assign sum = a ^ b ^ cin;
    assign cout = (a & b) | (b & cin) | (a & cin);
endmodule
"#);

let result = parser.parse(&source);
println!("Module parsed successfully.");
```

### Sequential Logic Parsing
```rust
use oak_verilog::{Parser, VerilogLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
module d_flip_flop (
    input wire clk,
    input wire d,
    output reg q
);
    always @(posedge clk) begin
        q <= d;
    end
endmodule
"#);

let result = parser.parse(&source);
println!("Sequential logic parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_verilog::{Parser, VerilogLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new("module test(); endmodule");
let result = parser.parse(&source);
println!("Token parsing completed.");
```

### Error Handling
```rust
use oak_verilog::{Parser, VerilogLanguage, SourceText};

let parser = Parser::new();
let source = SourceText::new(r#"
module broken_module (
    input wire clk,
    // Missing closing parenthesis
    output reg [3:0] count
);
    always @(posedge clk begin
        count <= count + 1;
    end
endmodule
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

- **VerilogSource**: Root container for Verilog source files
- **Module**: Verilog module definitions
- **Port**: Module port declarations
- **Declaration**: Variable and net declarations
- **Statement**: Procedural statements and blocks
- **Expression**: Various expression types including operators
- **Instance**: Module instantiation

## 📊 Performance

- **Streaming**: Parse large Verilog files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak Verilog integrates seamlessly with:

- **Hardware Design**: Building hardware design tools
- **Verification**: Creating verification and simulation tools
- **Synthesis**: Front-end for synthesis tools
- **IDE Support**: Language server protocol compatibility for Verilog
- **Educational Tools**: Building Verilog learning environments

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete Verilog module parsing
- Hardware description analysis
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-verilog) or open [issues](https://github.com/ygg-lang/oaks/issues).