use oak_visualize::{
    geometry::{Point, Rect, Size},
    layout::Layout,
    render::{ElementStyle, ExportFormat, LayoutExporter, RenderConfig, SvgRenderer},
};

#[test]
fn test_svg_renderer_creation() {
    let renderer = SvgRenderer::new();
    assert_eq!(renderer.config().canvas_width, 800.0);
    assert_eq!(renderer.config().canvas_height, 600.0);
}

#[test]
fn test_render_empty_layout() {
    let renderer = SvgRenderer::new();
    let layout = Layout::new();
    let result = renderer.render_layout(&layout);

    assert!(result.is_ok());
    let svg = result.unwrap();
    assert!(svg.contains("<svg"));
    assert!(svg.contains("</svg>"));
}

#[test]
fn test_render_simple_layout() {
    let renderer = SvgRenderer::new();
    let mut layout = Layout::new();

    layout.add_node("test".to_string(), Rect::new(Point::new(10.0, 20.0), Size::new(100.0, 50.0)));

    let result = renderer.render_layout(&layout);
    assert!(result.is_ok());

    let svg = result.unwrap();
    assert!(svg.contains("rect"));
    assert!(svg.contains("test"));
}

#[test]
fn test_element_style() {
    let style = ElementStyle::new().with_fill("#ff0000".to_string()).with_stroke("#000000".to_string(), 2.0).with_opacity(0.8);

    assert_eq!(style.fill_color, Some("#ff0000".to_string()));
    assert_eq!(style.stroke_color, Some("#000000".to_string()));
    assert_eq!(style.stroke_width, Some(2.0));
    assert_eq!(style.opacity, Some(0.8));
}

#[test]
fn test_layout_exporter() {
    let exporter = LayoutExporter::new(ExportFormat::Json);
    let mut layout = Layout::new();

    layout.add_node("node1".to_string(), Rect::new(Point::new(0.0, 0.0), Size::new(50.0, 30.0)));

    let result = exporter.export(&layout);
    assert!(result.is_ok());

    let json = result.unwrap();
    assert!(json.contains("node1"));
    assert!(json.contains("nodes"));
    assert!(json.contains("edges"));
}

#[test]
fn test_render_config_defaults() {
    let config = RenderConfig::default();

    assert_eq!(config.canvas_width, 800.0);
    assert_eq!(config.canvas_height, 600.0);
    assert_eq!(config.background_color, "#ffffff");
    assert_eq!(config.node_fill_color, "#e1f5fe");
    assert_eq!(config.node_stroke_color, "#0277bd");
    assert_eq!(config.edge_color, "#666666");
    assert!(config.show_labels);
    assert!(config.show_arrows);
}

#[test]
fn test_element_style_builder() {
    let style = ElementStyle::new()
        .with_fill("#blue".to_string())
        .with_text("#white".to_string(), 14.0)
        .with_class("node-style".to_string())
        .with_attribute("data-id".to_string(), "test".to_string());

    assert_eq!(style.fill_color, Some("#blue".to_string()));
    assert_eq!(style.text_color, Some("#white".to_string()));
    assert_eq!(style.text_size, Some(14.0));
    assert_eq!(style.class_name, Some("node-style".to_string()));
    assert_eq!(style.attributes.get("data-id"), Some(&"test".to_string()));
}

#[test]
fn test_svg_renderer_with_config() {
    let config = RenderConfig {
        canvas_width: 1200.0,
        canvas_height: 800.0,
        background_color: "#f0f0f0".to_string(),
        ..Default::default()
    };

    let renderer = SvgRenderer::new().with_config(config.clone());
    assert_eq!(renderer.config().canvas_width, 1200.0);
    assert_eq!(renderer.config().canvas_height, 800.0);
    assert_eq!(renderer.config().background_color, "#f0f0f0");
}

#[test]
fn test_layout_exporter_formats() {
    let layout = Layout::new();

    // Test SVG export
    let svg_exporter = LayoutExporter::new(ExportFormat::Svg);
    let svg_result = svg_exporter.export(&layout);
    assert!(svg_result.is_ok());
    assert!(svg_result.unwrap().contains("<svg"));

    // Test HTML export
    let html_exporter = LayoutExporter::new(ExportFormat::Html);
    let html_result = html_exporter.export(&layout);
    assert!(html_result.is_ok());
    let html = html_result.unwrap();
    assert!(html.contains("<!DOCTYPE html"));
    assert!(html.contains("<svg"));

    // Test JSON export
    let json_exporter = LayoutExporter::new(ExportFormat::Json);
    let json_result = json_exporter.export(&layout);
    assert!(json_result.is_ok());
    let json = json_result.unwrap();
    assert!(json.contains("nodes"));
    assert!(json.contains("edges"));
}
