use oak_visualize::theme::VisualizationTheme;

#[test]
fn test_default_theme() {
    let theme = VisualizationTheme::default();
    assert_eq!(theme.name, "Light");
    assert_eq!(theme.background_color, "#FFFFFF")
}

#[test]
fn test_one_light_theme() {
    let theme = VisualizationTheme::one_light();
    assert_eq!(theme.name, "One Light");
    assert_eq!(theme.background_color, "#FAFAFA");
    assert_eq!(theme.node.fill_color, "#FFFFFF")
}

#[test]
fn test_one_dark_pro_theme() {
    let theme = VisualizationTheme::one_dark_pro();
    assert_eq!(theme.name, "One Dark Pro");
    assert_eq!(theme.background_color, "#282C34");
    assert_eq!(theme.node.fill_color, "#21252B")
}

#[test]
fn test_github_theme() {
    let theme = VisualizationTheme::github();
    assert_eq!(theme.name, "GitHub");
    assert_eq!(theme.background_color, "#FFFFFF");
    assert_eq!(theme.node.fill_color, "#F6F8FA")
}

#[test]
fn test_light_theme() {
    let theme = VisualizationTheme::light();
    assert_eq!(theme.name, "Light");
    assert_eq!(theme.background_color, "#FFFFFF");
    assert_eq!(theme.node.fill_color, "#F8F9FA");
    assert_eq!(theme.edge.color, "#6C757D")
}

#[test]
fn test_dark_theme() {
    let theme = VisualizationTheme::dark();
    assert_eq!(theme.name, "Dark");
    assert_eq!(theme.background_color, "#1E1E1E");
    assert_eq!(theme.node.fill_color, "#2D2D30");
    assert_eq!(theme.edge.color, "#CCCCCC")
}

#[test]
fn test_node_theme_properties() {
    let theme = VisualizationTheme::default();
    let node = &theme.node;

    assert!(!node.fill_color.is_empty());
    assert!(!node.stroke_color.is_empty());
    assert!(node.stroke_width > 0.0);
    assert!(node.border_radius >= 0.0)
}

#[test]
fn test_edge_theme_properties() {
    let theme = VisualizationTheme::default();
    let edge = &theme.edge;

    assert!(!edge.color.is_empty());
    assert!(edge.width > 0.0);
    assert!(!edge.style.is_empty())
}

#[test]
fn test_text_theme_properties() {
    let theme = VisualizationTheme::default();
    let text = &theme.text;

    assert!(!text.font_family.is_empty());
    assert!(text.font_size > 0.0);
    assert!(!text.color.is_empty());
    assert!(!text.font_weight.is_empty())
}

#[test]
fn test_highlight_theme_properties() {
    let theme = VisualizationTheme::default();
    let highlight = &theme.highlight;

    assert!(!highlight.selected_color.is_empty());
    assert!(!highlight.hover_color.is_empty());
    assert!(!highlight.error_color.is_empty());
    assert!(!highlight.warning_color.is_empty())
}

#[test]
fn test_shadow_config() {
    let theme = VisualizationTheme::default();
    let shadow = &theme.node.shadow;

    // Shadow should have valid configuration
    assert!(!shadow.color.is_empty());
    assert!(shadow.blur_radius >= 0.0)
}

#[test]
fn test_arrow_config() {
    let theme = VisualizationTheme::default();
    let arrow = &theme.edge.arrow;

    assert!(arrow.size > 0.0);
    assert!(!arrow.arrow_type.is_empty())
}

#[test]
fn test_theme_serialization() {
    let theme = VisualizationTheme::one_light();

    // Test that theme can be serialized to JSON
    let json = oak_json::to_string(&theme);
    assert!(json.is_ok());

    // Test that theme can be deserialized from JSON
    let json_str = json.unwrap();
    let deserialized: Result<VisualizationTheme, _> = oak_json::from_str(&json_str);
    assert!(deserialized.is_ok());

    let deserialized_theme = deserialized.unwrap();
    assert_eq!(deserialized_theme.name, theme.name);
    assert_eq!(deserialized_theme.background_color, theme.background_color)
}

#[test]
fn test_theme_color_formats() {
    let themes = vec![VisualizationTheme::light(), VisualizationTheme::dark(), VisualizationTheme::one_light(), VisualizationTheme::one_dark_pro(), VisualizationTheme::github()];

    for theme in themes {
        // All colors should start with # (hex format)
        assert!(theme.background_color.starts_with('#'));
        assert!(theme.node.fill_color.starts_with('#'));
        assert!(theme.node.stroke_color.starts_with('#'));
        assert!(theme.edge.color.starts_with('#'));
        assert!(theme.text.color.starts_with('#'));
        assert!(theme.highlight.selected_color.starts_with('#'));
        assert!(theme.highlight.hover_color.starts_with('#'));
        assert!(theme.highlight.error_color.starts_with('#'));
        assert!(theme.highlight.warning_color.starts_with('#'))
    }
}
