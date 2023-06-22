use oak_visualize::{
    geometry::Size,
    tree::{TreeLayout, TreeLayoutAlgorithm, TreeLayoutConfig, TreeNode, TreeRenderConfig, TreeRenderer},
};

#[test]
fn test_tree_node_creation() {
    let node = TreeNode::new("root".to_string(), "Root Node".to_string(), "root".to_string());

    assert_eq!(node.id, "root");
    assert_eq!(node.label, "Root Node");
    assert_eq!(node.node_type, "root");
    assert!(node.children.is_empty());
    assert!(node.attributes.is_empty());
    assert!(node.size.is_none());
    assert!(node.is_leaf());
}

#[test]
fn test_tree_node_with_children() {
    let node = TreeNode::new("root".to_string(), "Root Node".to_string(), "root".to_string()).with_child(TreeNode::new("child1".to_string(), "Child 1".to_string(), "child".to_string())).with_child(TreeNode::new(
        "child2".to_string(),
        "Child 2".to_string(),
        "child".to_string(),
    ));

    assert_eq!(node.children.len(), 2);
    assert_eq!(node.depth(), 2);
    assert_eq!(node.node_count(), 3);
    assert_eq!(node.leaf_count(), 2);
    assert!(!node.is_leaf());
}

#[test]
fn test_tree_node_with_multiple_children() {
    let children =
        vec![TreeNode::new("child1".to_string(), "Child 1".to_string(), "child".to_string()), TreeNode::new("child2".to_string(), "Child 2".to_string(), "child".to_string()), TreeNode::new("child3".to_string(), "Child 3".to_string(), "child".to_string())];

    let node = TreeNode::new("root".to_string(), "Root".to_string(), "root".to_string()).with_children(children);

    assert_eq!(node.children.len(), 3);
    assert_eq!(node.leaf_count(), 3);
}

#[test]
fn test_tree_node_attributes() {
    let node = TreeNode::new("test".to_string(), "Test".to_string(), "test".to_string()).with_attribute("key1".to_string(), "value1".to_string()).with_attribute("key2".to_string(), "value2".to_string());

    assert_eq!(node.attributes.len(), 2);
    assert_eq!(node.attributes.get("key1"), Some(&"value1".to_string()));
    assert_eq!(node.attributes.get("key2"), Some(&"value2".to_string()));
}

#[test]
fn test_tree_node_with_size() {
    let size = Size::new(100.0, 50.0);
    let node = TreeNode::new("test".to_string(), "Test".to_string(), "test".to_string()).with_size(size);

    assert_eq!(node.size, Some(size));
}

#[test]
fn test_tree_depth_calculation() {
    let deep_tree = TreeNode::new("root".to_string(), "Root".to_string(), "root".to_string()).with_child(
        TreeNode::new("level1".to_string(), "Level 1".to_string(), "node".to_string()).with_child(TreeNode::new("level2".to_string(), "Level 2".to_string(), "node".to_string()).with_child(TreeNode::new(
            "level3".to_string(),
            "Level 3".to_string(),
            "leaf".to_string(),
        ))),
    );

    assert_eq!(deep_tree.depth(), 4);
    assert_eq!(deep_tree.node_count(), 4);
    assert_eq!(deep_tree.leaf_count(), 1);
}

#[test]
fn test_layered_tree_layout() {
    let tree = TreeNode::new("root".to_string(), "Root".to_string(), "root".to_string()).with_child(TreeNode::new("child1".to_string(), "Child 1".to_string(), "child".to_string())).with_child(TreeNode::new(
        "child2".to_string(),
        "Child 2".to_string(),
        "child".to_string(),
    ));

    let layout_engine = TreeLayout::new().with_algorithm(TreeLayoutAlgorithm::Layered);
    let result = layout_engine.layout_tree(&tree);

    assert!(result.is_ok());
    let layout = result.unwrap();
    assert_eq!(layout.nodes.len(), 3);
    assert_eq!(layout.edges.len(), 2);

    // Check that all nodes are positioned
    assert!(layout.nodes.contains_key("root"));
    assert!(layout.nodes.contains_key("child1"));
    assert!(layout.nodes.contains_key("child2"));
}

#[test]
fn test_radial_tree_layout() {
    let tree = TreeNode::new("root".to_string(), "Root".to_string(), "root".to_string()).with_children(vec![
        TreeNode::new("child1".to_string(), "Child 1".to_string(), "child".to_string()),
        TreeNode::new("child2".to_string(), "Child 2".to_string(), "child".to_string()),
        TreeNode::new("child3".to_string(), "Child 3".to_string(), "child".to_string()),
    ]);

    let layout_engine = TreeLayout::new().with_algorithm(TreeLayoutAlgorithm::Radial);
    let result = layout_engine.layout_tree(&tree);

    assert!(result.is_ok());
    let layout = result.unwrap();
    assert_eq!(layout.nodes.len(), 4);
    assert_eq!(layout.edges.len(), 3);
}

#[test]
fn test_compact_tree_layout() {
    let tree = TreeNode::new("root".to_string(), "Root".to_string(), "root".to_string())
        .with_children(vec![TreeNode::new("child1".to_string(), "Child 1".to_string(), "child".to_string()), TreeNode::new("child2".to_string(), "Child 2".to_string(), "child".to_string())]);

    let layout_engine = TreeLayout::new().with_algorithm(TreeLayoutAlgorithm::Compact);
    let result = layout_engine.layout_tree(&tree);

    assert!(result.is_ok());
    let layout = result.unwrap();
    assert_eq!(layout.nodes.len(), 3);
    assert_eq!(layout.edges.len(), 2);
}

#[test]
fn test_balloon_tree_layout() {
    let tree = TreeNode::new("root".to_string(), "Root".to_string(), "root".to_string())
        .with_children(vec![TreeNode::new("child1".to_string(), "Child 1".to_string(), "child".to_string()), TreeNode::new("child2".to_string(), "Child 2".to_string(), "child".to_string())]);

    let layout_engine = TreeLayout::new().with_algorithm(TreeLayoutAlgorithm::Balloon);
    let result = layout_engine.layout_tree(&tree);

    assert!(result.is_ok());
    let layout = result.unwrap();
    assert_eq!(layout.nodes.len(), 3);
    assert_eq!(layout.edges.len(), 2);
}

#[test]
fn test_tree_layout_with_custom_config() {
    let config = TreeLayoutConfig { node_width: 150.0, node_height: 80.0, level_distance: 100.0, sibling_distance: 50.0, subtree_distance: 80.0 };

    let tree = TreeNode::new("root".to_string(), "Root".to_string(), "root".to_string()).with_child(TreeNode::new("child".to_string(), "Child".to_string(), "child".to_string()));

    let layout_engine = TreeLayout::new().with_algorithm(TreeLayoutAlgorithm::Layered).with_config(config);

    let result = layout_engine.layout_tree(&tree);
    assert!(result.is_ok());
}

#[test]
fn test_tree_layout_config_default() {
    let config = TreeLayoutConfig::default();
    assert!(config.node_width > 0.0);
    assert!(config.node_height > 0.0);
    assert!(config.level_distance > 0.0);
    assert!(config.sibling_distance > 0.0);
    assert!(config.subtree_distance > 0.0);
}

#[test]
fn test_tree_renderer_creation() {
    let _renderer = TreeRenderer::new();
    // Just test that it can be created without panicking

    let custom_config = TreeRenderConfig {
        node_fill_color: "#FF0000".to_string(),
        node_stroke_color: "#000000".to_string(),
        node_stroke_width: 2.0,
        edge_color: "#666666".to_string(),
        edge_width: 1.5,
        text_color: "#333333".to_string(),
        font_family: "Arial".to_string(),
        font_size: 14.0,
    };

    let _custom_renderer = TreeRenderer::new().with_config(custom_config);
    // Test that custom config can be applied
}

#[test]
fn test_tree_render_config_default() {
    let config = TreeRenderConfig::default();
    assert!(!config.node_fill_color.is_empty());
    assert!(!config.node_stroke_color.is_empty());
    assert!(config.node_stroke_width > 0.0);
    assert!(!config.edge_color.is_empty());
    assert!(config.edge_width > 0.0);
    assert!(!config.text_color.is_empty());
    assert!(!config.font_family.is_empty());
    assert!(config.font_size > 0.0);
}

#[test]
fn test_empty_tree_layout() {
    let tree = TreeNode::new("single".to_string(), "Single".to_string(), "single".to_string());

    let layout_engine = TreeLayout::new().with_algorithm(TreeLayoutAlgorithm::Layered);
    let result = layout_engine.layout_tree(&tree);

    assert!(result.is_ok());
    let layout = result.unwrap();
    assert_eq!(layout.nodes.len(), 1);
    assert_eq!(layout.edges.len(), 0);
}

#[test]
fn test_all_layout_algorithms() {
    let tree = TreeNode::new("root".to_string(), "Root".to_string(), "root".to_string()).with_child(TreeNode::new("child".to_string(), "Child".to_string(), "child".to_string()));

    let algorithms = vec![TreeLayoutAlgorithm::Layered, TreeLayoutAlgorithm::Radial, TreeLayoutAlgorithm::Compact, TreeLayoutAlgorithm::Balloon];

    for algorithm in algorithms {
        let layout_engine = TreeLayout::new().with_algorithm(algorithm);
        let result = layout_engine.layout_tree(&tree);
        assert!(result.is_ok(), "Layout failed for algorithm: {:?}", algorithm);

        let layout = result.unwrap();
        assert_eq!(layout.nodes.len(), 2);
        assert_eq!(layout.edges.len(), 1);
    }
}
