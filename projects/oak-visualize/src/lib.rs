#![doc = include_str!("readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

use std::fmt;

pub mod geometry;
pub mod graph;
pub mod layout;
pub mod render;
pub mod theme;
pub mod tree;

/// Error type for oak-visualize operations
#[derive(Debug)]
pub enum Error {
    /// Layout computation error
    LayoutError(String),
    /// Rendering error
    RenderError(String),
    /// Serialization error
    Serialization(String),
    /// IO error
    IoError(std::io::Error),
    /// Generic error
    Generic(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::LayoutError(msg) => write!(f, "Layout error: {}", msg),
            Error::RenderError(msg) => write!(f, "Render error: {}", msg),
            Error::Serialization(msg) => write!(f, "Serialization error: {}", msg),
            Error::IoError(err) => write!(f, "IO error: {}", err),
            Error::Generic(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

/// Result type alias for oak-visualize operations
pub type Result<T> = std::result::Result<T, Error>;

// Re-export commonly used types
pub use crate::{
    geometry::{Point, Rect, Size, Transform},
    graph::{Graph, GraphEdge, GraphLayout, GraphLayoutAlgorithm, GraphLayoutConfig, GraphNode},
    layout::{EdgeType, Layout, LayoutConfig, LayoutEdge, LayoutEngine, LayoutNode, NodeType},
    render::{ElementStyle, ExportFormat, LayoutExporter, RenderConfig, SvgRenderer},
    theme::{ArrowConfig, EdgeTheme, HighlightTheme, NodeTheme, ShadowConfig, TextTheme, VisualizationTheme},
    tree::{TreeLayout, TreeLayoutAlgorithm, TreeLayoutConfig, TreeNode},
};
