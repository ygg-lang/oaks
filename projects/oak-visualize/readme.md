# 🚀 oak-visualize

[![Crates.io](https://img.shields.io/crates/v/oak-visualize.svg)](https://crates.io/crates/oak-visualize)
[![Documentation](https://docs.rs/oak-visualize/badge.svg)](https://docs.rs/oak-visualize)

**AST Visualization for Oak Languages** — Render syntax trees as graphs and diagrams for debugging and documentation.

## 🎯 Why oak-visualize?

Visualizing syntax trees helps developers understand parser behavior and debug issues. `oak-visualize` provides tools for rendering Oak syntax trees as visual graphs.

## ✨ Key Features

- **📊 Graph Rendering** — Convert syntax trees to visual graph representations
- **🎨 Multiple Output Formats** — SVG, DOT (Graphviz), JSON
- **🔍 Tree Inspection** — Interactive exploration of tree structure
- **📝 Documentation Generation** — Generate visual documentation for grammars
- **🧩 Language Agnostic** — Works with any Oak language parser

## 🏗️ Architecture

- `TreeVisualizer` — Main visualizer with format-specific output methods

### Supported Formats

| Format | Use Case |
|--------|----------|
| SVG | Web display, documentation |
| DOT | Graphviz processing, research |
| JSON | Tooling integration |

## 🔗 Ecosystem Integration

Used by parser developers for debugging, documentation generators, and IDE extensions for tree inspection.

## 📖 Documentation

For usage examples and API details, see the [API documentation](https://docs.rs/oak-visualize).

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
