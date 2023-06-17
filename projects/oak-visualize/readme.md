# Oak Visualization Library

[![Crates.io](https://img.shields.io/crates/v/oak-visualize.svg)](https://crates.io/crates/oak-visualize)
[![Documentation](https://docs.rs/oak-visualize/badge.svg)](https://docs.rs/oak-visualize)

Advanced visualization and layout algorithms for Pex language constructs, including AST visualization, dependency graphs, and code structure diagrams.

## 🎯 Overview

Oak of visualize is a comprehensive visualization library designed to create beautiful and informative visualizations of Pex language constructs. Built on the solid foundation of oak-core, it provides advanced layout algorithms for trees, graphs, and geometric structures, enabling developers to visualize code structures, dependencies, and relationships.

## ✨ Features

- **AST Visualization**: Visualize Abstract Syntax Trees with customizable layouts
- **Dependency Graphs**: Create dependency and relationship visualizations
- **Tree Layouts**: Multiple tree layout algorithms (hierarchical, radial, force-directed)
- **Graph Layouts**: Advanced graph layout algorithms for complex structures
- **Geometric Algorithms**: Computational geometry for optimal positioning
- **Multiple Output Formats**: SVG, PNG, and interactive HTML outputs

## 🚀 Quick Start

Basic example:

```rust
use oak_visualize::{Visualize, TreeLayout};
use oak_core::ast::AstNode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a simple AST
    let ast = create_sample_ast();
    
    // Generate tree visualization
    let visualizer = TreeLayout::new();
    let svg = visualizer.visualize(&ast)?;
    
    println!("Generated SVG: {}", svg);
    Ok(())
}
```

## 📋 Visualization Examples

### AST Tree Visualization
```rust
use oak_visualize::{TreeLayout, Visualize};
use oak_core::ast::{AstNode, BinaryExpr, Literal};

let mut root = AstNode::BinaryExpr(BinaryExpr {
    left: Box::new(AstNode::Literal(Literal::Number(42.0))),
    operator: "+".to_string(),
    right: Box::new(AstNode::Literal(Literal::Number(8.0))),
});

let layout = TreeLayout::new()
    .with_direction(TreeDirection::TopDown)
    .with_spacing(50.0, 80.0);

let svg = layout.visualize(&root)?;
std::fs::write("ast_tree.svg", svg)?;
```

### Dependency Graph Visualization
```rust
use oak_visualize::{GraphLayout, Graph, Node, Edge};
use std::collections::HashMap;

let mut graph = Graph::new();

// Add nodes
let node_a = graph.add_node(Node::new("Module A"));
let node_b = graph.add_node(Node::new("Module B"));
let node_c = graph.add_node(Node::new("Module C"));

// Add edges (dependencies)
graph.add_edge(Edge::new(node_a, node_b, "imports"));
graph.add_edge(Edge::new(node_b, node_c, "uses"));
graph.add_edge(Edge::new(node_a, node_c, "references"));

// Create force-directed layout
let layout = GraphLayout::force_directed()
    .with_repulsion(100.0)
    .with_attraction(0.1)
    .with_iterations(100);

let svg = layout.visualize(&graph)?;
std::fs::write("dependency_graph.svg", svg)?;
```

### Radial Tree Layout
```rust
use oak_visualize::{RadialLayout, Tree, TreeNode};

let mut tree = Tree::new("Root Module");

// Build hierarchical structure
let child1 = tree.root.add_child("Child 1");
let child2 = tree.root.add_child("Child 2");

child1.add_child("Grandchild 1.1");
child1.add_child("Grandchild 1.2");
child2.add_child("Grandchild 2.1");

// Create radial layout
let layout = RadialLayout::new()
    .with_radius(200.0)
    .with_angle_spacing(30.0);

let svg = layout.visualize(&tree)?;
std::fs::write("radial_tree.svg", svg)?;
```

## 🔧 Advanced Features

### Custom Styling
```rust
use oak_visualize::{Style, Color, NodeStyle, EdgeStyle};

let style = Style::new()
    .with_node_style(NodeStyle {
        fill_color: Color::rgb(70, 130, 180),
        stroke_color: Color::rgb(25, 25, 112),
        stroke_width: 2.0,
        font_size: 14.0,
        font_family: "Arial".to_string(),
    })
    .with_edge_style(EdgeStyle {
        stroke_color: Color::rgb(105, 105, 105),
        stroke_width: 1.5,
        arrow_size: 8.0,
    });

let layout = TreeLayout::new().with_style(style);
```

### Interactive HTML Output
```rust
use oak_visualize::{HtmlRenderer, InteractiveFeatures};

let features = InteractiveFeatures {
    zoom: true,
    pan: true,
    tooltips: true,
    highlight_on_hover: true,
    clickable_nodes: true,
};

let renderer = HtmlRenderer::new()
    .with_interactive_features(features)
    .with_responsive(true);

let html = renderer.render_visualization(&layout)?;
std::fs::write("interactive_visualization.html", html)?;
```

### Custom Layout Algorithms
```rust
use oak_visualize::{LayoutAlgorithm, Position, Node};

struct CustomLayout {
    spacing: f64,
}

impl LayoutAlgorithm for CustomLayout {
    fn layout(&self, nodes: &[Node]) -> Result<Vec<Position>, LayoutError> {
        let mut positions = Vec::new();
        
        for (i, node) in nodes.iter().enumerate() {
            let x = (i as f64) * self.spacing;
            let y = (i as f64).sin() * 100.0; // Sine wave pattern
            positions.push(Position::new(x, y));
        }
        
        Ok(positions)
    }
}
```

### Geometric Computations
```rust
use oak_visualize::geometry::{Point, Rectangle, Circle};
use oak_visualize::algorithms::{intersection, bounding_box};

let rect = Rectangle::new(Point::new(0.0, 0.0), 100.0, 80.0);
let circle = Circle::new(Point::new(50.0, 40.0), 30.0);

// Check intersection
if intersection::rectangle_circle(&rect, &circle) {
    println!("Rectangle and circle intersect");
}

// Calculate bounding box
let points = vec![Point::new(10.0, 20.0), Point::new(30.0, 40.0), Point::new(50.0, 60.0)];
let bbox = bounding_box::from_points(&points)?;
println!("Bounding box: {:?}", bbox);
```

## 🏗️ Layout Algorithms

The library provides multiple layout algorithms:

- **Tree Layouts**: Hierarchical, radial, force-directed tree layouts
- **Graph Layouts**: Force-directed, circular, hierarchical graph layouts
- **Geometric Layouts**: Grid-based, spiral, custom geometric patterns
- **Optimization**: Minimize edge crossings, optimize node spacing

## 📊 Performance

- **Efficient Algorithms**: Optimized layout algorithms for large datasets
- **Incremental Updates**: Support for incremental layout updates
- **Memory Efficient**: Minimal memory footprint for large visualizations
- **Parallel Processing**: Multi-threaded layout computation where applicable

## 🔗 Integration

Oak of visualize integrates seamlessly with:

- **oak-core**: Direct integration with AST structures
- **Web Applications**: Generate SVG/HTML for web display
- **Documentation Tools**: Create visual documentation
- **IDE Plugins**: Visual debugging and code exploration
- **Analysis Tools**: Visualize code metrics and dependencies

## 📚 Examples

Check out the [examples](examples/) directory for comprehensive examples:

- AST visualization for different programming languages
- Dependency graph generation and layout
- Interactive HTML visualizations
- Custom styling and theming
- Performance optimization techniques

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

---

**Pex Visualization Library** - Bringing code structures to life through beautiful visualizations 🎨