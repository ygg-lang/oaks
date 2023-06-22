use crate::ast::TypstRoot;

/// Typst 语言的格式化器
pub struct TypstFormatter;

impl TypstFormatter {
    pub fn new() -> Self {
        Self
    }

    pub fn format(&self, _root: &TypstRoot) -> String {
        // TODO: 实现具体的格式化逻辑
        String::new()
    }
}
