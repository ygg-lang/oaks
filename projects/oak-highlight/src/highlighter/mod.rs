use std::{
    range::Range,
    string::{String, ToString},
    vec::Vec,
};

/// Highlight style configuration for visual text formatting.
///
/// This struct defines the visual appearance of highlighted text segments,
/// including colors, font weight, and text decorations.
#[derive(Debug, Clone, PartialEq)]
pub struct HighlightStyle {
    /// Foreground text color in hex format (e.g., "#FF0000" for red)
    pub color: Option<String>,
    /// Background color in hex format (e.g., "#FFFF00" for yellow)
    pub background_color: Option<String>,
    /// Whether text should be displayed in bold
    pub bold: bool,
    /// Whether text should be displayed in italic
    pub italic: bool,
    /// Whether text should be underlined
    pub underline: bool,
}

impl Default for HighlightStyle {
    fn default() -> Self {
        Self { color: None, background_color: None, bold: false, italic: false, underline: false }
    }
}

/// Highlight theme configuration containing style definitions for different token types.
///
/// A theme defines the visual appearance for various programming language constructs
/// such as keywords, strings, comments, and operators.
#[derive(Debug, Clone)]
pub struct HighlightTheme {
    /// Theme name identifier
    pub name: String,
    /// Style for language keywords (if, while, return, etc.)
    pub keyword: HighlightStyle,
    /// Style for string literals
    pub string: HighlightStyle,
    /// Style for numeric literals
    pub number: HighlightStyle,
    /// Style for comments
    pub comment: HighlightStyle,
    /// Style for identifiers (variable names, function names, etc.)
    pub identifier: HighlightStyle,
    /// Style for operators (+, -, *, /, etc.)
    pub operator: HighlightStyle,
    /// Style for delimiters (parentheses, brackets, braces, etc.)
    pub delimiter: HighlightStyle,
    /// Style for error tokens or invalid kind
    pub error: HighlightStyle,
}

impl Default for HighlightTheme {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            keyword: HighlightStyle { color: Some("#0000FF".to_string()), bold: true, ..Default::default() },
            string: HighlightStyle { color: Some("#008000".to_string()), ..Default::default() },
            number: HighlightStyle { color: Some("#FF6600".to_string()), ..Default::default() },
            comment: HighlightStyle { color: Some("#808080".to_string()), italic: true, ..Default::default() },
            identifier: HighlightStyle::default(),
            operator: HighlightStyle { color: Some("#800080".to_string()), ..Default::default() },
            delimiter: HighlightStyle { color: Some("#000080".to_string()), ..Default::default() },
            error: HighlightStyle {
                color: Some("#FF0000".to_string()),
                background_color: Some("#FFCCCC".to_string()),
                ..Default::default()
            },
        }
    }
}

/// A segment of highlighted text with associated style and content.
///
/// Represents a contiguous range of text that shares the same highlighting style.
#[derive(Debug, Clone)]
pub struct HighlightSegment {
    /// Byte range in the source text that this segment covers
    pub span: Range<usize>,
    /// Visual style to apply to this text segment
    pub style: HighlightStyle,
    /// The actual text content of this segment
    pub text: String,
}

/// Result of kind highlighting containing styled text segments.
///
/// This struct holds the complete highlighted representation of source code,
/// broken down into segments with their associated styles.
#[derive(Debug, Clone)]
pub struct HighlightResult {
    /// Vector of highlighted text segments in order
    pub segments: Vec<HighlightSegment>,
    /// The original source text that was highlighted
    pub source: String,
}

/// Base trait for kind highlighters.
///
/// This trait defines the interface for kind highlighting implementations
/// that can analyze source code and produce styled text segments.
pub trait Highlighter {}

/// HTML 导出器
pub struct HtmlExporter {
    include_css: bool,
}

impl HtmlExporter {
    pub fn new(include_css: bool) -> Self {
        Self { include_css }
    }

    /// 导出为 HTML
    pub fn export(&self, result: &HighlightResult) -> String {
        let mut html = String::new();

        if self.include_css {
            html.push_str("<style>\n");
            html.push_str(".highlight { font-family: 'Courier New', monospace; }\n");
            html.push_str("</style>\n");
        }

        html.push_str("<div class=\"highlight\">");

        for segment in &result.segments {
            html.push_str(&self.segment_to_html(segment));
        }

        html.push_str("</div>");
        html
    }

    fn segment_to_html(&self, segment: &HighlightSegment) -> String {
        let mut style_attrs = Vec::new();

        if let Some(color) = &segment.style.color {
            style_attrs.push(format!("color: {}", color));
        }

        if let Some(bg_color) = &segment.style.background_color {
            style_attrs.push(format!("background-color: {}", bg_color));
        }

        if segment.style.bold {
            style_attrs.push("font-weight: bold".to_string());
        }

        if segment.style.italic {
            style_attrs.push("font-style: italic".to_string());
        }

        if segment.style.underline {
            style_attrs.push("text-decoration: underline".to_string());
        }

        let escaped_text = html_escape(&segment.text);

        if style_attrs.is_empty() {
            escaped_text
        }
        else {
            format!("<span style=\"{}\">{}</span>", style_attrs.join("; "), escaped_text)
        }
    }
}

/// HTML 转义
fn html_escape(text: &str) -> String {
    use std::string::ToString;
    text.chars()
        .map(|c| match c {
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            '&' => "&amp;".to_string(),
            '"' => "&quot;".to_string(),
            '\'' => "&#39;".to_string(),
            _ => c.to_string(),
        })
        .collect()
}
