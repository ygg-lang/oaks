/// Visualization theme configuration.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VisualizationTheme {
    /// The name of the theme.
    pub name: String,
    /// The background color of the visualization (hex string).
    pub background_color: String,
    /// The style configuration for nodes.
    pub node: NodeTheme,
    /// The style configuration for edges.
    pub edge: EdgeTheme,
    /// The style configuration for text.
    pub text: TextTheme,
    /// The style configuration for highlighted states.
    pub highlight: HighlightTheme,
}

/// Node theme configuration.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NodeTheme {
    /// The default fill color for nodes (hex string).
    pub fill_color: String,
    /// The default stroke color for nodes (hex string).
    pub stroke_color: String,
    /// The width of the node's stroke.
    pub stroke_width: f32,
    /// The border radius for rounded node corners.
    pub border_radius: f32,
    /// The shadow configuration for nodes.
    pub shadow: ShadowConfig,
}

/// Edge theme configuration.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EdgeTheme {
    /// The default color for edges (hex string).
    pub color: String,
    /// The width of the edge line.
    pub width: f32,
    /// The line style (e.g., "solid", "dashed", "dotted").
    pub style: String,
    /// The arrowhead configuration for directed edges.
    pub arrow: ArrowConfig,
}

/// Text theme configuration.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextTheme {
    /// The font family for text elements.
    pub font_family: String,
    /// The font size in pixels.
    pub font_size: f32,
    /// The default color for text (hex string).
    pub color: String,
    /// The font weight (e.g., "normal", "bold").
    pub font_weight: String,
}

/// Highlight theme configuration.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HighlightTheme {
    /// The color used for selected elements (hex string).
    pub selected_color: String,
    /// The color used for elements on hover (hex string).
    pub hover_color: String,
    /// The color used for elements in an error state (hex string).
    pub error_color: String,
    /// The color used for elements in a warning state (hex string).
    pub warning_color: String,
}

/// Shadow configuration.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ShadowConfig {
    /// Whether to enable shadow effects.
    pub enabled: bool,
    /// The color of the shadow (hex string).
    pub color: String,
    /// The horizontal offset of the shadow.
    pub offset_x: f32,
    /// The vertical offset of the shadow.
    pub offset_y: f32,
    /// The blur radius of the shadow.
    pub blur_radius: f32,
}

/// Arrow configuration for directed edges.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ArrowConfig {
    /// Whether to show arrowheads.
    pub enabled: bool,
    /// The size of the arrowhead.
    pub size: f32,
    /// The type of arrowhead (e.g., "triangle", "circle", "diamond").
    pub arrow_type: String,
}

impl VisualizationTheme {
    /// Converts to render configuration
    pub fn to_render_config(&self) -> crate::render::RenderConfig {
        crate::render::RenderConfig {
            background_color: self.background_color.clone(),
            node_fill_color: self.node.fill_color.clone(),
            node_stroke_color: self.node.stroke_color.clone(),
            node_stroke_width: self.node.stroke_width as f64,
            edge_color: self.edge.color.clone(),
            edge_width: self.edge.width as f64,
            text_color: self.text.color.clone(),
            text_size: self.text.font_size as f64,
            font_family: self.text.font_family.clone(),
            show_arrows: self.edge.arrow.enabled,
            arrow_size: self.edge.arrow.size as f64,
            ..Default::default()
        }
    }
}

impl Default for VisualizationTheme {
    fn default() -> Self {
        Self::light()
    }
}

impl VisualizationTheme {
    /// Light theme
    pub fn light() -> Self {
        Self {
            name: "Light".to_string(),
            background_color: "#FFFFFF".to_string(),
            node: NodeTheme {
                fill_color: "#F8F9FA".to_string(),
                stroke_color: "#DEE2E6".to_string(),
                stroke_width: 1.0,
                border_radius: 4.0,
                shadow: ShadowConfig { enabled: true, color: "rgba(0, 0, 0, 0.1)".to_string(), offset_x: 0.0, offset_y: 2.0, blur_radius: 4.0 },
            },
            edge: EdgeTheme { color: "#6C757D".to_string(), width: 1.5, style: "solid".to_string(), arrow: ArrowConfig { enabled: true, size: 8.0, arrow_type: "triangle".to_string() } },
            text: TextTheme { font_family: "Arial, sans-serif".to_string(), font_size: 12.0, color: "#212529".to_string(), font_weight: "normal".to_string() },
            highlight: HighlightTheme { selected_color: "#007BFF".to_string(), hover_color: "#0056B3".to_string(), error_color: "#DC3545".to_string(), warning_color: "#FFC107".to_string() },
        }
    }

    /// Dark theme
    pub fn dark() -> Self {
        Self {
            name: "Dark".to_string(),
            background_color: "#1E1E1E".to_string(),
            node: NodeTheme {
                fill_color: "#2D2D30".to_string(),
                stroke_color: "#3E3E42".to_string(),
                stroke_width: 1.0,
                border_radius: 4.0,
                shadow: ShadowConfig { enabled: true, color: "rgba(0, 0, 0, 0.3)".to_string(), offset_x: 0.0, offset_y: 2.0, blur_radius: 4.0 },
            },
            edge: EdgeTheme { color: "#CCCCCC".to_string(), width: 1.5, style: "solid".to_string(), arrow: ArrowConfig { enabled: true, size: 8.0, arrow_type: "triangle".to_string() } },
            text: TextTheme { font_family: "Arial, sans-serif".to_string(), font_size: 12.0, color: "#CCCCCC".to_string(), font_weight: "normal".to_string() },
            highlight: HighlightTheme { selected_color: "#0E639C".to_string(), hover_color: "#1177BB".to_string(), error_color: "#F14C4C".to_string(), warning_color: "#FFCC02".to_string() },
        }
    }

    /// One Light theme - based on Atom One Light
    pub fn one_light() -> Self {
        Self {
            name: "One Light".to_string(),
            background_color: "#FAFAFA".to_string(),
            node: NodeTheme {
                fill_color: "#FFFFFF".to_string(),
                stroke_color: "#E1E4E8".to_string(),
                stroke_width: 1.0,
                border_radius: 6.0,
                shadow: ShadowConfig { enabled: true, color: "rgba(149, 157, 165, 0.2)".to_string(), offset_x: 0.0, offset_y: 8.0, blur_radius: 24.0 },
            },
            edge: EdgeTheme { color: "#586069".to_string(), width: 1.5, style: "solid".to_string(), arrow: ArrowConfig { enabled: true, size: 8.0, arrow_type: "triangle".to_string() } },
            text: TextTheme { font_family: "SF Pro Display, -apple-system, BlinkMacSystemFont, Segoe UI, Roboto, sans-serif".to_string(), font_size: 12.0, color: "#24292E".to_string(), font_weight: "400".to_string() },
            highlight: HighlightTheme { selected_color: "#0366D6".to_string(), hover_color: "#0256CC".to_string(), error_color: "#D73A49".to_string(), warning_color: "#F66A0A".to_string() },
        }
    }

    /// One Dark Pro theme - based on Atom One Dark Pro
    pub fn one_dark_pro() -> Self {
        Self {
            name: "One Dark Pro".to_string(),
            background_color: "#282C34".to_string(),
            node: NodeTheme {
                fill_color: "#21252B".to_string(),
                stroke_color: "#3E4451".to_string(),
                stroke_width: 1.0,
                border_radius: 6.0,
                shadow: ShadowConfig { enabled: true, color: "rgba(0, 0, 0, 0.4)".to_string(), offset_x: 0.0, offset_y: 8.0, blur_radius: 24.0 },
            },
            edge: EdgeTheme { color: "#ABB2BF".to_string(), width: 1.5, style: "solid".to_string(), arrow: ArrowConfig { enabled: true, size: 8.0, arrow_type: "triangle".to_string() } },
            text: TextTheme { font_family: "SF Mono, Monaco, Inconsolata, Roboto Mono, monospace".to_string(), font_size: 12.0, color: "#ABB2BF".to_string(), font_weight: "400".to_string() },
            highlight: HighlightTheme { selected_color: "#61AFEF".to_string(), hover_color: "#528BFF".to_string(), error_color: "#E06C75".to_string(), warning_color: "#E5C07B".to_string() },
        }
    }

    /// GitHub theme
    pub fn github() -> Self {
        Self {
            name: "GitHub".to_string(),
            background_color: "#FFFFFF".to_string(),
            node: NodeTheme {
                fill_color: "#F6F8FA".to_string(),
                stroke_color: "#D0D7DE".to_string(),
                stroke_width: 1.0,
                border_radius: 6.0,
                shadow: ShadowConfig { enabled: true, color: "rgba(31, 35, 40, 0.04)".to_string(), offset_x: 0.0, offset_y: 1.0, blur_radius: 0.0 },
            },
            edge: EdgeTheme { color: "#656D76".to_string(), width: 1.0, style: "solid".to_string(), arrow: ArrowConfig { enabled: true, size: 6.0, arrow_type: "triangle".to_string() } },
            text: TextTheme { font_family: "-apple-system, BlinkMacSystemFont, Segoe UI, Helvetica, Arial, sans-serif".to_string(), font_size: 12.0, color: "#24292F".to_string(), font_weight: "400".to_string() },
            highlight: HighlightTheme { selected_color: "#0969DA".to_string(), hover_color: "#0860CA".to_string(), error_color: "#CF222E".to_string(), warning_color: "#9A6700".to_string() },
        }
    }
}
