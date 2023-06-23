#![doc = include_str!("readme.md")]
//! DHall Code Formatter

pub struct DHallFormatter {
    _indent_level: usize,
    _indent_str: String,
}

impl DHallFormatter {
    pub fn new() -> Self {
        Self { _indent_level: 0, _indent_str: "  ".to_string() }
    }

    pub fn format(&self, source: &str) -> String {
        // TODO: 实现真正的 DHall 格式化逻辑
        source.to_string()
    }
}
