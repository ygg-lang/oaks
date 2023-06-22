use crate::highlighter::{HighlightResult, HighlightSegment};
use std::{
    format,
    string::{String, ToString},
    vec::Vec,
};

/// Supported export formats for highlighted code.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExportFormat {
    /// HTML with optional CSS classes or inline styles.
    Html,
    /// CSS style definitions for highlighting.
    Css,
    /// JSON representation of highlighted segments.
    Json,
    /// XML representation of highlighted segments.
    Xml,
    /// ANSI escape codes for terminal color output.
    Ansi,
}

/// A trait for exporting [HighlightResult] into various string formats.
pub trait Exporter {
    /// Exports the highlighting result into a string representation.
    fn export(&self, result: &HighlightResult<'_>) -> String;
}

/// An exporter that generates HTML markup for highlighted code.
pub struct HtmlExporter {
    /// Whether to include a `<style>` block with default CSS classes.
    pub include_css: bool,
    /// Whether to use inline `style="..."` attributes instead of CSS classes.
    pub inline_styles: bool,
}

impl HtmlExporter {
    /// Creates a new [HtmlExporter] with the specified configuration.
    pub fn new(include_css: bool, inline_styles: bool) -> Self {
        Self { include_css, inline_styles }
    }
}

impl Exporter for HtmlExporter {
    fn export(&self, result: &HighlightResult<'_>) -> String {
        let mut html = String::new();

        if self.include_css {
            html.push_str("<style>\n");
            html.push_str(".highlight { font-family: 'Courier New', monospace; white-space: pre; }\n");
            html.push_str(".highlight .bold { font-weight: bold; }\n");
            html.push_str(".highlight .italic { font-style: italic; }\n");
            html.push_str(".highlight .underline { text-decoration: underline; }\n");
            html.push_str("</style>\n");
        }

        html.push_str("<div class=\"highlight\">");

        for segment in &result.segments {
            if self.inline_styles {
                html.push_str(&self.segment_to_html_inline(segment));
            }
            else {
                html.push_str(&self.segment_to_html_class(segment));
            }
        }

        html.push_str("</div>");
        html
    }
}

impl HtmlExporter {
    fn segment_to_html_inline(&self, segment: &HighlightSegment<'_>) -> String {
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

        if style_attrs.is_empty() { escaped_text } else { format!("<span style=\"{}\">{}</span>", style_attrs.join("; "), escaped_text) }
    }

    fn segment_to_html_class(&self, segment: &HighlightSegment<'_>) -> String {
        let escaped_text = html_escape(&segment.text);

        // Generate CSS class names based on style
        let mut classes = Vec::new();

        if segment.style.bold {
            classes.push("bold");
        }

        if segment.style.italic {
            classes.push("italic");
        }

        if segment.style.underline {
            classes.push("underline");
        }

        if classes.is_empty() { escaped_text } else { format!("<span class=\"{}\">{}</span>", classes.join(" "), escaped_text) }
    }
}

/// An exporter that generates CSS style definitions for highlighting classes.
pub struct CssExporter;

impl Exporter for CssExporter {
    fn export(&self, _result: &HighlightResult<'_>) -> String {
        let mut css = String::new();

        css.push_str(".highlight {\n");
        css.push_str("  font-family: 'Courier New', monospace;\n");
        css.push_str("  white-space: pre;\n");
        css.push_str("}\n\n");

        css.push_str(".highlight .bold { font-weight: bold; }\n");
        css.push_str(".highlight .italic { font-style: italic; }\n");
        css.push_str(".highlight .underline { text-decoration: underline; }\n");

        css
    }
}

/// An exporter that generates a JSON representation of the highlighting result.
pub struct JsonExporter {
    /// Whether to format the JSON output with indentation.
    pub pretty: bool,
}

impl Exporter for JsonExporter {
    fn export(&self, result: &HighlightResult<'_>) -> String {
        if self.pretty { serde_json::to_string_pretty(result).unwrap_or_default() } else { serde_json::to_string(result).unwrap_or_default() }
    }
}

/// An exporter that generates ANSI escape codes for terminal color output.
pub struct AnsiExporter;

impl Exporter for AnsiExporter {
    fn export(&self, result: &HighlightResult<'_>) -> String {
        let mut output = String::new();

        for segment in &result.segments {
            output.push_str(&self.segment_to_ansi(segment));
        }

        output
    }
}

impl AnsiExporter {
    fn segment_to_ansi(&self, segment: &HighlightSegment<'_>) -> String {
        let mut codes = Vec::new();

        // Reset
        codes.push("0");

        if segment.style.bold {
            codes.push("1");
        }

        if segment.style.italic {
            codes.push("3");
        }

        if segment.style.underline {
            codes.push("4");
        }

        // Basic color mapping
        if let Some(color) = &segment.style.color {
            match color.as_str() {
                "#FF0000" | "#F44747" | "#DC322F" | "#FF5555" => codes.push("31"), // Red
                "#00FF00" | "#6A9955" | "#859900" => codes.push("32"),             // Green
                "#FFFF00" | "#F1FA8C" | "#E6DB74" => codes.push("33"),             // Yellow
                "#0000FF" | "#569CD6" | "#005CC5" => codes.push("34"),             // Blue
                "#FF00FF" | "#FF79C6" | "#D73A49" => codes.push("35"),             // Magenta
                "#00FFFF" | "#9CDCFE" | "#2AA198" => codes.push("36"),             // Cyan
                _ => codes.push("37"),                                             // White
            }
        }

        if codes.len() > 1 { format!("\x1b[{}m{}\x1b[0m", codes.join(";"), segment.text) } else { segment.text.to_string() }
    }
}

/// HTML 转义
fn html_escape(text: &str) -> String {
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
