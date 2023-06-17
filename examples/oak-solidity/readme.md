# Oak Solidity Parser

[![Crates.io](https://img.shields.io/crates/v/oak-solidity.svg)](https://crates.io/crates/oak-solidity)
[![Documentation](https://docs.rs/oak-solidity/badge.svg)](https://docs.rs/oak-solidity)

A Solidity parser for the Oak parsing framework, providing robust parsing capabilities for Ethereum smart contracts and Solidity language constructs.

## Features

- **Complete Solidity Support**: Parse contracts, interfaces, libraries, functions, modifiers, events, and structs
- **Modern Solidity Versions**: Support for Solidity ^0.8.0 syntax and features
- **State Variables**: Handle visibility modifiers, mutability, and initializations
- **Function Parsing**: Support for function signatures, modifiers, parameters, and return types
- **Expression Parsing**: Comprehensive expression support including function calls, member access, and operators
- **Statement Parsing**: Handle control flow (if/else, for, while), try/catch, and assembly blocks
- **Error Handling**: Detailed error messages with line and column information
- **AST Generation**: Rich Abstract Syntax Tree for semantic analysis and code generation
- **Zero Dependencies**: Pure Rust implementation with no external dependencies

## 🎯 Overview

Oak Solidity is a robust parser for Solidity, designed to handle complete Solidity syntax including modern features. Built on the solid foundation of oak-core, it provides both high-level convenience and detailed AST generation for Solidity analysis and tooling.

## 🚀 Quick Start

```rust
use oak::Parser;
use oak_solidity::SolidityLanguage;

fn main() {
    let source = r#"
        pragma solidity ^0.8.0;
        
        contract SimpleStorage {
            uint256 private storedData;
            
            function set(uint256 x) public {
                storedData = x;
            }
            
            function get() public view returns (uint256) {
                return storedData;
            }
        }
    "#;
    
    let mut parser = Parser::new();
    let language = SolidityLanguage::new();
    
    match parser.parse(&source, &language) {
        Ok(ast) => {
            println!("Successfully parsed Solidity contract!");
            println!("AST: {:#?}", ast);
        }
        Err(error) => {
            eprintln!("Parse error: {}", error);
        }
    }
}
```

## 📋 Parsing Examples

### Parsing Complex Contracts

```rust
use oak::Parser;
use oak_solidity::SolidityLanguage;

fn main() {
    let complex_contract = r#"
        pragma solidity ^0.8.0;
        
        interface IERC20 {
            function transfer(address to, uint256 amount) external returns (bool);
            function balanceOf(address account) external view returns (uint256);
        }
        
        contract TokenVault is Ownable {
            IERC20 public token;
            mapping(address => uint256) public deposits;
            
            event Deposit(address indexed user, uint256 amount);
            event Withdrawal(address indexed user, uint256 amount);
            
            modifier validAmount(uint256 amount) {
                require(amount > 0, "Amount must be positive");
                _;
            }
            
            constructor(address _token) {
                token = IERC20(_token);
            }
            
            function deposit(uint256 amount) external validAmount(amount) {
                require(token.transferFrom(msg.sender, address(this), amount), "Transfer failed");
                deposits[msg.sender] += amount;
                emit Deposit(msg.sender, amount);
            }
            
            function withdraw(uint256 amount) external validAmount(amount) {
                require(deposits[msg.sender] >= amount, "Insufficient balance");
                deposits[msg.sender] -= amount;
                require(token.transfer(msg.sender, amount), "Transfer failed");
                emit Withdrawal(msg.sender, amount);
            }
        }
    "#;
    
    let mut parser = Parser::new();
    let language = SolidityLanguage::new();
    
    match parser.parse(&complex_contract, &language) {
        Ok(ast) => {
            println!("Successfully parsed complex contract with inheritance!");
            // Process the AST for semantic analysis or code generation
        }
        Err(error) => {
            eprintln!("Parse error at line {}: {}", error.line(), error.message());
        }
    }
}
```

### Error Handling

```rust
use oak::Parser;
use oak_solidity::SolidityLanguage;

fn parse_with_diagnostics(source: &str) {
    let mut parser = Parser::new();
    let language = SolidityLanguage::new();
    
    match parser.parse(source, &language) {
        Ok(ast) => {
            println!("Parsed successfully!");
        }
        Err(error) => {
            eprintln!("Parse error at line {}, column {}", error.line(), error.column());
            eprintln!("Error: {}", error.message());
            
            // Show context around the error
            let lines: Vec<&str> = source.lines().collect();
            if error.line() > 0 && error.line() <= lines.len() {
                eprintln!("Context:");
                eprintln!("  {}", lines[error.line() - 1]);
                eprintln!("  {}^", " ".repeat(error.column()));
            }
        }
    }
}
```

## 🏗️ AST Structure

The parser generates a rich AST with the following main node types:

- **SourceUnit**: Top-level container for all Solidity constructs
- **PragmaDirective**: Version and compiler directives
- **ImportDirective**: Import statements
- **ContractDefinition**: Contract, interface, and library definitions
- **FunctionDefinition**: Function declarations with modifiers and body
- **ModifierDefinition**: Custom function modifiers
- **EventDefinition**: Event declarations
- **StateVariableDeclaration**: Contract state variables
- **ExpressionStatement**: Various expression types including:
  - Function calls and member access
  - Binary and unary operations
  - Conditional expressions
  - Array and struct access
- **Statement**: Control flow and other statements

## 📊 Performance

- **Zero-copy parsing**: Minimal string allocations during parsing
- **Streaming support**: Parse large contracts incrementally
- **Error recovery**: Continue parsing after encountering errors
- **Memory efficient**: Compact AST representation

## 🔗 Integration

The parser integrates seamlessly with the broader Oak ecosystem:

```rust
use oak::Parser;
use oak_solidity::SolidityLanguage;

// Use with other Oak tools for analysis and transformation
fn analyze_solidity_contract(source: &str) -> Result<Analysis, ParseError> {
    let mut parser = Parser::new();
    let language = SolidityLanguage::new();
    let ast = parser.parse(source, &language)?;
    
    // Perform semantic analysis, linting, or transformations
    perform_analysis(ast)
}
```

## 🤝 Contributing

Contributions are welcome! 

Please feel free to submit pull requests at the [project repository](https://github.com/ygg-lang/oaks/tree/dev/examples/oak-solidity) or open [issues](https://github.com/ygg-lang/oaks/issues).