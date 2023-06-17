#![doc = "Graph layout algorithms for dependency and relationship visualization"]

use crate::{
    geometry::{Point, Rect, Size},
    layout::{Edge, Layout},
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Graph node for visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub label: String,
    pub node_type: String,
    pub size: Option<Size>,
    pub attributes: HashMap<String, String>,
    pub weight: f64,
}

impl GraphNode {
    pub fn new(id: String, label: String) -> Self {
        Self { id, label, node_type: "default".to_string(), size: None, attributes: HashMap::new(), weight: 1.0 }
    }

    pub fn with_type(mut self, node_type: String) -> Self {
        self.node_type = node_type;
        self
    }

    pub fn with_size(mut self, size: Size) -> Self {
        self.size = Some(size);
        self
    }

    pub fn with_attribute(mut self, key: String, value: String) -> Self {
        self.attributes.insert(key, value);
        self
    }

    pub fn with_weight(mut self, weight: f64) -> Self {
        self.weight = weight;
        self
    }
}

/// Graph edge for visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub from: String,
    pub to: String,
    pub label: Option<String>,
    pub edge_type: String,
    pub weight: f64,
    pub directed: bool,
    pub attributes: HashMap<String, String>,
}

impl GraphEdge {
    pub fn new(from: String, to: String) -> Self {
        Self {
            from,
            to,
            label: None,
            edge_type: "default".to_string(),
            weight: 1.0,
            directed: true,
            attributes: HashMap::new(),
        }
    }

    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    pub fn with_type(mut self, edge_type: String) -> Self {
        self.edge_type = edge_type;
        self
    }

    pub fn with_weight(mut self, weight: f64) -> Self {
        self.weight = weight;
        self
    }

    pub fn undirected(mut self) -> Self {
        self.directed = false;
        self
    }

    pub fn with_attribute(mut self, key: String, value: String) -> Self {
        self.attributes.insert(key, value);
        self
    }
}

/// Graph representation
#[derive(Debug, Clone)]
pub struct Graph {
    pub nodes: HashMap<String, GraphNode>,
    pub edges: Vec<GraphEdge>,
    pub directed: bool,
}

impl Graph {
    pub fn new(directed: bool) -> Self {
        Self { nodes: HashMap::new(), edges: Vec::new(), directed }
    }

    pub fn add_node(&mut self, node: GraphNode) {
        self.nodes.insert(node.id.clone(), node);
    }

    pub fn add_edge(&mut self, edge: GraphEdge) {
        self.edges.push(edge);
    }

    pub fn get_neighbors(&self, node_id: &str) -> Vec<&str> {
        let mut neighbors = Vec::new();

        for edge in &self.edges {
            if edge.from == node_id {
                neighbors.push(edge.to.as_str());
            }
            if !self.directed && edge.to == node_id {
                neighbors.push(edge.from.as_str());
            }
        }

        neighbors
    }

    pub fn get_degree(&self, node_id: &str) -> usize {
        self.get_neighbors(node_id).len()
    }

    pub fn is_connected(&self) -> bool {
        if self.nodes.is_empty() {
            return true;
        }

        let start_node = self.nodes.keys().next().unwrap();
        let mut visited = HashSet::new();
        let mut stack = vec![start_node.as_str()];

        while let Some(node) = stack.pop() {
            if visited.contains(node) {
                continue;
            }

            visited.insert(node);

            for neighbor in self.get_neighbors(node) {
                if !visited.contains(neighbor) {
                    stack.push(neighbor);
                }
            }
        }

        visited.len() == self.nodes.len()
    }

    pub fn find_cycles(&self) -> Vec<Vec<String>> {
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut path = Vec::new();

        for node_id in self.nodes.keys() {
            if !visited.contains(node_id) {
                self.dfs_cycles(node_id, &mut visited, &mut rec_stack, &mut path, &mut cycles);
            }
        }

        cycles
    }

    fn dfs_cycles(
        &self,
        node: &str,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
        cycles: &mut Vec<Vec<String>>,
    ) {
        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());
        path.push(node.to_string());

        for neighbor in self.get_neighbors(node) {
            if !visited.contains(neighbor) {
                self.dfs_cycles(neighbor, visited, rec_stack, path, cycles);
            }
            else if rec_stack.contains(neighbor) {
                // Found a cycle
                if let Some(cycle_start) = path.iter().position(|n| n == neighbor) {
                    cycles.push(path[cycle_start..].to_vec());
                }
            }
        }

        rec_stack.remove(node);
        path.pop();
    }
}

/// Graph layout engine
pub struct GraphLayout {
    algorithm: GraphLayoutAlgorithm,
    config: GraphLayoutConfig,
}

impl GraphLayout {
    pub fn new(algorithm: GraphLayoutAlgorithm) -> Self {
        Self { algorithm, config: GraphLayoutConfig::default() }
    }

    pub fn with_config(mut self, config: GraphLayoutConfig) -> Self {
        self.config = config;
        self
    }

    pub fn layout_graph(&self, graph: &Graph) -> crate::Result<Layout> {
        match self.algorithm {
            GraphLayoutAlgorithm::ForceDirected => self.force_directed_layout(graph),
            GraphLayoutAlgorithm::Circular => self.circular_layout(graph),
            GraphLayoutAlgorithm::Hierarchical => self.hierarchical_layout(graph),
            GraphLayoutAlgorithm::Grid => self.grid_layout(graph),
            GraphLayoutAlgorithm::Organic => self.organic_layout(graph),
        }
    }

    /// Force-directed layout using spring-mass model
    fn force_directed_layout(&self, graph: &Graph) -> crate::Result<Layout> {
        let mut layout = Layout::new();

        if graph.nodes.is_empty() {
            return Ok(layout);
        }

        let mut positions: HashMap<String, Point> = HashMap::new();
        let mut velocities: HashMap<String, Point> = HashMap::new();

        // Initialize random positions
        for (i, node_id) in graph.nodes.keys().enumerate() {
            let angle = 2.0 * std::f64::consts::PI * i as f64 / graph.nodes.len() as f64;
            let radius = 100.0;
            positions.insert(node_id.clone(), Point::new(radius * angle.cos(), radius * angle.sin()));
            velocities.insert(node_id.clone(), Point::origin());
        }

        // Run simulation
        for _ in 0..self.config.iterations {
            let mut forces: HashMap<String, Point> = HashMap::new();

            // Initialize forces
            for node_id in graph.nodes.keys() {
                forces.insert(node_id.clone(), Point::origin());
            }

            // Repulsion forces between all nodes
            let node_ids: Vec<_> = graph.nodes.keys().collect();
            for i in 0..node_ids.len() {
                for j in (i + 1)..node_ids.len() {
                    let node1_id = node_ids[i];
                    let node2_id = node_ids[j];

                    if let (Some(pos1), Some(pos2)) = (positions.get(node1_id), positions.get(node2_id)) {
                        let diff = *pos1 - *pos2;
                        let distance = pos1.distance_to(pos2).max(1.0);
                        let force_magnitude = self.config.repulsion_strength / (distance * distance);
                        let force_direction = Point::new(diff.x / distance, diff.y / distance);
                        let force = Point::new(force_direction.x * force_magnitude, force_direction.y * force_magnitude);

                        *forces.get_mut(node1_id).unwrap() = *forces.get(node1_id).unwrap() + force;
                        *forces.get_mut(node2_id).unwrap() = *forces.get(node2_id).unwrap() - force;
                    }
                }
            }

            // Attraction forces along edges
            for edge in &graph.edges {
                if let (Some(pos1), Some(pos2)) = (positions.get(&edge.from), positions.get(&edge.to)) {
                    let diff = *pos2 - *pos1;
                    let distance = pos1.distance_to(pos2);
                    let ideal_length = self.config.ideal_edge_length;
                    let force_magnitude = self.config.spring_strength * (distance - ideal_length) * edge.weight;

                    if distance > 0.0 {
                        let force_direction = Point::new(diff.x / distance, diff.y / distance);
                        let force = Point::new(force_direction.x * force_magnitude, force_direction.y * force_magnitude);

                        *forces.get_mut(&edge.from).unwrap() = *forces.get(&edge.from).unwrap() + force;
                        *forces.get_mut(&edge.to).unwrap() = *forces.get(&edge.to).unwrap() - force;
                    }
                }
            }

            // Update positions
            for node_id in graph.nodes.keys() {
                if let (Some(force), Some(velocity), Some(position)) =
                    (forces.get(node_id), velocities.get_mut(node_id), positions.get_mut(node_id))
                {
                    *velocity =
                        Point::new(velocity.x * self.config.damping + force.x, velocity.y * self.config.damping + force.y);

                    // Limit velocity
                    let speed = (velocity.x * velocity.x + velocity.y * velocity.y).sqrt();
                    if speed > self.config.max_velocity {
                        let scale = self.config.max_velocity / speed;
                        velocity.x *= scale;
                        velocity.y *= scale;
                    }

                    *position = *position + *velocity;
                }
            }
        }

        // Convert positions to layout
        for (node_id, node) in &graph.nodes {
            if let Some(position) = positions.get(node_id) {
                let size = node.size.unwrap_or(Size::new(self.config.node_width, self.config.node_height));
                let rect = Rect::new(Point::new(position.x - size.width / 2.0, position.y - size.height / 2.0), size);
                layout.add_node(node_id.clone(), rect);
            }
        }

        // Add edges
        for edge in &graph.edges {
            if let (Some(from_rect), Some(to_rect)) = (layout.nodes.get(&edge.from), layout.nodes.get(&edge.to)) {
                let layout_edge =
                    Edge::new(edge.from.clone(), edge.to.clone()).with_points(vec![from_rect.center(), to_rect.center()]);
                layout.add_edge(layout_edge);
            }
        }

        Ok(layout)
    }

    /// Circular layout - arrange nodes in a circle
    fn circular_layout(&self, graph: &Graph) -> crate::Result<Layout> {
        let mut layout = Layout::new();

        if graph.nodes.is_empty() {
            return Ok(layout);
        }

        let node_count = graph.nodes.len();
        let radius = self.config.circle_radius;
        let angle_step = 2.0 * std::f64::consts::PI / node_count as f64;

        for (i, (node_id, node)) in graph.nodes.iter().enumerate() {
            let angle = i as f64 * angle_step;
            let position = Point::new(radius * angle.cos(), radius * angle.sin());
            let size = node.size.unwrap_or(Size::new(self.config.node_width, self.config.node_height));
            let rect = Rect::new(Point::new(position.x - size.width / 2.0, position.y - size.height / 2.0), size);
            layout.add_node(node_id.clone(), rect);
        }

        // Add edges
        for edge in &graph.edges {
            if let (Some(from_rect), Some(to_rect)) = (layout.nodes.get(&edge.from), layout.nodes.get(&edge.to)) {
                let layout_edge =
                    Edge::new(edge.from.clone(), edge.to.clone()).with_points(vec![from_rect.center(), to_rect.center()]);
                layout.add_edge(layout_edge);
            }
        }

        Ok(layout)
    }

    /// Hierarchical layout - arrange nodes in layers
    fn hierarchical_layout(&self, graph: &Graph) -> crate::Result<Layout> {
        let mut layout = Layout::new();

        if graph.nodes.is_empty() {
            return Ok(layout);
        }

        // Perform topological sort to determine layers
        let layers = self.topological_layers(graph)?;

        // Position nodes layer by layer
        for (layer_index, layer) in layers.iter().enumerate() {
            let y = layer_index as f64 * self.config.layer_distance;
            let layer_width = layer.len() as f64 * (self.config.node_width + self.config.node_spacing);
            let start_x = -layer_width / 2.0;

            for (node_index, node_id) in layer.iter().enumerate() {
                let x = start_x + node_index as f64 * (self.config.node_width + self.config.node_spacing);
                let node = &graph.nodes[node_id];
                let size = node.size.unwrap_or(Size::new(self.config.node_width, self.config.node_height));
                let rect = Rect::new(Point::new(x, y), size);
                layout.add_node(node_id.clone(), rect);
            }
        }

        // Add edges
        for edge in &graph.edges {
            if let (Some(from_rect), Some(to_rect)) = (layout.nodes.get(&edge.from), layout.nodes.get(&edge.to)) {
                let layout_edge =
                    Edge::new(edge.from.clone(), edge.to.clone()).with_points(vec![from_rect.center(), to_rect.center()]);
                layout.add_edge(layout_edge);
            }
        }

        Ok(layout)
    }

    /// Grid layout - arrange nodes in a regular grid
    fn grid_layout(&self, graph: &Graph) -> crate::Result<Layout> {
        let mut layout = Layout::new();

        if graph.nodes.is_empty() {
            return Ok(layout);
        }

        let node_count = graph.nodes.len();
        let cols = (node_count as f64).sqrt().ceil() as usize;
        let _rows = (node_count + cols - 1) / cols;

        for (i, (node_id, node)) in graph.nodes.iter().enumerate() {
            let row = i / cols;
            let col = i % cols;
            let x = col as f64 * (self.config.node_width + self.config.node_spacing);
            let y = row as f64 * (self.config.node_height + self.config.node_spacing);
            let size = node.size.unwrap_or(Size::new(self.config.node_width, self.config.node_height));
            let rect = Rect::new(Point::new(x, y), size);
            layout.add_node(node_id.clone(), rect);
        }

        // Add edges
        for edge in &graph.edges {
            if let (Some(from_rect), Some(to_rect)) = (layout.nodes.get(&edge.from), layout.nodes.get(&edge.to)) {
                let layout_edge =
                    Edge::new(edge.from.clone(), edge.to.clone()).with_points(vec![from_rect.center(), to_rect.center()]);
                layout.add_edge(layout_edge);
            }
        }

        Ok(layout)
    }

    /// Organic layout - natural-looking arrangement
    fn organic_layout(&self, graph: &Graph) -> crate::Result<Layout> {
        // For now, use force-directed with different parameters
        let mut organic_config = self.config.clone();
        organic_config.spring_strength *= 0.5;
        organic_config.repulsion_strength *= 1.5;
        organic_config.damping = 0.95;

        let organic_layout = GraphLayout::new(GraphLayoutAlgorithm::ForceDirected).with_config(organic_config);

        organic_layout.force_directed_layout(graph)
    }

    /// Perform topological sort to determine node layers
    fn topological_layers(&self, graph: &Graph) -> crate::Result<Vec<Vec<String>>> {
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut layers = Vec::new();

        // Initialize in-degrees
        for node_id in graph.nodes.keys() {
            in_degree.insert(node_id.clone(), 0);
        }

        // Calculate in-degrees
        for edge in &graph.edges {
            *in_degree.get_mut(&edge.to).unwrap() += 1;
        }

        // Process layers
        while !in_degree.is_empty() {
            let mut current_layer = Vec::new();

            // Find nodes with in-degree 0
            let zero_in_degree: Vec<_> =
                in_degree.iter().filter(|&(_, &degree)| degree == 0).map(|(id, _)| id.clone()).collect();

            if zero_in_degree.is_empty() {
                // Cycle detected - break it by selecting node with minimum in-degree
                if let Some((min_node, _)) = in_degree.iter().min_by_key(|&(_, &degree)| degree) {
                    current_layer.push(min_node.clone());
                }
            }
            else {
                current_layer = zero_in_degree;
            }

            // Remove processed nodes and update in-degrees
            for node_id in &current_layer {
                in_degree.remove(node_id);

                // Decrease in-degree of neighbors
                for edge in &graph.edges {
                    if edge.from == *node_id {
                        if let Some(degree) = in_degree.get_mut(&edge.to) {
                            *degree = degree.saturating_sub(1);
                        }
                    }
                }
            }

            layers.push(current_layer);
        }

        Ok(layers)
    }
}

/// Graph layout configuration
#[derive(Debug, Clone)]
pub struct GraphLayoutConfig {
    pub node_width: f64,
    pub node_height: f64,
    pub node_spacing: f64,
    pub layer_distance: f64,
    pub circle_radius: f64,
    pub iterations: usize,
    pub spring_strength: f64,
    pub repulsion_strength: f64,
    pub damping: f64,
    pub max_velocity: f64,
    pub ideal_edge_length: f64,
}

impl Default for GraphLayoutConfig {
    fn default() -> Self {
        Self {
            node_width: 80.0,
            node_height: 40.0,
            node_spacing: 20.0,
            layer_distance: 100.0,
            circle_radius: 200.0,
            iterations: 100,
            spring_strength: 0.1,
            repulsion_strength: 1000.0,
            damping: 0.9,
            max_velocity: 10.0,
            ideal_edge_length: 100.0,
        }
    }
}

/// Graph layout algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphLayoutAlgorithm {
    ForceDirected, // Spring-mass model
    Circular,      // Circular arrangement
    Hierarchical,  // Layered arrangement
    Grid,          // Regular grid
    Organic,       // Natural-looking
}
