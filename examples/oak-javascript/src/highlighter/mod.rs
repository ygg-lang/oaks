#[cfg(feature = "oak-highlight")]
use oak_highlight::highlighter::Highlighter;

/// JSON 高亮
#[cfg(feature = "oak-highlight")]
pub struct JsonHighlighter {
    use_parser: bool,
}

#[cfg(feature = "oak-highlight")]
impl Highlighter for JsonHighlighter {}
