# oak-jinja

High-performance incremental Jinja2 parser for the oak ecosystem with flexible configuration, supporting template rendering and syntax analysis.

## Features

- **Incremental Parsing**: Efficiently parses only changed parts of templates
- **Flexible Configuration**: Customizable syntax and behavior
- **Template Rendering Support**: Built-in support for Jinja2 template rendering
- **Syntax Analysis**: Detailed AST for advanced tooling
- **LSP Integration**: Supports language server protocol for IDE integration

## Usage

```rust
use oak_jinja::{JinjaLexer, JinjaParser, JinjaBuilder};

let source = "Hello, {{ name }}!";
let lexer = JinjaLexer::new();
let tokens = lexer.tokenize(source);
let parser = JinjaParser::new();
let ast = parser.parse(tokens);
let builder = JinjaBuilder::new();
let root = builder.build(ast);
```

## Syntax Support

- **Variables**: `{{ variable }}`
- **Blocks**: `{% block name %}{% endblock %}`
- **Comments**: `{# comment #}`
- **Control Structures**: `{% if %}`, `{% for %}`, `{% while %}`
- **Macros**: `{% macro name() %}{% endmacro %}`
- **Includes**: `{% include "template.html" %}`
- **Extensions**: `{% extends "base.html" %}`

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/ygg-lang/oaks/blob/master/License.md) file for details.