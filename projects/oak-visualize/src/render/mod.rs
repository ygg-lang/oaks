#![doc = "Rendering module for converting layouts to visual formats"]

use crate::{
    geometry::{Point, Rect, Size},
    layout::{Edge, Layout},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Rendering configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderConfig {
    pub canvas_width: f64,
    pub canvas_height: f64,
    pub background_color: String,
    pub node_fill_color: String,
    pub node_stroke_color: String,
    pub node_stroke_width: f64,
    pub edge_color: String,
    pub edge_width: f64,
    pub text_color: String,
    pub text_size: f64,
    pub font_family: String,
    pub padding: f64,
    pub show_labels: bool,
    pub show_arrows: bool,
    pub arrow_size: f64,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            canvas_width: 800.0,
            canvas_height: 600.0,
            background_color: "#ffffff".to_string(),
            node_fill_color: "#e1f5fe".to_string(),
            node_stroke_color: "#0277bd".to_string(),
            node_stroke_width: 2.0,
            edge_color: "#666666".to_string(),
            edge_width: 1.5,
            text_color: "#333333".to_string(),
            text_size: 12.0,
            font_family: "Arial, sans-serif".to_string(),
            padding: 20.0,
            show_labels: true,
            show_arrows: true,
            arrow_size: 8.0,
        }
    }
}

/// Style information for rendering elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementStyle {
    pub fill_color: Option<String>,
    pub stroke_color: Option<String>,
    pub stroke_width: Option<f64>,
    pub text_color: Option<String>,
    pub text_size: Option<f64>,
    pub opacity: Option<f64>,
    pub class_name: Option<String>,
    pub attributes: HashMap<String, String>,
}

impl Default for ElementStyle {
    fn default() -> Self {
        Self {
            fill_color: None,
            stroke_color: None,
            stroke_width: None,
            text_color: None,
            text_size: None,
            opacity: None,
            class_name: None,
            attributes: HashMap::new(),
        }
    }
}

impl ElementStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_fill(mut self, color: String) -> Self {
        self.fill_color = Some(color);
        self
    }

    pub fn with_stroke(mut self, color: String, width: f64) -> Self {
        self.stroke_color = Some(color);
        self.stroke_width = Some(width);
        self
    }

    pub fn with_text(mut self, color: String, size: f64) -> Self {
        self.text_color = Some(color);
        self.text_size = Some(size);
        self
    }

    pub fn with_opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn with_class(mut self, class_name: String) -> Self {
        self.class_name = Some(class_name);
        self
    }

    pub fn with_attribute(mut self, key: String, value: String) -> Self {
        self.attributes.insert(key, value);
        self
    }
}

/// SVG renderer for layouts
pub struct SvgRenderer {
    config: RenderConfig,
    node_styles: HashMap<String, ElementStyle>,
    edge_styles: HashMap<String, ElementStyle>,
}

impl SvgRenderer {
    pub fn new() -> Self {
        Self { config: RenderConfig::default(), node_styles: HashMap::new(), edge_styles: HashMap::new() }
    }

    pub fn with_config(mut self, config: RenderConfig) -> Self {
        self.config = config;
        self
    }

    pub fn set_node_style(&mut self, node_id: String, style: ElementStyle) {
        self.node_styles.insert(node_id, style);
    }

    pub fn set_edge_style(&mut self, edge_id: String, style: ElementStyle) {
        self.edge_styles.insert(edge_id, style);
    }

    pub fn render_layout(&self, layout: &Layout) -> crate::Result<String> {
        let mut svg = String::new();

        // Calculate bounds and apply padding
        let bounds = self.calculate_bounds(layout);
        let canvas_width = bounds.size.width + 2.0 * self.config.padding;
        let canvas_height = bounds.size.height + 2.0 * self.config.padding;

        // SVG header
        svg.push_str(&format!(
            r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">"#,
            canvas_width, canvas_height
        ));
        svg.push('\n');

        // Background
        svg.push_str(&format!(r#"  <rect width="100%" height="100%" fill="{}"/>"#, self.config.background_color));
        svg.push('\n');

        // Define styles
        svg.push_str("  <defs>\n");
        svg.push_str("    <style>\n");
        svg.push_str("      .node { cursor: pointer; }\n");
        svg.push_str("      .node:hover { opacity: 0.8; }\n");
        svg.push_str("      .edge { pointer-events: none; }\n");
        svg.push_str("      .label { pointer-events: none; user-select: none; }\n");
        svg.push_str("    </style>\n");

        // Arrow marker for directed edges
        if self.config.show_arrows {
            svg.push_str(&format!(
                r#"    <marker id="arrowhead" markerWidth="{}" markerHeight="{}" refX="{}" refY="{}" orient="auto">
      <polygon points="0 0, {} {}, {} 0" fill="{}"/>
    </marker>"#,
                self.config.arrow_size,
                self.config.arrow_size,
                self.config.arrow_size,
                self.config.arrow_size / 2.0,
                self.config.arrow_size,
                self.config.arrow_size,
                self.config.arrow_size,
                self.config.edge_color
            ));
            svg.push('\n');
        }

        svg.push_str("  </defs>\n");

        // Transform group to apply padding offset
        svg.push_str(&format!(
            r#"  <g transform="translate({}, {})">"#,
            self.config.padding - bounds.origin.x,
            self.config.padding - bounds.origin.y
        ));
        svg.push('\n');

        // Render edges first (so they appear behind nodes)
        for edge in &layout.edges {
            self.render_edge(&mut svg, edge)?;
        }

        // Render nodes
        for (node_id, rect) in &layout.nodes {
            self.render_node(&mut svg, node_id, rect)?;
        }

        svg.push_str("  </g>\n");
        svg.push_str("</svg>");

        Ok(svg)
    }

    fn render_node(&self, svg: &mut String, node_id: &str, rect: &Rect) -> crate::Result<()> {
        let style = self.node_styles.get(node_id);

        let fill_color = style.and_then(|s| s.fill_color.as_ref()).unwrap_or(&self.config.node_fill_color);
        let stroke_color = style.and_then(|s| s.stroke_color.as_ref()).unwrap_or(&self.config.node_stroke_color);
        let stroke_width = style.and_then(|s| s.stroke_width).unwrap_or(self.config.node_stroke_width);

        // Node rectangle
        svg.push_str(&format!(
            r#"    <rect x="{}" y="{}" width="{}" height="{}" fill="{}" stroke="{}" stroke-width="{}" class="node""#,
            rect.origin.x, rect.origin.y, rect.size.width, rect.size.height, fill_color, stroke_color, stroke_width
        ));

        // Add custom attributes
        if let Some(style) = style {
            if let Some(opacity) = style.opacity {
                svg.push_str(&format!(r#" opacity="{}""#, opacity));
            }
            if let Some(class) = &style.class_name {
                svg.push_str(&format!(r#" class="node {}""#, class));
            }
            for (key, value) in &style.attributes {
                svg.push_str(&format!(r#" {}="{}""#, key, value));
            }
        }

        svg.push_str("/>\n");

        // Node label
        if self.config.show_labels {
            let text_color = style.and_then(|s| s.text_color.as_ref()).unwrap_or(&self.config.text_color);
            let text_size = style.and_then(|s| s.text_size).unwrap_or(self.config.text_size);

            let center = rect.center();
            svg.push_str(&format!(
                r#"    <text x="{}" y="{}" text-anchor="middle" dominant-baseline="central" fill="{}" font-size="{}" font-family="{}" class="label">{}</text>"#,
                center.x,
                center.y,
                text_color,
                text_size,
                self.config.font_family,
                node_id
            ));
            svg.push('\n');
        }

        Ok(())
    }

    fn render_edge(&self, svg: &mut String, edge: &Edge) -> crate::Result<()> {
        let edge_id = format!("{}_{}", edge.from, edge.to);
        let style = self.edge_styles.get(&edge_id);

        let stroke_color = style.and_then(|s| s.stroke_color.as_ref()).unwrap_or(&self.config.edge_color);
        let stroke_width = style.and_then(|s| s.stroke_width).unwrap_or(self.config.edge_width);

        if edge.points.len() < 2 {
            return Ok(());
        }

        // Create path from points
        let mut path_data = String::new();
        path_data.push_str(&format!("M {} {}", edge.points[0].x, edge.points[0].y));

        for point in &edge.points[1..] {
            path_data.push_str(&format!(" L {} {}", point.x, point.y));
        }

        svg.push_str(&format!(
            r#"    <path d="{}" stroke="{}" stroke-width="{}" fill="none" class="edge""#,
            path_data, stroke_color, stroke_width
        ));

        // Add arrow marker for directed edges
        if self.config.show_arrows {
            svg.push_str(r#" marker-end="url(#arrowhead)""#);
        }

        // Add custom attributes
        if let Some(style) = style {
            if let Some(opacity) = style.opacity {
                svg.push_str(&format!(r#" opacity="{}""#, opacity));
            }
            if let Some(class) = &style.class_name {
                svg.push_str(&format!(r#" class="edge {}""#, class));
            }
            for (key, value) in &style.attributes {
                svg.push_str(&format!(r#" {}="{}""#, key, value));
            }
        }

        svg.push_str("/>\n");

        // Edge label
        if let Some(label) = &edge.label {
            let mid_point = if edge.points.len() >= 2 {
                let start = &edge.points[0];
                let end = &edge.points[edge.points.len() - 1];
                Point::new((start.x + end.x) / 2.0, (start.y + end.y) / 2.0)
            }
            else {
                edge.points[0]
            };

            let text_color = style.and_then(|s| s.text_color.as_ref()).unwrap_or(&self.config.text_color);
            let text_size = style.and_then(|s| s.text_size).unwrap_or(self.config.text_size * 0.8);

            svg.push_str(&format!(
                r#"    <text x="{}" y="{}" text-anchor="middle" dominant-baseline="central" fill="{}" font-size="{}" font-family="{}" class="label">{}</text>"#,
                mid_point.x,
                mid_point.y - 5.0, // Offset slightly above the edge
                text_color,
                text_size,
                self.config.font_family,
                label
            ));
            svg.push('\n');
        }

        Ok(())
    }

    fn calculate_bounds(&self, layout: &Layout) -> Rect {
        if layout.nodes.is_empty() {
            return Rect::new(Point::origin(), Size::new(self.config.canvas_width, self.config.canvas_height));
        }

        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for rect in layout.nodes.values() {
            min_x = min_x.min(rect.origin.x);
            min_y = min_y.min(rect.origin.y);
            max_x = max_x.max(rect.origin.x + rect.size.width);
            max_y = max_y.max(rect.origin.y + rect.size.height);
        }

        Rect::new(Point::new(min_x, min_y), Size::new(max_x - min_x, max_y - min_y))
    }
}

impl Default for SvgRenderer {
    fn default() -> Self {
        Self::new()
    }
}

/// Export formats for rendered layouts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Svg,
    Html,
    Json,
}

/// Layout exporter
pub struct LayoutExporter {
    format: ExportFormat,
    config: RenderConfig,
}

impl LayoutExporter {
    pub fn new(format: ExportFormat) -> Self {
        Self { format, config: RenderConfig::default() }
    }

    pub fn with_config(mut self, config: RenderConfig) -> Self {
        self.config = config;
        self
    }

    pub fn export(&self, layout: &Layout) -> crate::Result<String> {
        match self.format {
            ExportFormat::Svg => {
                let renderer = SvgRenderer::new().with_config(self.config.clone());
                renderer.render_layout(layout)
            }
            ExportFormat::Html => self.export_html(layout),
            ExportFormat::Json => self.export_json(layout),
        }
    }

    fn export_html(&self, layout: &Layout) -> crate::Result<String> {
        let renderer = SvgRenderer::new().with_config(self.config.clone());
        let svg_content = renderer.render_layout(layout)?;

        let html = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Pex Visualization</title>
    <style>
        body {{
            margin: 0;
            padding: 20px;
            font-family: Arial, sans-serif;
            background-color: #f5f5f5;
        }}
        .container {{
            max-width: 100%;
            margin: 0 auto;
            background-color: white;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
            padding: 20px;
        }}
        svg {{
            max-width: 100%;
            height: auto;
            border: 1px solid #ddd;
            border-radius: 4px;
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1>Pex Visualization</h1>
        {}
    </div>
</body>
</html>"#,
            svg_content
        );

        Ok(html)
    }

    fn export_json(&self, layout: &Layout) -> crate::Result<String> {
        #[derive(Serialize)]
        struct JsonLayout {
            nodes: HashMap<String, JsonRect>,
            edges: Vec<JsonEdge>,
        }

        #[derive(Serialize)]
        struct JsonRect {
            x: f64,
            y: f64,
            width: f64,
            height: f64,
        }

        #[derive(Serialize)]
        struct JsonEdge {
            from: String,
            to: String,
            points: Vec<JsonPoint>,
            label: Option<String>,
        }

        #[derive(Serialize)]
        struct JsonPoint {
            x: f64,
            y: f64,
        }

        let json_layout = JsonLayout {
            nodes: layout
                .nodes
                .iter()
                .map(|(id, rect)| {
                    (
                        id.clone(),
                        JsonRect { x: rect.origin.x, y: rect.origin.y, width: rect.size.width, height: rect.size.height },
                    )
                })
                .collect(),
            edges: layout
                .edges
                .iter()
                .map(|edge| JsonEdge {
                    from: edge.from.clone(),
                    to: edge.to.clone(),
                    points: edge.points.iter().map(|p| JsonPoint { x: p.x, y: p.y }).collect(),
                    label: edge.label.clone(),
                })
                .collect(),
        };

        serde_json::to_string_pretty(&json_layout)
            .map_err(|e| crate::Error::Serialization(format!("Failed to serialize layout to JSON: {}", e)))
    }
}
