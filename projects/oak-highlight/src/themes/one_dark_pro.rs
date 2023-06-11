use crate::highlighter::{HighlightStyle, HighlightTheme};
use alloc::string::{String, ToString};

impl HighlightTheme {
    /// One Dark Pro 主题 - 基于 Atom One Dark Pro 配色方案
    pub fn one_dark_pro() -> Self {
        Self {
            name: "One Dark Pro".to_string(),
            keyword: HighlightStyle {
                color: Some("#C678DD".to_string()), // Purple - 关键字
                bold: true,
                ..Default::default()
            },
            string: HighlightStyle {
                color: Some("#98C379".to_string()), // Green - 字符串
                ..Default::default()
            },
            number: HighlightStyle {
                color: Some("#D19A66".to_string()), // Orange - 数字
                ..Default::default()
            },
            comment: HighlightStyle {
                color: Some("#5C6370".to_string()), // Gray - 注释
                italic: true,
                ..Default::default()
            },
            identifier: HighlightStyle {
                color: Some("#ABB2BF".to_string()), // Light Gray - 标识符
                ..Default::default()
            },
            operator: HighlightStyle {
                color: Some("#56B6C2".to_string()), // Cyan - 操作符
                ..Default::default()
            },
            delimiter: HighlightStyle {
                color: Some("#ABB2BF".to_string()), // Light Gray - 分隔符
                ..Default::default()
            },
            error: HighlightStyle {
                color: Some("#E06C75".to_string()),            // Red - 错误
                background_color: Some("#3E2723".to_string()), // Dark Red Background
                ..Default::default()
            },
        }
    }
}
