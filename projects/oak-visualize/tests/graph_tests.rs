use oak_visualize::graph::{Graph, GraphEdge, GraphLayout, GraphLayoutAlgorithm, GraphLayoutConfig, GraphNode};

#[test]
fn test_graph_creation() {
    let mut graph = Graph::new(true);

    graph.add_node(GraphNode::new("a".to_string(), "Node A".to_string()));
    graph.add_node(GraphNode::new("b".to_string(), "Node B".to_string()));
    graph.add_edge(GraphEdge::new("a".to_string(), "b".to_string()));

    assert_eq!(graph.nodes.len(), 2);
    assert_eq!(graph.edges.len(), 1);
    assert_eq!(graph.get_degree("a"), 1);
    assert_eq!(graph.get_degree("b"), 0);
}

#[test]
fn test_circular_layout() {
    let mut graph = Graph::new(false);

    for i in 0..5 {
        graph.add_node(GraphNode::new(format!("node{}", i), format!("Node {}", i)));
    }

    let layout_engine = GraphLayout::new().with_algorithm(GraphLayoutAlgorithm::Circular);
    let result = layout_engine.layout_graph(&graph);

    assert!(result.is_ok());
    let layout = result.unwrap();
    assert_eq!(layout.nodes.len(), 5);
}

#[test]
fn test_force_directed_layout() {
    let mut graph = Graph::new(true);

    graph.add_node(GraphNode::new("a".to_string(), "A".to_string()));
    graph.add_node(GraphNode::new("b".to_string(), "B".to_string()));
    graph.add_node(GraphNode::new("c".to_string(), "C".to_string()));
    graph.add_edge(GraphEdge::new("a".to_string(), "b".to_string()));
    graph.add_edge(GraphEdge::new("b".to_string(), "c".to_string()));

    let layout_engine = GraphLayout::new().with_algorithm(GraphLayoutAlgorithm::ForceDirected).with_config(GraphLayoutConfig { iterations: 10, ..Default::default() });

    let result = layout_engine.layout_graph(&graph);
    assert!(result.is_ok());

    let layout = result.unwrap();
    assert_eq!(layout.nodes.len(), 3);
    assert_eq!(layout.edges.len(), 2);
}

#[test]
fn test_graph_node_builder() {
    let node = GraphNode::new("test".to_string(), "Test Node".to_string()).with_type("function".to_string()).with_weight(2.5);

    assert_eq!(node.id, "test");
    assert_eq!(node.label, "Test Node");
    assert_eq!(node.node_type, "function");
    assert_eq!(node.weight, 2.5);
}

#[test]
fn test_graph_edge_builder() {
    let edge = GraphEdge::new("a".to_string(), "b".to_string()).with_label("calls".to_string()).with_type("dependency".to_string()).with_weight(1.5).undirected();

    assert_eq!(edge.from, "a");
    assert_eq!(edge.to, "b");
    assert_eq!(edge.label, Some("calls".to_string()));
    assert_eq!(edge.edge_type, "dependency");
    assert_eq!(edge.weight, 1.5);
    assert!(!edge.directed);
}

#[test]
fn test_graph_neighbors() {
    let mut graph = Graph::new(true);

    graph.add_node(GraphNode::new("a".to_string(), "A".to_string()));
    graph.add_node(GraphNode::new("b".to_string(), "B".to_string()));
    graph.add_node(GraphNode::new("c".to_string(), "C".to_string()));

    graph.add_edge(GraphEdge::new("a".to_string(), "b".to_string()));
    graph.add_edge(GraphEdge::new("a".to_string(), "c".to_string()));

    let neighbors = graph.get_neighbors("a");
    assert_eq!(neighbors.len(), 2);
    assert!(neighbors.contains(&"b"));
    assert!(neighbors.contains(&"c"));
}

#[test]
fn test_graph_connectivity() {
    let mut graph = Graph::new(false);

    // Connected graph
    graph.add_node(GraphNode::new("a".to_string(), "A".to_string()));
    graph.add_node(GraphNode::new("b".to_string(), "B".to_string()));
    graph.add_edge(GraphEdge::new("a".to_string(), "b".to_string()));

    assert!(graph.is_connected());

    // Disconnected graph
    graph.add_node(GraphNode::new("c".to_string(), "C".to_string()));
    assert!(!graph.is_connected());
}
