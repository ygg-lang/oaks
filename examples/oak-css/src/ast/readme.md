# CSS AST Module

This module defines the Abstract Syntax Tree (AST) structure for CSS code.

## AST Nodes

- **CssRoot**: The root node of the CSS AST, containing a list of top-level nodes.
- **CssNode**: A variant representing different types of CSS nodes (rule sets, at-rules, comments).
- **RuleSet**: A CSS rule set consisting of selectors and declarations.
- **Declaration**: A CSS declaration consisting of a property name and value.
- **Selector**: A variant representing different types of CSS selectors.
- **Value**: A variant representing different types of CSS values.
- **AtRule**: A variant representing different types of CSS at-rules.

## Usage

```rust
use oak_css::{CssRoot, CssNode, RuleSet, Declaration, Selector, Value};

// Create a simple CSS AST
let ast = CssRoot {
    nodes: vec![
        CssNode::RuleSet(RuleSet {
            selectors: vec![Selector::Element("div".to_string())],
            declarations: vec![Declaration {
                property: "color".to_string(),
                value: Value::Keyword("red".to_string()),
                span: 0..15,
            }],
            span: 0..15,
        }),
    ],
};
```

## Features

- **Serde Support**: AST nodes can be serialized and deserialized when the `serde` feature is enabled.
- **Source Location**: All AST nodes include span information for source location tracking.
- **Comprehensive Structure**: Covers all major CSS constructs including selectors, rules, declarations, and values.