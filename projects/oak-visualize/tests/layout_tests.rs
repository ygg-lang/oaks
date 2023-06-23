use oak_visualize::{
    geometry::Size,
    layout::{EdgeType, ForceDirectedLayout, HierarchicalLayout, Layout, LayoutConfig, LayoutDirection, LayoutEdge, LayoutEngine, LayoutNode, NodeType},
};

#[test]
fn test_layout_config_default() {
    let config = LayoutConfig::default();
    assert!(config.node_width > 0.0);
    assert!(config.node_height > 0.0);
    assert!(config.horizontal_spacing > 0.0);
    assert!(config.vertical_spacing > 0.0);
    assert!(config.margin >= 0.0);
    assert!(config.padding >= 0.0)
}

#[test]
fn test_layout_creation() {
    let mut layout = Layout::new();
    assert!(layout.nodes.is_empty());
    assert!(layout.edges.is_empty());

    let rect = oak_visualize::geometry::Rect::from_xywh(0.0, 0.0, 100.0, 60.0);
    layout.add_node("test".to_string(), rect);
    assert_eq!(layout.nodes.len(), 1);
    assert!(layout.nodes.contains_key("test"))
}

#[test]
fn test_layout_node_creation() {
    let node = LayoutNode::new("test_id".to_string(), "Test Label".to_string());
    assert_eq!(node.id, "test_id");
    assert_eq!(node.label, "Test Label");
    assert_eq!(node.node_type, NodeType::Default);

    let sized_node = node.with_size(Size::new(200.0, 100.0));
    assert_eq!(sized_node.size.width, 200.0);
    assert_eq!(sized_node.size.height, 100.0);

    let typed_node = sized_node.with_type(NodeType::Function);
    assert_eq!(typed_node.node_type, NodeType::Function)
}

#[test]
fn test_layout_edge_creation() {
    let edge = LayoutEdge::new("from".to_string(), "to".to_string());
    assert_eq!(edge.from, "from");
    assert_eq!(edge.to, "to");
    assert_eq!(edge.edge_type, EdgeType::Default);
    assert!(edge.label.is_none());

    let labeled_edge = edge.with_label("test label".to_string());
    assert_eq!(labeled_edge.label, Some("test label".to_string()));

    let typed_edge = labeled_edge.with_type(EdgeType::Dependency);
    assert_eq!(typed_edge.edge_type, EdgeType::Dependency)
}

#[test]
fn test_hierarchical_layout() {
    let nodes = vec![LayoutNode::new("root".to_string(), "Root".to_string()), LayoutNode::new("child1".to_string(), "Child 1".to_string()), LayoutNode::new("child2".to_string(), "Child 2".to_string())];

    let edges = vec![LayoutEdge::new("root".to_string(), "child1".to_string()), LayoutEdge::new("root".to_string(), "child2".to_string())];

    let layout_engine = HierarchicalLayout::new(LayoutDirection::TopDown);
    let config = LayoutConfig::default();

    let result = layout_engine.layout(&nodes, &edges, &config);
    assert!(result.is_ok());

    let layout = result.unwrap();
    assert_eq!(layout.nodes.len(), 3);
    assert!(layout.nodes.contains_key("root"));
    assert!(layout.nodes.contains_key("child1"));
    assert!(layout.nodes.contains_key("child2"))
}

#[test]
fn test_hierarchical_layout_directions() {
    let nodes = vec![LayoutNode::new("a".to_string(), "A".to_string()), LayoutNode::new("b".to_string(), "B".to_string())];

    let edges = vec![LayoutEdge::new("a".to_string(), "b".to_string())];

    let config = LayoutConfig::default();

    // Test different layout directions
    let directions = vec![LayoutDirection::TopDown, LayoutDirection::BottomUp, LayoutDirection::LeftRight, LayoutDirection::RightLeft];

    for direction in directions {
        let layout_engine = HierarchicalLayout::new(direction);
        let result = layout_engine.layout(&nodes, &edges, &config);
        assert!(result.is_ok(), "Layout failed for direction: {:?}", direction);

        let layout = result.unwrap();
        assert_eq!(layout.nodes.len(), 2)
    }
}

#[test]
fn test_force_directed_layout() {
    let nodes = vec![LayoutNode::new("a".to_string(), "A".to_string()), LayoutNode::new("b".to_string(), "B".to_string()), LayoutNode::new("c".to_string(), "C".to_string())];

    let edges = vec![LayoutEdge::new("a".to_string(), "b".to_string()), LayoutEdge::new("b".to_string(), "c".to_string())];

    let layout_engine = ForceDirectedLayout::new().with_iterations(10);
    let config = LayoutConfig::default();

    let result = layout_engine.layout(&nodes, &edges, &config);
    assert!(result.is_ok());

    let layout = result.unwrap();
    assert_eq!(layout.nodes.len(), 3);
    assert!(layout.nodes.contains_key("a"));
    assert!(layout.nodes.contains_key("b"));
    assert!(layout.nodes.contains_key("c"))
}

#[test]
fn test_force_directed_layout_configuration() {
    let layout_engine = ForceDirectedLayout::new().with_iterations(50).with_spring_strength(0.5).with_repulsion_strength(1000.0);

    let nodes = vec![LayoutNode::new("node1".to_string(), "Node 1".to_string()), LayoutNode::new("node2".to_string(), "Node 2".to_string())];

    let edges = vec![LayoutEdge::new("node1".to_string(), "node2".to_string())];

    let config = LayoutConfig::default();
    let result = layout_engine.layout(&nodes, &edges, &config);
    assert!(result.is_ok())
}

#[test]
fn test_empty_layout() {
    let nodes = vec![];
    let edges = vec![];
    let config = LayoutConfig::default();

    let hierarchical = HierarchicalLayout::new(LayoutDirection::TopDown);
    let result = hierarchical.layout(&nodes, &edges, &config);
    assert!(result.is_ok());

    let layout = result.unwrap();
    assert!(layout.nodes.is_empty());
    assert!(layout.edges.is_empty())
}

#[test]
fn test_single_node_layout() {
    let nodes = vec![LayoutNode::new("single".to_string(), "Single Node".to_string())];
    let edges = vec![];
    let config = LayoutConfig::default();

    let hierarchical = HierarchicalLayout::new(LayoutDirection::TopDown);
    let result = hierarchical.layout(&nodes, &edges, &config);
    assert!(result.is_ok());

    let layout = result.unwrap();
    assert_eq!(layout.nodes.len(), 1);
    assert!(layout.nodes.contains_key("single"))
}

#[test]
fn test_node_types() {
    let node_types = vec![NodeType::Default, NodeType::Function, NodeType::Struct, NodeType::Enum, NodeType::Variable, NodeType::Constant, NodeType::Module];

    for (i, node_type) in node_types.iter().enumerate() {
        let node = LayoutNode::new(format!("node_{}", i), format!("Node {}", i)).with_type(*node_type);
        assert_eq!(node.node_type, *node_type)
    }
}

#[test]
fn test_edge_types() {
    let edge_types = vec![EdgeType::Default, EdgeType::Dependency, EdgeType::Inheritance, EdgeType::Association, EdgeType::Composition, EdgeType::Call];

    for (i, edge_type) in edge_types.iter().enumerate() {
        let edge = LayoutEdge::new(format!("from_{}", i), format!("to_{}", i)).with_type(*edge_type);
        assert_eq!(edge.edge_type, *edge_type)
    }
}
