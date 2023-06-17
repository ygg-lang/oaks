#![doc = include_str!("readme.md")]

pub mod geometry;
pub mod graph;
pub mod layout;
pub mod render;
pub mod theme;
pub mod tree;

// Re-export commonly used types
pub use geometry::{Point, Rect, Size, Transform};
pub use graph::{Graph, GraphEdge, GraphLayout, GraphLayoutAlgorithm, GraphLayoutConfig, GraphNode};
pub use layout::{EdgeType, Layout, LayoutConfig, LayoutEdge, LayoutEngine, LayoutNode, NodeType};
pub use render::{ElementStyle, ExportFormat, LayoutExporter, RenderConfig, SvgRenderer};
pub use theme::{ArrowConfig, EdgeTheme, HighlightTheme, NodeTheme, ShadowConfig, TextTheme, VisualizationTheme};
pub use tree::{TreeLayout, TreeLayoutAlgorithm, TreeLayoutConfig, TreeNode};

/// Result type for visualization operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for visualization operations
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Layout error: {0}")]
    Layout(String),

    #[error("Rendering error: {0}")]
    Rendering(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_config_default() {
        let config = LayoutConfig::default();
        assert_eq!(config.node_width, 100.0);
        assert_eq!(config.node_height, 60.0);
    }
}
