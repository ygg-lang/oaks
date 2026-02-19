# Wolfram Parser Module

The Wolfram Parser module provides comprehensive syntax analysis for the Wolfram Language. It transforms the stream of tokens produced by the `WolframLexer` into a structured Abstract Syntax Tree (AST), enabling deep analysis and transformation of Wolfram expressions.

## Purpose

The parser is responsible for understanding the hierarchical and operator-heavy structure of the Wolfram Language. It handles operator precedence, complex function call syntax, and various structural forms (lists, associations, etc.) while ensuring the resulting AST accurately reflects the intended symbolic structure.

## Features

- **Recursive Descent Parsing**: Uses an efficient recursive descent strategy to handle the highly recursive nature of Wolfram expressions.
- **Operator Precedence Handling**: Correctly implements the extensive and sometimes complex operator precedence rules of the Wolfram Language.
- **Flexible Function Call Parsing**: Supports standard `f[x]`, prefix `f@x`, and postfix `x//f` function call syntaxes.
- **Pattern and Rule Recognition**: Specialized parsing logic for Wolfram's powerful pattern matching and rule-based programming constructs.
- **Error Recovery**: Implements sophisticated error recovery mechanisms to continue parsing and provide multiple diagnostics even in the presence of syntax errors.
- **Incremental Support**: Designed to work with the Oak incremental parsing framework, re-parsing only changed sections of the source.

## Parsing Process

1. **Token Stream Acquisition**: Receives tokens from the `WolframLexer`.
2. **Expression Analysis**: Identifies the head and arguments of expressions.
3. **Operator Resolution**: Determines the structure of binary and unary operations based on precedence and associativity.
4. **AST Construction**: Builds the `WolframRoot` and constituent `Expression` nodes.
5. **Diagnostic Generation**: Produces warnings or errors for invalid syntax.

## Usage Example

```rust
use oak_wolfram::parser::WolframParser;
use oak_wolfram::lexer::WolframLexer;

fn main() {
    let source = "f[x] + g[y] * z";
    let lexer = WolframLexer::new();
    let tokens = lexer.tokenize(source);
    
    let parser = WolframParser::new();
    let result = parser.parse(tokens);
    
    match result {
        Ok(ast) => println!("Parsed AST with {} top-level expressions", ast.expressions.len()),
        Err(diagnostics) => {
            for diag in diagnostics {
                println!("Error: {}", diag.message);
            }
        }
    }
}
```

## Design Principles

1. **Accuracy**: Aims to match the parsing behavior of the official Wolfram engine.
2. **Performance**: Optimized for fast parsing of large symbolic data sets.
3. **Diagnostics**: Provides clear and actionable error messages for syntax violations.
4. **Flexibility**: Supports various input forms and can be configured for different parsing contexts.
