# Oak Visualization Library

[![Crates.io](https://img.shields.io/crates/v/oak-visualize.svg)](https://crates.io/crates/oak-visualize)
[![Documentation](https://docs.rs/oak-visualize/badge.svg)](https://docs.rs/oak-visualize)

Advanced visualization and layout algorithms for Oak language constructs, including AST visualization, dependency graphs, and code structure diagrams.

## 🎯 Overview

Oak Visualization is a comprehensive library designed to create beautiful and informative visualizations of programming language constructs. Built on the solid foundation of `oak-core`, it provides advanced layout algorithms for trees, graphs, and geometric structures, enabling developers to visualize code structures, dependencies, and complex relationships.

## ✨ Features

- **AST Visualization**: Visualize Abstract Syntax Trees with customizable layouts and node styling.
- **Dependency Graphs**: Create complex dependency and relationship visualizations with automatic edge routing.
- **Advanced Tree Layouts**: Multiple algorithms including Hierarchical (Reingold-Tilford), Radial, and Force-Directed.
- **Geometric Algorithms**: Computational geometry for optimal node positioning and collision avoidance.
- **Interactive HTML**: Generate interactive visualizations with zoom, pan, and tooltip support.
- **Multiple Output Formats**: Export to SVG, PNG, and interactive HTML.

## 🚀 Quick Start

Basic example using `oak-core` integration:

```rust
use oak_visualize::{to_svg, Visualize};
use oak_core::tree::RedNode;

fn main() -> oak_visualize::Result<()> {
    // Assume you have a RedNode from oak-core
    let tree: &RedNode<MyKind> = get_tree();
    
    // One-line visualization to SVG
    let svg = tree.visualize()?;
    
    std::fs::write("tree.svg", svg).unwrap();
    Ok(())
}
```

## 📋 Visualization Examples

### Custom Tree Construction

```rust
use oak_visualize::tree::{TreeLayout, TreeLayoutAlgorithm, TreeNode};

let mut root = TreeNode::new("root", "Binary Expression (+)", "op")
    .with_child(TreeNode::new("l", "42", "num"))
    .with_child(TreeNode::new("r", "8", "num"));

let layout = TreeLayout::new()
    .with_algorithm(TreeLayoutAlgorithm::Layered)
    .with_spacing(50.0, 80.0);

let svg = layout.visualize(&root)?;
```

### Dependency Graph with Force-Directed Layout

```rust
use oak_visualize::graph::{Graph, GraphNode, GraphEdge, GraphLayout, GraphLayoutAlgorithm};

let mut graph = Graph::new(true);
graph.add_node(GraphNode::new("a", "Module A"));
graph.add_node(GraphNode::new("b", "Module B"));
graph.add_edge(GraphEdge::new("a", "b"));

let svg = GraphLayout::new()
    .with_algorithm(GraphLayoutAlgorithm::ForceDirected)
    .with_repulsion(200.0)
    .visualize(&graph)?;
```

## 🔧 Advanced Features

### Custom Styling

```rust
use oak_visualize::{Style, Color, NodeStyle};

let style = Style::new()
    .with_node_style(NodeStyle {
        fill_color: Color::rgb(70, 130, 180),
        stroke_color: Color::rgb(25, 25, 112),
        stroke_width: 2.0,
        font_size: 14.0,
        font_family: "Arial".to_string(),
    });

let layout = TreeLayout::new().with_style(style);
```

### Interactive Features

```rust
use oak_visualize::{HtmlRenderer, InteractiveFeatures};

let features = InteractiveFeatures {
    zoom: true,
    pan: true,
    tooltips: true,
    highlight_on_hover: true,
    clickable_nodes: true,
};

let html = HtmlRenderer::new()
    .with_interactive_features(features)
    .render_visualization(&layout)?;
```

## 🏗️ Layout Algorithms

- **Hierarchical**: Optimized Reingold-Tilford algorithm for compact tree layouts.
- **Radial**: Circular layouts for large trees to maximize space efficiency.
- **Force-Directed**: Physical simulation (Spring-Embedder) for organic graph structures.
- **Sugiyama**: Layered graph drawing for Directed Acyclic Graphs (DAGs).

## 📊 Performance

- **Efficient Layouts**: O(N log N) or O(N) complexity for most tree layout algorithms.
- **Parallel Processing**: Layout computations for large graphs can be parallelized.
- **Incremental Updates**: Support for updating existing visualizations with minimal re-computation.
- **Low Memory Footprint**: Uses optimized geometric primitives and minimal intermediate allocations.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

---

**Oak Visualization** - Bringing code structure to life 🚀
