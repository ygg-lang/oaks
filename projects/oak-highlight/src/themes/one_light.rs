use crate::highlighter::{HighlightStyle, HighlightTheme};
use alloc::string::{String, ToString};

impl HighlightTheme {
    /// One Light 主题 - 基于 Atom One Light 配色方案
    pub fn one_light() -> Self {
        Self {
            name: "One Light".to_string(),
            keyword: HighlightStyle {
                color: Some("#A626A4".to_string()), // Purple - 关键字
                bold: true,
                ..Default::default()
            },
            string: HighlightStyle {
                color: Some("#50A14F".to_string()), // Green - 字符串
                ..Default::default()
            },
            number: HighlightStyle {
                color: Some("#986801".to_string()), // Orange - 数字
                ..Default::default()
            },
            comment: HighlightStyle {
                color: Some("#A0A1A7".to_string()), // Light Gray - 注释
                italic: true,
                ..Default::default()
            },
            identifier: HighlightStyle {
                color: Some("#383A42".to_string()), // Dark Gray - 标识符
                ..Default::default()
            },
            operator: HighlightStyle {
                color: Some("#0184BC".to_string()), // Blue - 操作符
                ..Default::default()
            },
            delimiter: HighlightStyle {
                color: Some("#383A42".to_string()), // Dark Gray - 分隔符
                ..Default::default()
            },
            error: HighlightStyle {
                color: Some("#E45649".to_string()),            // Red - 错误
                background_color: Some("#FFEAEA".to_string()), // Light Red Background
                ..Default::default()
            },
        }
    }
}
