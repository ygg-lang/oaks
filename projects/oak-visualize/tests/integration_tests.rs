use oak_visualize::{
    geometry::{Point, Rect, Size},
    graph::{Graph, GraphEdge, GraphLayout, GraphLayoutAlgorithm, GraphNode},
    layout::{Edge, Layout},
    render::{ExportFormat, LayoutExporter, SvgRenderer},
    theme::VisualizationTheme,
    tree::{TreeLayout, TreeLayoutAlgorithm, TreeNode},
};

#[test]
fn test_complete_graph_visualization_pipeline() {
    // 1. Create graph data structure
    let mut graph = Graph::new(true);

    graph.add_node(GraphNode::new("main".to_string(), "Main Function".to_string()));
    graph.add_node(GraphNode::new("helper1".to_string(), "Helper 1".to_string()));
    graph.add_node(GraphNode::new("helper2".to_string(), "Helper 2".to_string()));
    graph.add_node(GraphNode::new("util".to_string(), "Utility".to_string()));

    graph.add_edge(GraphEdge::new("main".to_string(), "helper1".to_string()));
    graph.add_edge(GraphEdge::new("main".to_string(), "helper2".to_string()));
    graph.add_edge(GraphEdge::new("helper1".to_string(), "util".to_string()));
    graph.add_edge(GraphEdge::new("helper2".to_string(), "util".to_string()));

    // 2. Use graph layout algorithm
    let layout_engine = GraphLayout::new().with_algorithm(GraphLayoutAlgorithm::ForceDirected);
    let layout_result = layout_engine.layout_graph(&graph);
    assert!(layout_result.is_ok());

    let layout = layout_result.unwrap();
    assert_eq!(layout.nodes.len(), 4);
    assert_eq!(layout.edges.len(), 4);

    // 3. Render to SVG
    let renderer = SvgRenderer::new();
    let svg_result = renderer.render_layout(&layout);
    assert!(svg_result.is_ok());

    let svg = svg_result.unwrap();
    assert!(svg.contains("<svg"));
    assert!(svg.contains("Main Function"));
    assert!(svg.contains("Helper 1"));
    assert!(svg.contains("Helper 2"));
    assert!(svg.contains("Utility"));
}

#[test]
fn test_complete_tree_visualization_pipeline() {
    // 1. Create tree data structure
    let root = TreeNode::new("root".to_string(), "Root Package".to_string(), "package".to_string());

    let module1 = TreeNode::new("module1".to_string(), "Module 1".to_string(), "module".to_string()).with_child(TreeNode::new("func1".to_string(), "Function 1".to_string(), "function".to_string())).with_child(TreeNode::new(
        "func2".to_string(),
        "Function 2".to_string(),
        "function".to_string(),
    ));

    let module2 = TreeNode::new("module2".to_string(), "Module 2".to_string(), "module".to_string()).with_child(TreeNode::new("class1".to_string(), "Class 1".to_string(), "class".to_string()));

    let root = root.with_child(module1).with_child(module2);

    // 2. Use tree layout algorithm
    let layout_engine = TreeLayout::new().with_algorithm(TreeLayoutAlgorithm::Layered);
    let layout_result = layout_engine.layout_tree(&root);
    assert!(layout_result.is_ok());

    let layout = layout_result.unwrap();
    assert_eq!(layout.nodes.len(), 6); // root + 2 modules + 3 children

    // 3. Render to SVG
    let renderer = SvgRenderer::new();
    let svg_result = renderer.render_layout(&layout);
    assert!(svg_result.is_ok());

    let svg = svg_result.unwrap();
    assert!(svg.contains("<svg"));
    assert!(svg.contains("Root Package"));
    assert!(svg.contains("Module 1"));
}

#[test]
fn test_theme_integration_with_rendering() {
    // 1. Create a simple layout
    let mut layout = Layout::new();
    layout.add_node("test_node".to_string(), Rect::new(Point::new(50.0, 50.0), Size::new(100.0, 60.0)));

    // 2. Test rendering configuration with different themes
    let themes = vec![VisualizationTheme::light(), VisualizationTheme::dark(), VisualizationTheme::one_light(), VisualizationTheme::one_dark_pro(), VisualizationTheme::github()];

    for theme in themes {
        let config = theme.to_render_config();

        let renderer = SvgRenderer::new().with_config(config);
        let svg_result = renderer.render_layout(&layout);
        assert!(svg_result.is_ok());

        let svg = svg_result.unwrap();
        assert!(svg.contains(&theme.node.fill_color));
        assert!(svg.contains("test_node"));
    }
}

#[test]
fn test_multi_format_export_integration() {
    // 1. Create a layout
    let mut layout = Layout::new();
    layout.add_node("export_test".to_string(), Rect::new(Point::new(0.0, 0.0), Size::new(80.0, 40.0)));

    // 2. Test multiple export formats
    let formats = vec![ExportFormat::Svg, ExportFormat::Html, ExportFormat::Json];

    for format in formats {
        let exporter = LayoutExporter::new(format);
        let export_result = exporter.export(&layout);
        assert!(export_result.is_ok());

        let content = export_result.unwrap();
        match format {
            ExportFormat::Svg => {
                assert!(content.contains("<svg"));
                assert!(content.contains("export_test"))
            }
            ExportFormat::Html => {
                assert!(content.contains("<!DOCTYPE html"));
                assert!(content.contains("<svg"));
                assert!(content.contains("export_test"))
            }
            ExportFormat::Json => {
                assert!(content.contains("nodes"));
                assert!(content.contains("edges"));
                assert!(content.contains("export_test"))
            }
        }
    }
}

#[test]
fn test_geometry_layout_render_integration() {
    // 1. Use geometry module to create a complex layout
    let nodes = vec![
        ("center".to_string(), Rect::new(Point::new(200.0, 200.0), Size::new(100.0, 50.0))),
        ("top".to_string(), Rect::new(Point::new(200.0, 100.0), Size::new(80.0, 40.0))),
        ("bottom".to_string(), Rect::new(Point::new(200.0, 300.0), Size::new(80.0, 40.0))),
        ("left".to_string(), Rect::new(Point::new(100.0, 200.0), Size::new(80.0, 40.0))),
        ("right".to_string(), Rect::new(Point::new(300.0, 200.0), Size::new(80.0, 40.0))),
    ];

    let mut layout = Layout::new();
    for (id, rect) in nodes {
        layout.add_node(id, rect)
    }

    // 2. Add edge connections
    let edges = vec![("center", "top"), ("center", "bottom"), ("center", "left"), ("center", "right")];

    for (from, to) in edges {
        let from_rect = layout.nodes.get(from).unwrap();
        let to_rect = layout.nodes.get(to).unwrap();

        let edge_points = vec![from_rect.rect.center(), to_rect.rect.center()];

        let edge = Edge::new(from.to_string(), to.to_string()).with_points(edge_points);
        layout.add_edge(edge)
    }

    // 3. Render and verify
    let renderer = SvgRenderer::new();
    let svg_result = renderer.render_layout(&layout);
    assert!(svg_result.is_ok());

    let svg = svg_result.unwrap();
    assert!(svg.contains("center"));
    assert!(svg.contains("top"));
    assert!(svg.contains("bottom"));
    assert!(svg.contains("left"));
    assert!(svg.contains("right"));

    // Verify SVG contains correct coordinates
    assert!(svg.contains("200")); // center position
    assert!(svg.contains("100")); // top position
    assert!(svg.contains("300")); // bottom/right positions
}

#[test]
fn test_error_handling_integration() {
    // Test handling of an empty graph
    let empty_graph = Graph::new(true);
    let layout_engine = GraphLayout::new().with_algorithm(GraphLayoutAlgorithm::Circular);
    let layout_result = layout_engine.layout_graph(&empty_graph);
    assert!(layout_result.is_ok());

    let layout = layout_result.unwrap();
    assert!(layout.nodes.is_empty());
    assert!(layout.edges.is_empty());

    // Test rendering of an empty layout
    let renderer = SvgRenderer::new();
    let svg_result = renderer.render_layout(&layout);
    assert!(svg_result.is_ok());

    let svg = svg_result.unwrap();
    assert!(svg.contains("<svg"));
    assert!(svg.contains("</svg>"))
}

#[test]
fn test_large_graph_performance() {
    // Create a large graph to test performance
    let mut graph = Graph::new(false);

    // Add 100 nodes
    for i in 0..100 {
        graph.add_node(GraphNode::new(format!("node_{}", i), format!("Node {}", i)))
    }

    // Add some edges (create grid-like connections)
    for i in 0..10 {
        for j in 0..10 {
            let current = i * 10 + j;
            if j < 9 {
                // Horizontal connection
                graph.add_edge(GraphEdge::new(format!("node_{}", current), format!("node_{}", current + 1)))
            }
            if i < 9 {
                // Vertical connection
                graph.add_edge(GraphEdge::new(format!("node_{}", current), format!("node_{}", current + 10)))
            }
        }
    }

    // Use force-directed layout (computationally intensive)
    let layout_engine = GraphLayout::new().with_algorithm(GraphLayoutAlgorithm::ForceDirected);
    let layout_result = layout_engine.layout_graph(&graph);
    assert!(layout_result.is_ok());

    let layout = layout_result.unwrap();
    assert_eq!(layout.nodes.len(), 100);

    // Render the large graph
    let renderer = SvgRenderer::new();
    let svg_result = renderer.render_layout(&layout);
    assert!(svg_result.is_ok());

    let svg = svg_result.unwrap();
    assert!(svg.contains("Node 0"));
    assert!(svg.contains("Node 99"))
}
