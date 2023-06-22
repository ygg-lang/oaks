use crate::kind::JsonSyntaxKind;

/// 高亮类型的本地定义
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    /// 关键字
    Keyword,
    /// 字符串
    String,
    /// 数字
    Number,
    /// 注释
    Comment,
    /// 标识符
    Identifier,
    /// 字面量 (null, true, false)
    Literal,
}

/// 高亮器 trait
pub trait Highlighter {
    /// 对给定的文本进行高亮处理
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
}

/// JSON 语法高亮器
pub struct JsonHighlighter;

impl JsonHighlighter {
    pub fn new() -> Self {
        Self
    }
}

impl Highlighter for JsonHighlighter {
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        // TODO: 实现基于词法分析的高亮
        Vec::new()
    }
}
