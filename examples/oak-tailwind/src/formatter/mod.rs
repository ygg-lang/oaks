use oak_core::SourceText;

/// Tailwind 语言的格式化器
pub struct TailwindFormatter;

impl TailwindFormatter {
    pub fn format(&self, source: &SourceText, _indent: usize) -> String {
        // TODO: 实现 Tailwind 格式化逻辑
        source.text().to_string()
    }
}
