use oak_core::SourceText;

/// Twig 语言的格式化器
pub struct TwigFormatter;

impl TwigFormatter {
    pub fn format(&self, source: &SourceText, _indent: usize) -> String {
        // TODO: 实现 Twig 格式化逻辑
        source.text().to_string()
    }
}
