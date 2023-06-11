use crate::highlighter::{HighlightResult, HighlightSegment};
use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};

/// 导出格式
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExportFormat {
    Html,
    Css,
    Json,
    Xml,
    Ansi,
}

/// 导出器特征
pub trait Exporter {
    fn export(&self, result: &HighlightResult) -> String;
}

/// HTML 导出器
pub struct HtmlExporter {
    pub include_css: bool,
    pub inline_styles: bool,
}

impl HtmlExporter {
    pub fn new(include_css: bool, inline_styles: bool) -> Self {
        Self { include_css, inline_styles }
    }
}

impl Exporter for HtmlExporter {
    fn export(&self, result: &HighlightResult) -> String {
        let mut html = String::new();

        if self.include_css {
            html.push_str("<style>\n");
            html.push_str(".highlight { font-family: 'Courier New', monospace; white-space: pre; }\n");
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
    fn segment_to_html_inline(&self, segment: &HighlightSegment) -> String {
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

    fn segment_to_html_class(&self, segment: &HighlightSegment) -> String {
        let escaped_text = html_escape(&segment.text);

        // 根据样式生成 CSS 类名
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

/// CSS 导出器
pub struct CssExporter;

impl Exporter for CssExporter {
    fn export(&self, _result: &HighlightResult) -> String {
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

/// JSON 导出器
pub struct JsonExporter;

impl Exporter for JsonExporter {
    fn export(&self, result: &HighlightResult) -> String {
        let mut json = String::new();

        json.push_str("{\n");
        json.push_str("  \"source\": ");
        json.push_str(&json_escape(&result.source));
        json.push_str(",\n");
        json.push_str("  \"segments\": [\n");

        for (i, segment) in result.segments.iter().enumerate() {
            if i > 0 {
                json.push_str(",\n");
            }
            json.push_str("    {\n");
            json.push_str(&format!("      \"start\": {},\n", segment.span.start));
            json.push_str(&format!("      \"end\": {},\n", segment.span.end));
            json.push_str("      \"text\": ");
            json.push_str(&json_escape(&segment.text));
            json.push_str(",\n");
            json.push_str("      \"style\": {\n");

            if let Some(color) = &segment.style.color {
                json.push_str("        \"color\": ");
                json.push_str(&json_escape(color));
                json.push_str(",\n");
            }

            if let Some(bg_color) = &segment.style.background_color {
                json.push_str("        \"backgroundColor\": ");
                json.push_str(&json_escape(bg_color));
                json.push_str(",\n");
            }

            json.push_str(&format!("        \"bold\": {},\n", segment.style.bold));
            json.push_str(&format!("        \"italic\": {},\n", segment.style.italic));
            json.push_str(&format!("        \"underline\": {}\n", segment.style.underline));
            json.push_str("      }\n");
            json.push_str("    }");
        }

        json.push_str("\n  ]\n");
        json.push_str("}\n");
        json
    }
}

/// ANSI 导出器（终端颜色）
pub struct AnsiExporter;

impl Exporter for AnsiExporter {
    fn export(&self, result: &HighlightResult) -> String {
        let mut output = String::new();

        for segment in &result.segments {
            output.push_str(&self.segment_to_ansi(segment));
        }

        output
    }
}

impl AnsiExporter {
    fn segment_to_ansi(&self, segment: &HighlightSegment) -> String {
        let mut codes = Vec::new();

        // 重置
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

        // 简单的颜色映射
        if let Some(color) = &segment.style.color {
            match color.as_str() {
                "#FF0000" | "#F44747" | "#DC322F" | "#FF5555" => codes.push("31"), // 红色
                "#00FF00" | "#6A9955" | "#859900" => codes.push("32"),             // 绿色
                "#FFFF00" | "#F1FA8C" | "#E6DB74" => codes.push("33"),             // 黄色
                "#0000FF" | "#569CD6" | "#005CC5" => codes.push("34"),             // 蓝色
                "#FF00FF" | "#FF79C6" | "#D73A49" => codes.push("35"),             // 洋红
                "#00FFFF" | "#9CDCFE" | "#2AA198" => codes.push("36"),             // 青色
                _ => codes.push("37"),                                             // 白色
            }
        }

        if codes.len() > 1 { format!("\x1b[{}m{}\x1b[0m", codes.join(";"), segment.text) } else { segment.text.clone() }
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

/// JSON 转义
fn json_escape(text: &str) -> String {
    let mut result = String::new();
    result.push('"');

    for c in text.chars() {
        match c {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            c if c.is_control() => {
                result.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => result.push(c),
        }
    }

    result.push('"');
    result
}
