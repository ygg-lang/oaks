# oak-rst

High-performance incremental reStructuredText parser for the oak ecosystem.

## Features

- Incremental parsing for efficient editing
- Support for core reStructuredText syntax
- Extensible architecture
- Integration with the oak ecosystem

## Usage

```rust
use oak_rst::{RstLanguage, RstParser};

let language = RstLanguage::default();
let parser = RstParser::new(&language);

// Parse reStructuredText content
let content = "Hello World\n==========\n\nThis is a paragraph.";
let parse_result = parser.parse(content, &[], &mut ());

// Access the parsed AST
let root = parse_result.unwrap();
```

## Supported Syntax

- Headings
- Paragraphs
- Lists (bullet points)
- Tables
- Code blocks
- Comments
- Directives
- Emphasis and strong text
- Literal text
- Links and references

## Configuration

The `RstLanguage` struct allows you to configure various aspects of the parser:

- `allow_directives`: Enable/disable directives
- `allow_substitutions`: Enable/disable substitutions
- `allow_roles`: Enable/disable roles
- `allow_footnotes`: Enable/disable footnotes
- `allow_citations`: Enable/disable citations
- `allow_admonitions`: Enable/disable admonitions

## License

This project is licensed under the MIT License.