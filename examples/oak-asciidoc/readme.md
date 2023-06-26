# oak-asciidoc

High-performance incremental AsciiDoc parser for the oak ecosystem.

## Features

- Incremental parsing for efficient editing
- Support for core AsciiDoc syntax
- Extensible architecture
- Integration with the oak ecosystem

## Usage

```rust
use oak_asciidoc::{AsciidocLanguage, AsciidocParser};

let language = AsciidocLanguage::default();
let parser = AsciidocParser::new(&language);

// Parse AsciiDoc content
let content = "= Hello World\n\nThis is a paragraph.\n\n- List item 1\n- List item 2";
let parse_result = parser.parse(content, &[], &mut ());

// Access the parsed AST
let root = parse_result.unwrap();
```

## Supported Syntax

- Headings
- Paragraphs
- Lists
- Tables
- Code blocks
- Comments
- Blocks
- Macros
- Attributes
- Cross-references
- Footnotes
- Emphasis and strong text
- Monospace text
- Links and images

## Configuration

The `AsciidocLanguage` struct allows you to configure various aspects of the parser:

- `allow_macros`: Enable/disable macros
- `allow_attributes`: Enable/disable attributes
- `allow_blocks`: Enable/disable blocks
- `allow_footnotes`: Enable/disable footnotes
- `allow_cross_references`: Enable/disable cross-references
- `allow_includes`: Enable/disable include directives

## License

This project is licensed under the MIT License.