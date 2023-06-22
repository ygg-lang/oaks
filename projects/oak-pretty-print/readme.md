# Oak Pretty Print

[![Crates.io](https://img.shields.io/crates/v/oak-pretty-print.svg)](https://crates.io/crates/oak-pretty-print)
[![Documentation](https://docs.rs/oak-pretty-print/badge.svg)](https://docs.rs/oak-pretty-print)

A high-performance, language-agnostic code formatting library built on the Oak ecosystem.

## 🎯 Overview

`oak-pretty-print` provides a flexible and powerful engine for formatting source code. By leveraging `oak-core`'s red-green trees and universal roles, it can format code for any language that implements the Oak traits. It features a rule-based system that allows for fine-grained control over indentation, spacing, line breaks, and more.

## ✨ Features

- **Language Agnostic**: Works with any language that implements `oak-core::Language`.
- **Rule-Based System**: Highly customizable through a priority-based rule engine.
- **Universal Roles**: Can apply formatting based on universal semantic roles (e.g., Container, Definition).
- **Advanced Layout**: Supports complex line-breaking logic using groups and indentation levels.
- **Configurable**: Easily adjust indentation styles, line endings, and maximum line lengths.
- **no_std Support**: Designed to work in restricted environments with only `alloc` required.

## 🚀 Quick Start

Basic usage of the formatter:

```rust
use oak_pretty_print::{Formatter, FormatConfig, RuleSet, create_builtin_rules};
use my_language::MyLanguage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Configure the formatter
    let config = FormatConfig::default()
        .with_indent_style(IndentStyle::Spaces(4))
        .with_max_line_length(80);

    // 2. Set up rules
    let mut rules = RuleSet::new();
    rules.add_rules(create_builtin_rules::<MyLanguage>());

    // 3. Create the formatter
    let mut formatter = Formatter::<MyLanguage>::new(config, rules);

    // 4. Format a node
    let result = formatter.format(&red_node, &source_code)?;
    println!("Formatted code:\n{}", result.content);
    Ok(())
}
```

## 📋 Configuration

The `FormatConfig` allows you to customize the global behavior of the formatter:

```rust
use oak_pretty_print::{FormatConfig, IndentStyle, LineEnding};

let config = FormatConfig::new()
    .with_indent_style(IndentStyle::Tabs)
    .with_line_ending(LineEnding::Unix)
    .with_max_line_length(100)
    .with_trim_trailing_whitespace(true);
```

## 🔧 Advanced Usage

### Custom Formatting Rules

You can create custom rules to handle specific language constructs:

```rust
use oak_pretty_print::{BasicFormatRule, FormatRule, FormatContext};
use oak_core::language::UniversalElementRole;

let rule = BasicFormatRule::new("custom_indent")
    .with_priority(10)
    .with_node_rule(
        |node| node.green.kind.is_universal(UniversalElementRole::Container),
        |_node, context| {
            context.increase_indent();
            Ok(())
        },
    );
```

### Using the `doc!` Macro

For more manual control over document structure, you can use the `doc!` macro from `oak-macros`:

```rust
use oak_macros::doc;

let my_doc = doc! {
    [
        "fn", " ", "main", "()", " ",
        group {
            [
                "{",
                indent {
                    [hard_line, "println!(\"Hello World\");"]
                },
                hard_line,
                "}"
            ]
        }
    ]
};
```

## 🏗️ Architecture

Oak Pretty Print uses a two-stage process:
1. **Rule Application**: AST nodes are visited and rules generate a `Doc` tree.
2. **Printing**: The `Doc` tree is rendered into a string based on the current configuration and layout constraints.

## 📊 Performance

- **Fast Rendering**: The `Doc` tree printer is optimized for linear time complexity relative to the document size.
- **Efficient Memory**: Uses internal pooling for document fragments to minimize allocations.
- **Streaming Support**: Capable of generating output in chunks for large files.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

---

**Oak Pretty Print** - Beautiful code for every language 🚀
