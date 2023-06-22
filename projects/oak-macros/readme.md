# Oak Macros

[![Crates.io](https://img.shields.io/crates/v/oak-macros.svg)](https://crates.io/crates/oak-macros)
[![Documentation](https://docs.rs/oak-macros/badge.svg)](https://docs.rs/oak-macros)

Procedural macros for the Oak ecosystem, providing a domain-specific language for building document structures and trees.

## 🎯 Overview

Oak Macros simplifies the creation of complex data structures used within the Oak framework. Its primary feature is the `doc!` macro, which provides a declarative syntax for building `Doc` trees used in `oak-pretty-print`.

## ✨ Features

- **Declarative Syntax**: Use a natural, code-like syntax to define document structures.
- **Pretty Printing DSL**: Built-in support for indentation, groups, and line breaking logic.
- **Type Safety**: Compile-time checking of macro inputs.
- **Extensible**: Designed to support additional macros as the Oak ecosystem grows.

## 🚀 Quick Start

Using the `doc!` macro for pretty printing:

```rust
use oak_macros::doc;
use oak_pretty_print::Doc;

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

## 📋 DSL Syntax

The `doc!` macro supports several specialized keywords:

- `nil`: An empty document.
- `line`: A mandatory newline.
- `soft_line`: A newline that collapses to nothing if it fits on one line.
- `soft_line_space`: A newline that collapses to a space if it fits.
- `hard_line`: A mandatory newline.
- `indent { ... }`: Indents the enclosed content.
- `group { ... }`: Treats the content as a single unit for line-breaking decisions.
- `[...]`: Concatenates multiple document elements.
- `"text"`: Literal text content.

## 🏗️ Integration

Oak Macros is primarily used with:

- **Oak Pretty Print**: The primary consumer of the `doc!` macro for code formatting.
- **Oak Code Generators**: For generating beautifully formatted source code.

## 📊 Performance

- **Zero Runtime Overhead**: Macros expand at compile time into efficient Rust code.
- **Minimal Allocations**: Generated code uses optimized `Vec` and `Box` structures.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

---

**Oak Macros** - Simplifying tree construction with powerful DSLs 🚀
