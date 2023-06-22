# Oak VHDL Parser

[![Crates.io](https://img.shields.io/crates/v/oak-vhdl.svg)](https://crates.io/crates/oak-vhdl)
[![Documentation](https://docs.rs/oak-vhdl/badge.svg)](https://docs.rs/oak-vhdl)

High-performance incremental VHDL parser for the oak ecosystem with flexible configuration, optimized for hardware description and digital circuit design.

## 🎯 Overview

Oak VHDL is a robust parser for VHDL, designed to handle complete VHDL syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for hardware description and digital circuit design.

## ✨ Features

- **Complete VHDL Syntax**: Supports all VHDL features including modern specifications
- **Full AST Generation**: Generates comprehensive Abstract Syntax Trees
- **Lexer Support**: Built-in tokenization with proper span information
- **Error Recovery**: Graceful handling of syntax errors with detailed diagnostics

## 🚀 Quick Start

Basic example:

```rust
use oak_core::{Parser, source::SourceText, ParseSession};
use oak_vhdl::{VhdlParser, VhdlLanguage};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let language = VhdlLanguage::default();
    let parser = VhdlParser::new(&language);
    let mut session = ParseSession::<VhdlLanguage>::default();
    let source = SourceText::new(r#"
library IEEE;
use IEEE.STD_LOGIC_1164.ALL;
use IEEE.NUMERIC_STD.ALL;

entity counter is
    Port ( clk : in STD_LOGIC;
           reset : in STD_LOGIC;
           count : out STD_LOGIC_VECTOR (3 downto 0));
end counter;

architecture Behavioral of counter is
    signal internal_count : unsigned(3 downto 0) := (others => '0');
begin
    process(clk, reset)
    begin
        if reset = '1' then
            internal_count <= (others => '0');
        elsif rising_edge(clk) then
            internal_count <= internal_count + 1;
        end if;
    end process;
    
    count <= std_logic_vector(internal_count);
end Behavioral;
    "#);
    
    let result = parser.parse(&source, &[], &mut session);
    println!("Parsed VHDL successfully.");
    Ok(())
}
```

## 📋 Parsing Examples

### Entity Parsing
```rust
use oak_core::{Parser, source::SourceText, ParseSession};
use oak_vhdl::{VhdlParser, VhdlLanguage};

let language = VhdlLanguage::default();
let parser = VhdlParser::new(&language);
let mut session = ParseSession::<VhdlLanguage>::default();
let source = SourceText::new(r#"
library IEEE;
use IEEE.STD_LOGIC_1164.ALL;

entity full_adder is
    Port ( a : in STD_LOGIC;
           b : in STD_LOGIC;
           cin : in STD_LOGIC;
           sum : out STD_LOGIC;
           cout : out STD_LOGIC);
end full_adder;
"#);

let result = parser.parse(&source, &[], &mut session);
println!("Entity parsed successfully.");
```

### Architecture Parsing
```rust
use oak_core::{Parser, source::SourceText, ParseSession};
use oak_vhdl::{VhdlParser, VhdlLanguage};

let language = VhdlLanguage::default();
let parser = VhdlParser::new(&language);
let mut session = ParseSession::<VhdlLanguage>::default();
let source = SourceText::new(r#"
architecture Structural of full_adder is
begin
    sum <= a xor b xor cin;
    cout <= (a and b) or (cin and (a xor b));
end Structural;
"#);

let result = parser.parse(&source, &[], &mut session);
println!("Architecture parsed successfully.");
```

### Package Parsing
```rust
use oak_core::{Parser, source::SourceText, parser::session::ParseSession};
use oak_vhdl::{VhdlParser, VhdlLanguage};

let language = VhdlLanguage::default();
let parser = VhdlParser::new(&language);
let mut session = ParseSession::<VhdlLanguage>::default();
let source = SourceText::new(r#"
package my_types is
    type state_type is (IDLE, READ, WRITE, DONE);
    constant MAX_COUNT : integer := 255;
end my_types;
"#);

let result = parser.parse(&source, &[], &mut session);
println!("Package parsed successfully.");
```

## 🔧 Advanced Features

### Token-Level Parsing
```rust
use oak_core::{Parser, source::SourceText, parser::session::ParseSession};
use oak_vhdl::{VhdlParser, VhdlLanguage};

let language = VhdlLanguage::default();
let parser = VhdlParser::new(&language);
let mut session = ParseSession::<VhdlLanguage>::default();
let source = SourceText::new("entity Test is end Test;");
let result = parser.parse(&source, &[], &mut session);
// Token information is available in the parse result
```

### Error Handling
```rust
use oak_core::{Parser, source::SourceText, parser::session::ParseSession};
use oak_vhdl::{VhdlParser, VhdlLanguage};

let language = VhdlLanguage::default();
let parser = VhdlParser::new(&language);
let mut session = ParseSession::<VhdlLanguage>::default();
let source = SourceText::new(r#"
entity Broken is
    Port ( clk : in STD_LOGIC -- Missing semicolon
end Broken;
"#);

let result = parser.parse(&source, &[], &mut session);
if let Err(e) = result.result {
    println!("Parse error: {:?}", e);
}
```

## 🏗️ AST Structure

The parser generates a comprehensive AST with the following main structures:

- **VhdlSource**: Root container for VHDL source files
- **DesignUnit**: VHDL design units (entity, architecture, package, etc.)
- **Entity**: Entity declarations with ports and generics
- **Architecture**: Architecture implementations with statements
- **Process**: Process statements with sensitivity lists
- **SignalDeclaration**: Signal and variable declarations
- **ConcurrentStatement**: Concurrent statements (assignments, instances, etc.)
- **SequentialStatement**: Sequential statements within processes

## 📊 Performance

- **Streaming**: Parse large VHDL files without loading entirely into memory
- **Incremental**: Re-parse only changed sections
- **Memory Efficient**: Smart AST node allocation
- **Fast Recovery**: Quick error recovery for better IDE integration

## 🔗 Integration

Oak VHDL integrates seamlessly with:

- **Hardware Design**: Building hardware design tools
- **Simulation**: Creating simulation and verification tools
- **Synthesis**: Front-end for synthesis tools
- **IDE Support**: Language server protocol compatibility for VHDL
- **Educational Tools**: Building VHDL learning environments

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- Complete VHDL design unit parsing
- Hardware description analysis
- Integration with development workflows

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-vhdl) or open [issues](https://github.com/ygg-lang/oaks/issues).