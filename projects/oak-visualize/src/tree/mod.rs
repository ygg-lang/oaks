#![doc = "Tree layout algorithms for AST visualization"]

use crate::{
    geometry::{Point, Rect, Size},
    layout::{Edge, Layout},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

static NODE_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn next_node_id() -> String {
    format!("node_{}", NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst))
}

/// Tree node for visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeNode {
    pub id: String,
    pub label: String,
    pub node_type: String,
    pub children: Vec<TreeNode>,
    pub attributes: HashMap<String, String>,
    pub size: Option<Size>,
}

impl TreeNode {
    pub fn new(id: String, label: String, node_type: String) -> Self {
        Self { id, label, node_type, children: Vec::new(), attributes: HashMap::new(), size: None }
    }

    pub fn with_child(mut self, child: TreeNode) -> Self {
        self.children.push(child);
        self
    }

    pub fn with_children(mut self, children: Vec<TreeNode>) -> Self {
        self.children = children;
        self
    }

    pub fn with_attribute(mut self, key: String, value: String) -> Self {
        self.attributes.insert(key, value);
        self
    }

    pub fn with_size(mut self, size: Size) -> Self {
        self.size = Some(size);
        self
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    pub fn depth(&self) -> usize {
        if self.children.is_empty() { 1 } else { 1 + self.children.iter().map(|child| child.depth()).max().unwrap_or(0) }
    }

    pub fn node_count(&self) -> usize {
        1 + self.children.iter().map(|child| child.node_count()).sum::<usize>()
    }

    pub fn leaf_count(&self) -> usize {
        if self.children.is_empty() { 1 } else { self.children.iter().map(|child| child.leaf_count()).sum() }
    }
}

impl crate::Visualize for TreeNode {
    fn visualize(&self) -> crate::Result<String> {
        TreeLayout::new().visualize(self)
    }
}

// Bridge for oak-core types
impl<'a, L: oak_core::Language> From<&oak_core::GreenTree<'a, L>> for TreeNode {
    fn from(green: &oak_core::GreenTree<'a, L>) -> Self {
        match green {
            oak_core::GreenTree::Node(node) => {
                let mut tree_node = TreeNode::new(next_node_id(), format!("{:?}", node.kind), "node".to_string());
                for child in node.children() {
                    tree_node.children.push(TreeNode::from(child));
                }
                tree_node
            }
            oak_core::GreenTree::Leaf(leaf) => TreeNode::new(next_node_id(), format!("{:?}", leaf.kind), "leaf".to_string()),
        }
    }
}

impl<'a, L: oak_core::Language> From<&oak_core::RedTree<'a, L>> for TreeNode {
    fn from(red: &oak_core::RedTree<'a, L>) -> Self {
        match red {
            oak_core::RedTree::Node(node) => {
                let mut tree_node = TreeNode::new(next_node_id(), format!("{:?}", node.green.kind), "node".to_string());
                for child in node.children() {
                    tree_node.children.push(TreeNode::from(child));
                }
                tree_node
            }
            oak_core::RedTree::Leaf(leaf) => TreeNode::new(next_node_id(), format!("{:?}", leaf.kind), "leaf".to_string()),
        }
    }
}

impl<'a, L: oak_core::Language> From<oak_core::GreenTree<'a, L>> for TreeNode {
    fn from(green: oak_core::GreenTree<'a, L>) -> Self {
        TreeNode::from(&green)
    }
}

impl<'a, L: oak_core::Language> From<oak_core::RedTree<'a, L>> for TreeNode {
    fn from(red: oak_core::RedTree<'a, L>) -> Self {
        TreeNode::from(&red)
    }
}

impl<'a, L: oak_core::Language> crate::Visualize for oak_core::GreenTree<'a, L> {
    fn visualize(&self) -> crate::Result<String> {
        TreeNode::from(self).visualize()
    }
}

impl<'a, L: oak_core::Language> crate::Visualize for oak_core::RedTree<'a, L> {
    fn visualize(&self) -> crate::Result<String> {
        TreeNode::from(self).visualize()
    }
}

/// Tree layout engine
pub struct TreeLayout {
    algorithm: TreeLayoutAlgorithm,
    config: TreeLayoutConfig,
}

impl Default for TreeLayout {
    fn default() -> Self {
        Self { algorithm: TreeLayoutAlgorithm::Layered, config: TreeLayoutConfig::default() }
    }
}

impl TreeLayout {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_algorithm(mut self, algorithm: TreeLayoutAlgorithm) -> Self {
        self.algorithm = algorithm;
        self
    }

    pub fn with_config(mut self, config: TreeLayoutConfig) -> Self {
        self.config = config;
        self
    }

    pub fn visualize(&self, tree: &TreeNode) -> crate::Result<String> {
        let layout = self.layout_tree(tree)?;
        crate::render::SvgRenderer::new().render_layout(&layout)
    }

    pub fn layout_tree(&self, tree: &TreeNode) -> crate::Result<Layout> {
        match self.algorithm {
            TreeLayoutAlgorithm::Layered => self.layered_layout(tree),
            TreeLayoutAlgorithm::Radial => self.radial_layout(tree),
            TreeLayoutAlgorithm::Compact => self.compact_layout(tree),
            TreeLayoutAlgorithm::Balloon => self.balloon_layout(tree),
        }
    }

    /// Layered tree layout (traditional top-down or left-right)
    fn layered_layout(&self, tree: &TreeNode) -> crate::Result<Layout> {
        let mut layout = Layout::new();
        let mut node_positions = HashMap::new();
        let mut level_widths = HashMap::new();

        // Calculate level widths
        self.calculate_level_widths(tree, 0, &mut level_widths);

        // Position nodes
        let mut current_positions = HashMap::new();
        self.position_layered_node(tree, 0, 0.0, &level_widths, &mut current_positions, &mut node_positions);

        // Add nodes to layout
        for (id, (position, size, label, node_type)) in node_positions {
            let rect = Rect::new(position, size);
            let nt = match node_type.as_str() {
                "function" => crate::layout::NodeType::Function,
                "struct" => crate::layout::NodeType::Struct,
                "module" => crate::layout::NodeType::Module,
                _ => crate::layout::NodeType::Default,
            };
            layout.add_node_with_metadata(id, label, rect, nt);
        }

        // Add edges
        self.add_tree_edges(tree, &mut layout);

        Ok(layout)
    }

    /// Radial tree layout (root at center, children in circles)
    fn radial_layout(&self, tree: &TreeNode) -> crate::Result<Layout> {
        let mut layout = Layout::new();
        let mut node_positions = HashMap::new();

        // Position root at center
        let root_size = self.get_node_size(tree);
        let root_pos = Point::new(0.0, 0.0);
        node_positions.insert(tree.id.clone(), (root_pos, root_size, tree.label.clone(), tree.node_type.clone()));

        // Position children in concentric circles
        if !tree.children.is_empty() {
            let radius = self.config.level_distance;
            self.position_radial_children(&tree.children, root_pos, radius, 0.0, 2.0 * std::f64::consts::PI, 1, &mut node_positions);
        }

        // Add nodes to layout
        for (id, (position, size, label, node_type)) in node_positions {
            let rect = Rect::new(Point::new(position.x - size.width / 2.0, position.y - size.height / 2.0), size);
            let nt = match node_type.as_str() {
                "function" => crate::layout::NodeType::Function,
                "struct" => crate::layout::NodeType::Struct,
                "module" => crate::layout::NodeType::Module,
                _ => crate::layout::NodeType::Default,
            };
            layout.add_node_with_metadata(id, label, rect, nt);
        }

        // Add edges
        self.add_tree_edges(tree, &mut layout);

        Ok(layout)
    }

    /// Compact tree layout (minimize space usage)
    fn compact_layout(&self, tree: &TreeNode) -> crate::Result<Layout> {
        let mut layout = Layout::new();
        let mut node_positions = HashMap::new();

        // Use a modified layered approach with tighter packing
        let positioned_tree = self.position_compact_node(tree, 0.0, 0.0, 0);
        self.extract_positions(&positioned_tree, &mut node_positions);

        // Add nodes to layout
        for (id, (position, size, label, node_type)) in node_positions {
            let rect = Rect::new(position, size);
            let nt = match node_type.as_str() {
                "function" => crate::layout::NodeType::Function,
                "struct" => crate::layout::NodeType::Struct,
                "module" => crate::layout::NodeType::Module,
                _ => crate::layout::NodeType::Default,
            };
            layout.add_node_with_metadata(id, label, rect, nt);
        }

        // Add edges
        self.add_tree_edges(tree, &mut layout);

        Ok(layout)
    }

    /// Balloon tree layout (children arranged in balloon-like clusters)
    fn balloon_layout(&self, tree: &TreeNode) -> crate::Result<Layout> {
        let mut layout = Layout::new();
        let mut node_positions = HashMap::new();

        // Similar to radial but with balloon-like clustering
        let root_size = self.get_node_size(tree);
        let root_pos = Point::new(0.0, 0.0);
        node_positions.insert(tree.id.clone(), (root_pos, root_size, tree.label.clone(), tree.node_type.clone()));

        // Position children in balloon clusters
        if !tree.children.is_empty() {
            let cluster_radius = self.config.level_distance;
            self.position_balloon_children(&tree.children, root_pos, cluster_radius, &mut node_positions);
        }

        // Add nodes to layout
        for (id, (position, size, label, node_type)) in node_positions {
            let rect = Rect::new(Point::new(position.x - size.width / 2.0, position.y - size.height / 2.0), size);
            let nt = match node_type.as_str() {
                "function" => crate::layout::NodeType::Function,
                "struct" => crate::layout::NodeType::Struct,
                "module" => crate::layout::NodeType::Module,
                _ => crate::layout::NodeType::Default,
            };
            layout.add_node_with_metadata(id, label, rect, nt);
        }

        // Add edges
        self.add_tree_edges(tree, &mut layout);

        Ok(layout)
    }

    // Helper methods
    fn get_node_size(&self, node: &TreeNode) -> Size {
        node.size.unwrap_or(Size::new(self.config.node_width, self.config.node_height))
    }

    fn calculate_level_widths(&self, node: &TreeNode, level: usize, level_widths: &mut HashMap<usize, f64>) {
        let node_size = self.get_node_size(node);
        let current_width = level_widths.get(&level).unwrap_or(&0.0);
        level_widths.insert(level, current_width + node_size.width + self.config.sibling_distance);

        for child in &node.children {
            self.calculate_level_widths(child, level + 1, level_widths);
        }
    }

    fn position_layered_node(&self, node: &TreeNode, level: usize, _parent_x: f64, level_widths: &HashMap<usize, f64>, current_positions: &mut HashMap<usize, f64>, node_positions: &mut HashMap<String, (Point, Size, String, String)>) {
        let node_size = self.get_node_size(node);
        let level_width = level_widths.get(&level).unwrap_or(&0.0);
        let default_x = -level_width / 2.0;
        let current_x = current_positions.get(&level).unwrap_or(&default_x);

        let x = if level == 0 {
            0.0 // Root at center
        }
        else {
            *current_x + node_size.width / 2.0
        };

        let y = level as f64 * self.config.level_distance;

        node_positions.insert(node.id.clone(), (Point::new(x, y), node_size, node.label.clone(), node.node_type.clone()));
        current_positions.insert(level, current_x + node_size.width + self.config.sibling_distance);

        // Position children
        for child in &node.children {
            self.position_layered_node(child, level + 1, x, level_widths, current_positions, node_positions);
        }
    }

    fn position_radial_children(&self, children: &[TreeNode], center: Point, radius: f64, start_angle: f64, angle_span: f64, level: usize, node_positions: &mut HashMap<String, (Point, Size, String, String)>) {
        if children.is_empty() {
            return;
        }

        let angle_step = angle_span / children.len() as f64;

        for (i, child) in children.iter().enumerate() {
            let angle = start_angle + i as f64 * angle_step + angle_step / 2.0;
            let child_pos = Point::new(center.x + radius * angle.cos(), center.y + radius * angle.sin());

            let child_size = self.get_node_size(child);
            node_positions.insert(child.id.clone(), (child_pos, child_size, child.label.clone(), child.node_type.clone()));

            // Recursively position grandchildren
            if !child.children.is_empty() {
                let child_radius = radius + self.config.level_distance;
                let child_angle_span = angle_step * 0.8; // Reduce angle span for children
                self.position_radial_children(&child.children, child_pos, child_radius, angle - child_angle_span / 2.0, child_angle_span, level + 1, node_positions);
            }
        }
    }

    fn position_compact_node(&self, node: &TreeNode, x: f64, y: f64, level: usize) -> PositionedTreeNode {
        let size = self.get_node_size(node);
        let mut positioned_children = Vec::new();
        let mut child_x = x;

        for child in &node.children {
            let positioned_child = self.position_compact_node(child, child_x, y + self.config.level_distance, level + 1);
            child_x += positioned_child.subtree_width + self.config.sibling_distance;
            positioned_children.push(positioned_child);
        }

        let subtree_width = if positioned_children.is_empty() { size.width } else { positioned_children.iter().map(|c| c.subtree_width).sum::<f64>() + (positioned_children.len() - 1) as f64 * self.config.sibling_distance };

        // Center the node over its children
        let node_x = if positioned_children.is_empty() { x } else { x + subtree_width / 2.0 - size.width / 2.0 };

        PositionedTreeNode { id: node.id.clone(), label: node.label.clone(), node_type: node.node_type.clone(), position: Point::new(node_x, y), size, subtree_width, children: positioned_children }
    }

    fn position_balloon_children(&self, children: &[TreeNode], center: Point, radius: f64, node_positions: &mut HashMap<String, (Point, Size, String, String)>) {
        if children.is_empty() {
            return;
        }

        // Arrange children in a circle around the parent
        let angle_step = 2.0 * std::f64::consts::PI / children.len() as f64;

        for (i, child) in children.iter().enumerate() {
            let angle = i as f64 * angle_step;
            let child_pos = Point::new(center.x + radius * angle.cos(), center.y + radius * angle.sin());

            let child_size = self.get_node_size(child);
            node_positions.insert(child.id.clone(), (child_pos, child_size, child.label.clone(), child.node_type.clone()));

            // Recursively position grandchildren in smaller balloons
            if !child.children.is_empty() {
                let child_radius = radius * 0.6; // Smaller radius for child balloons
                self.position_balloon_children(&child.children, child_pos, child_radius, node_positions);
            }
        }
    }

    fn extract_positions(&self, positioned_node: &PositionedTreeNode, positions: &mut HashMap<String, (Point, Size, String, String)>) {
        positions.insert(positioned_node.id.clone(), (positioned_node.position, positioned_node.size, positioned_node.label.clone(), positioned_node.node_type.clone()));

        for child in &positioned_node.children {
            self.extract_positions(child, positions);
        }
    }

    /// Recursively add edges from the given node to all its descendants
    fn add_tree_edges(&self, node: &TreeNode, layout: &mut Layout) {
        for child in &node.children {
            let edge = Edge::new(node.id.clone(), child.id.clone());
            layout.add_edge(edge);

            // Recursively add edges for children
            self.add_tree_edges(child, layout);
        }
    }
}

/// Tree layout configuration
#[derive(Debug, Clone)]
pub struct TreeLayoutConfig {
    pub node_width: f64,
    pub node_height: f64,
    pub level_distance: f64,
    pub sibling_distance: f64,
    pub subtree_distance: f64,
}

impl Default for TreeLayoutConfig {
    fn default() -> Self {
        Self { node_width: 100.0, node_height: 40.0, level_distance: 80.0, sibling_distance: 20.0, subtree_distance: 40.0 }
    }
}

/// Tree layout algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TreeLayoutAlgorithm {
    Layered, // Traditional hierarchical layout
    Radial,  // Radial/circular layout
    Compact, // Space-efficient layout
    Balloon, // Balloon-style clustering
}

/// Positioned tree node (internal representation)
#[derive(Debug, Clone)]
struct PositionedTreeNode {
    id: String,
    label: String,
    node_type: String,
    position: Point,
    size: Size,
    subtree_width: f64,
    children: Vec<PositionedTreeNode>,
}

/// Tree renderer for generating visual output
pub struct TreeRenderer {
    config: TreeRenderConfig,
}

impl TreeRenderer {
    pub fn new() -> Self {
        Self { config: TreeRenderConfig::default() }
    }

    pub fn with_config(mut self, config: TreeRenderConfig) -> Self {
        self.config = config;
        self
    }

    // SVG rendering functionality would be implemented here
    // Currently disabled due to missing svg dependency
    // #[cfg(feature = "svg")]
    // pub fn render_svg(&self, layout: &Layout, tree: &TreeNode) -> crate::Result<String> {
    //     use svg::{
    //         Document,
    //         node::element::{Group, Line, Rectangle, Text},
    //     }
    //
    //     let mut document = Document::new().set("viewBox", format!("0 0 {} {}", layout.bounds.width(), layout.bounds.height()));
    //
    //     // Render nodes
    //     for (node_id, rect) in &layout.nodes {
    //         let node_group = Group::new().set("id", format!("node-{}", node_id));
    //
    //         // Node rectangle
    //         let node_rect = Rectangle::new()
    //             .set("x", rect.x())
    //             .set("y", rect.y())
    //             .set("width", rect.width())
    //             .set("height", rect.height())
    //             .set("fill", &self.config.node_fill_color)
    //             .set("stroke", &self.config.node_stroke_color)
    //             .set("stroke-width", self.config.node_stroke_width);
    //
    //         // Node label
    //         let label = self.get_node_label(tree, node_id);
    //         let text = Text::new()
    //             .set("x", rect.center().x)
    //             .set("y", rect.center().y)
    //             .set("text-anchor", "middle")
    //             .set("dominant-baseline", "middle")
    //             .set("font-family", &self.config.font_family)
    //             .set("font-size", self.config.font_size)
    //             .set("fill", &self.config.text_color)
    //             .add(svg::node::Text::new(label));
    //
    //         document = document.add(node_group.add(node_rect).add(text));
    //     }
    //
    //     // Render edges
    //     for edge in &layout.edges {
    //         if let (Some(from_rect), Some(to_rect)) = (layout.nodes.get(&edge.from), layout.nodes.get(&edge.to)) {
    //             let from_point = from_rect.center();
    //             let to_point = to_rect.center();
    //
    //             let line = Line::new()
    //                 .set("x1", from_point.x)
    //                 .set("y1", from_point.y)
    //                 .set("x2", to_point.x)
    //                 .set("y2", to_point.y)
    //                 .set("stroke", &self.config.edge_color)
    //                 .set("stroke-width", self.config.edge_width);
    //
    //             document = document.add(line);
    //         }
    //     }
    //
    //     Ok(document.to_string())
    // }

    #[allow(dead_code)]
    fn get_node_label(&self, tree: &TreeNode, node_id: &str) -> String {
        self.find_node_label(tree, node_id).unwrap_or_else(|| node_id.to_string())
    }

    #[allow(dead_code)]
    fn find_node_label(&self, node: &TreeNode, target_id: &str) -> Option<String> {
        if node.id == target_id {
            return Some(node.label.clone());
        }

        node.children.iter().find_map(|child| self.find_node_label(child, target_id))
    }
}

impl Default for TreeRenderer {
    fn default() -> Self {
        Self::new()
    }
}

/// Tree rendering configuration
#[derive(Debug, Clone)]
pub struct TreeRenderConfig {
    pub node_fill_color: String,
    pub node_stroke_color: String,
    pub node_stroke_width: f64,
    pub edge_color: String,
    pub edge_width: f64,
    pub text_color: String,
    pub font_family: String,
    pub font_size: f64,
}

impl Default for TreeRenderConfig {
    fn default() -> Self {
        Self {
            node_fill_color: "#ffffff".to_string(),
            node_stroke_color: "#000000".to_string(),
            node_stroke_width: 1.0,
            edge_color: "#666666".to_string(),
            edge_width: 1.0,
            text_color: "#000000".to_string(),
            font_family: "Arial, sans-serif".to_string(),
            font_size: 12.0,
        }
    }
}
