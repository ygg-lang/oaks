#![doc = "Layout algorithms for visualizing code structures"]

use crate::geometry::{Point, Rect, Size};
use std::collections::HashMap;

/// Layout configuration
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LayoutConfig {
    /// Default width for nodes.
    pub node_width: f64,
    /// Default height for nodes.
    pub node_height: f64,
    /// Horizontal spacing between nodes.
    pub horizontal_spacing: f64,
    /// Vertical spacing between nodes.
    pub vertical_spacing: f64,
    /// Margin around the layout.
    pub margin: f64,
    /// Padding inside the layout.
    pub padding: f64,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self { node_width: 120.0, node_height: 60.0, horizontal_spacing: 40.0, vertical_spacing: 30.0, margin: 20.0, padding: 10.0 }
    }
}

/// Positioned node in a layout
#[derive(Debug, Clone)]
pub struct PositionedNode {
    /// Unique identifier for the node.
    pub id: String,
    /// Display label for the node.
    pub label: String,
    /// Rectangle representing the node's position and size.
    pub rect: Rect,
    /// Type of the node for styling.
    pub node_type: NodeType,
}

/// Layout result containing positioned elements
#[derive(Debug, Clone)]
pub struct Layout {
    /// Bounding box of all elements in the layout.
    pub bounds: Rect,
    /// Map of node IDs to their positioned nodes.
    pub nodes: HashMap<String, PositionedNode>,
    /// List of edges connecting the nodes.
    pub edges: Vec<Edge>,
}

impl Layout {
    /// Creates a new empty layout.
    pub fn new() -> Self {
        Self { bounds: Rect::default(), nodes: HashMap::new(), edges: Vec::new() }
    }

    /// Adds a node to the layout at the specified rectangle.
    pub fn add_node(&mut self, id: String, rect: Rect) {
        let label = id.clone();
        self.nodes.insert(id.clone(), PositionedNode { id, label, rect, node_type: NodeType::Default });
        self.update_bounds()
    }

    /// Adds a node with explicit label and type.
    pub fn add_node_with_metadata(&mut self, id: String, label: String, rect: Rect, node_type: NodeType) {
        self.nodes.insert(id.clone(), PositionedNode { id, label, rect, node_type });
        self.update_bounds()
    }

    /// Adds an edge to the layout.
    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge)
    }

    fn update_bounds(&mut self) {
        if self.nodes.is_empty() {
            self.bounds = Rect::default();
            return;
        }

        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for node in self.nodes.values() {
            let rect = &node.rect;
            min_x = min_x.min(rect.min_x());
            min_y = min_y.min(rect.min_y());
            max_x = max_x.max(rect.max_x());
            max_y = max_y.max(rect.max_y());
        }

        self.bounds = Rect::from_xywh(min_x, min_y, max_x - min_x, max_y - min_y);
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self::new()
    }
}

/// Edge connecting two nodes
#[derive(Debug, Clone)]
pub struct Edge {
    /// ID of the source node.
    pub from: String,
    /// ID of the target node.
    pub to: String,
    /// Control points for the edge path.
    pub points: Vec<Point>,
    /// Optional label for the edge.
    pub label: Option<String>,
}

impl Edge {
    /// Creates a new edge between two nodes.
    pub fn new(from: String, to: String) -> Self {
        Self { from, to, points: Vec::new(), label: None }
    }

    /// Sets the label for the edge.
    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    /// Sets the control points for the edge.
    pub fn with_points(mut self, points: Vec<Point>) -> Self {
        self.points = points;
        self
    }
}

/// Layout engine trait
pub trait LayoutEngine {
    /// Computes the layout for a set of nodes and edges.
    fn layout(&self, nodes: &[LayoutNode], edges: &[LayoutEdge], config: &LayoutConfig) -> crate::Result<Layout>;
}

/// Node to be laid out
#[derive(Debug, Clone)]
pub struct LayoutNode {
    /// Unique identifier for the node.
    pub id: String,
    /// Preferred size of the node.
    pub size: Size,
    /// Display label for the node.
    pub label: String,
    /// Type of the node for styling.
    pub node_type: NodeType,
}

impl LayoutNode {
    /// Creates a new layout node with ID and label.
    pub fn new(id: String, label: String) -> Self {
        Self { id, label, size: Size::default(), node_type: NodeType::Default }
    }

    /// Sets the preferred size of the node.
    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Sets the type of the node.
    pub fn with_type(mut self, node_type: NodeType) -> Self {
        self.node_type = node_type;
        self
    }
}

/// Edge to be laid out
#[derive(Debug, Clone)]
pub struct LayoutEdge {
    /// ID of the source node.
    pub from: String,
    /// ID of the target node.
    pub to: String,
    /// Optional label for the edge.
    pub label: Option<String>,
    /// Type of the edge for styling.
    pub edge_type: EdgeType,
}

impl LayoutEdge {
    /// Creates a new layout edge between two nodes.
    pub fn new(from: String, to: String) -> Self {
        Self { from, to, label: None, edge_type: EdgeType::Default }
    }

    /// Sets the label for the edge.
    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    /// Sets the type of the edge.
    pub fn with_type(mut self, edge_type: EdgeType) -> Self {
        self.edge_type = edge_type;
        self
    }
}

/// Node type for styling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    /// Default node type.
    Default,
    /// Function or method.
    Function,
    /// Struct or class.
    Struct,
    /// Enum definition.
    Enum,
    /// Variable or field.
    Variable,
    /// Constant value.
    Constant,
    /// Module or namespace.
    Module,
}

/// Edge type for styling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeType {
    /// Default edge type.
    Default,
    /// General dependency.
    Dependency,
    /// Inheritance or implementation.
    Inheritance,
    /// Association between elements.
    Association,
    /// Composition relationship.
    Composition,
    /// Function or method call.
    Call,
}

/// Hierarchical layout engine (tree-like structures)
pub struct HierarchicalLayout {
    direction: LayoutDirection,
}

impl HierarchicalLayout {
    /// Creates a new hierarchical layout engine with the specified direction.
    pub fn new(direction: LayoutDirection) -> Self {
        Self { direction }
    }
}

impl LayoutEngine for HierarchicalLayout {
    /// Lays out nodes hierarchically in the specified direction.
    fn layout(&self, nodes: &[LayoutNode], edges: &[LayoutEdge], config: &LayoutConfig) -> crate::Result<Layout> {
        let mut layout = Layout::new();

        if nodes.is_empty() {
            return Ok(layout);
        }

        // Build hierarchy from edges
        let hierarchy = build_hierarchy(nodes, edges)?;

        // Layout nodes hierarchically
        let positioned_nodes = match self.direction {
            LayoutDirection::TopDown => layout_top_down(&hierarchy, config),
            LayoutDirection::LeftRight => layout_left_right(&hierarchy, config),
            LayoutDirection::BottomUp => layout_bottom_up(&hierarchy, config),
            LayoutDirection::RightLeft => layout_right_left(&hierarchy, config),
        };

        // Add nodes to layout
        for (id, rect) in positioned_nodes {
            layout.add_node(id, rect);
        }

        // Add edges with routing
        for edge in edges {
            if let (Some(from_node), Some(to_node)) = (layout.nodes.get(&edge.from), layout.nodes.get(&edge.to)) {
                let routed_edge = route_edge(&from_node.rect, &to_node.rect, &edge.from, &edge.to, edge.label.clone());
                layout.add_edge(routed_edge);
            }
        }

        Ok(layout)
    }
}

/// Force-directed layout engine
pub struct ForceDirectedLayout {
    iterations: usize,
    spring_strength: f64,
    repulsion_strength: f64,
    damping: f64,
}

impl ForceDirectedLayout {
    /// Creates a new force-directed layout engine with default parameters.
    pub fn new() -> Self {
        Self { iterations: 100, spring_strength: 0.1, repulsion_strength: 1000.0, damping: 0.9 }
    }

    /// Sets the number of simulation iterations.
    pub fn with_iterations(mut self, iterations: usize) -> Self {
        self.iterations = iterations;
        self
    }

    /// Sets the strength of the attractive spring forces.
    pub fn with_spring_strength(mut self, strength: f64) -> Self {
        self.spring_strength = strength;
        self
    }

    /// Sets the strength of the repulsive forces between nodes.
    pub fn with_repulsion_strength(mut self, strength: f64) -> Self {
        self.repulsion_strength = strength;
        self
    }
}

impl Default for ForceDirectedLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutEngine for ForceDirectedLayout {
    /// Lays out nodes using a force-directed simulation.
    fn layout(&self, nodes: &[LayoutNode], edges: &[LayoutEdge], config: &LayoutConfig) -> crate::Result<Layout> {
        let mut layout = Layout::new();

        if nodes.is_empty() {
            return Ok(layout);
        }

        // Initialize random positions
        let mut positions: HashMap<String, Point> = HashMap::new();
        let mut velocities: HashMap<String, Point> = HashMap::new();

        for (i, node) in nodes.iter().enumerate() {
            let angle = 2.0 * std::f64::consts::PI * i as f64 / nodes.len() as f64;
            let radius = 100.0;
            positions.insert(node.id.clone(), Point::new(radius * angle.cos(), radius * angle.sin()));
            velocities.insert(node.id.clone(), Point::origin());
        }

        // Run force-directed simulation
        for _ in 0..self.iterations {
            let mut forces: HashMap<String, Point> = HashMap::new();

            // Initialize forces
            for node in nodes {
                forces.insert(node.id.clone(), Point::origin());
            }

            // Repulsion forces
            for i in 0..nodes.len() {
                for j in (i + 1)..nodes.len() {
                    let node1 = &nodes[i];
                    let node2 = &nodes[j];

                    if let (Some(pos1), Some(pos2)) = (positions.get(&node1.id), positions.get(&node2.id)) {
                        let diff = *pos1 - *pos2;
                        let distance = pos1.distance_to(pos2).max(1.0);
                        let force_magnitude = self.repulsion_strength / (distance * distance);
                        let force_direction = Point::new(diff.x / distance, diff.y / distance);
                        let force = Point::new(force_direction.x * force_magnitude, force_direction.y * force_magnitude);

                        *forces.get_mut(&node1.id).unwrap() = *forces.get(&node1.id).unwrap() + force;
                        *forces.get_mut(&node2.id).unwrap() = *forces.get(&node2.id).unwrap() - force
                    }
                }
            }

            // Attraction forces (springs)
            for edge in edges {
                if let (Some(pos1), Some(pos2)) = (positions.get(&edge.from), positions.get(&edge.to)) {
                    let diff = *pos2 - *pos1;
                    let distance = pos1.distance_to(pos2);
                    let ideal_length = config.horizontal_spacing;
                    let force_magnitude = self.spring_strength * (distance - ideal_length);
                    let force_direction = Point::new(diff.x / distance, diff.y / distance);
                    let force = Point::new(force_direction.x * force_magnitude, force_direction.y * force_magnitude);

                    *forces.get_mut(&edge.from).unwrap() = *forces.get(&edge.from).unwrap() + force;
                    *forces.get_mut(&edge.to).unwrap() = *forces.get(&edge.to).unwrap() - force
                }
            }

            // Update positions
            for node in nodes {
                if let (Some(force), Some(velocity), Some(position)) = (forces.get(&node.id), velocities.get_mut(&node.id), positions.get_mut(&node.id)) {
                    *velocity = Point::new(velocity.x * self.damping + force.x, velocity.y * self.damping + force.y);
                    *position = *position + *velocity
                }
            }
        }

        // Convert positions to rectangles
        for node in nodes {
            if let Some(position) = positions.get(&node.id) {
                let rect = Rect::new(Point::new(position.x - node.size.width / 2.0, position.y - node.size.height / 2.0), node.size);
                layout.add_node(node.id.clone(), rect)
            }
        }

        // Add edges
        for edge in edges {
            if let (Some(from_node), Some(to_node)) = (layout.nodes.get(&edge.from), layout.nodes.get(&edge.to)) {
                let routed_edge = route_edge(&from_node.rect, &to_node.rect, &edge.from, &edge.to, edge.label.clone());
                layout.add_edge(routed_edge)
            }
        }

        Ok(layout)
    }
}

/// Layout direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutDirection {
    /// Top to bottom.
    TopDown,
    /// Bottom to top.
    BottomUp,
    /// Left to right.
    LeftRight,
    /// Right to left.
    RightLeft,
}

/// Hierarchy node for tree layouts
#[derive(Debug, Clone)]
struct HierarchyNode {
    id: String,
    children: Vec<HierarchyNode>,
    size: Size,
}

// Helper functions
fn build_hierarchy(nodes: &[LayoutNode], edges: &[LayoutEdge]) -> crate::Result<Vec<HierarchyNode>> {
    // Simple implementation - find roots and build tree
    let mut children_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut has_parent: std::collections::HashSet<String> = std::collections::HashSet::new();

    for edge in edges {
        children_map.entry(edge.from.clone()).or_default().push(edge.to.clone());
        has_parent.insert(edge.to.clone());
    }

    let mut roots = Vec::new();
    for node in nodes {
        if !has_parent.contains(&node.id) {
            roots.push(build_hierarchy_node(&node.id, nodes, &children_map))
        }
    }

    Ok(roots)
}

fn build_hierarchy_node(id: &str, nodes: &[LayoutNode], children_map: &HashMap<String, Vec<String>>) -> HierarchyNode {
    let node = nodes.iter().find(|n| n.id == id).unwrap();
    let children = children_map.get(id).map(|child_ids| child_ids.iter().map(|child_id| build_hierarchy_node(child_id, nodes, children_map)).collect()).unwrap_or_default();

    HierarchyNode { id: id.to_string(), children, size: node.size }
}

fn layout_top_down(hierarchy: &[HierarchyNode], config: &LayoutConfig) -> HashMap<String, Rect> {
    let mut positions = HashMap::new();
    let mut current_y = config.margin;

    for root in hierarchy {
        layout_node_top_down(root, config.margin, &mut current_y, config, &mut positions);
        current_y += config.vertical_spacing
    }

    positions
}

fn layout_node_top_down(node: &HierarchyNode, x: f64, y: &mut f64, config: &LayoutConfig, positions: &mut HashMap<String, Rect>) {
    let rect = Rect::new(Point::new(x, *y), node.size);
    positions.insert(node.id.clone(), rect);

    *y += node.size.height + config.vertical_spacing;

    let mut child_x = x + config.horizontal_spacing;
    for child in &node.children {
        layout_node_top_down(child, child_x, y, config, positions);
        child_x += child.size.width + config.horizontal_spacing
    }
}

fn layout_left_right(hierarchy: &[HierarchyNode], config: &LayoutConfig) -> HashMap<String, Rect> {
    let mut positions = HashMap::new();
    let mut current_x = config.margin;

    for root in hierarchy {
        layout_node_left_right(root, &mut current_x, config.margin, config, &mut positions);
        current_x += config.horizontal_spacing
    }

    positions
}

fn layout_node_left_right(node: &HierarchyNode, x: &mut f64, y: f64, config: &LayoutConfig, positions: &mut HashMap<String, Rect>) {
    let rect = Rect::new(Point::new(*x, y), node.size);
    positions.insert(node.id.clone(), rect);

    *x += node.size.width + config.horizontal_spacing;

    let mut child_y = y + config.vertical_spacing;
    for child in &node.children {
        layout_node_left_right(child, x, child_y, config, positions);
        child_y += child.size.height + config.vertical_spacing
    }
}

fn layout_bottom_up(hierarchy: &[HierarchyNode], config: &LayoutConfig) -> HashMap<String, Rect> {
    // Similar to top_down but reversed
    layout_top_down(hierarchy, config)
}

fn layout_right_left(hierarchy: &[HierarchyNode], config: &LayoutConfig) -> HashMap<String, Rect> {
    // Similar to left_right but reversed
    layout_left_right(hierarchy, config)
}

fn route_edge(from_rect: &Rect, to_rect: &Rect, from_id: &str, to_id: &str, label: Option<String>) -> Edge {
    // Simple straight line routing
    let from_center = from_rect.center();
    let to_center = to_rect.center();

    Edge { from: from_id.to_string(), to: to_id.to_string(), points: vec![from_center, to_center], label }
}
