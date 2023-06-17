use oak_highlight::highlighter::Highlighter;

/// JSON 高亮
pub struct JsonHighlighter {
    use_parser: bool,
}

impl Highlighter for JsonHighlighter {}
