//! LLVM IR 语法高亮器

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
    /// 本地变量
    LocalVar,
    /// 全局变量
    GlobalVar,
    /// 元数据
    Metadata,
    /// 标识符
    Identifier,
}

/// 高亮器 trait
pub trait Highlighter {
    /// 对给定的文本进行高亮处理
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
}

pub struct LlirHighlighter;

impl LlirHighlighter {
    pub fn new() -> Self {
        Self
    }
}

impl Highlighter for LlirHighlighter {
    fn highlight(&self, _text: &str) -> Vec<(usize, usize, HighlightKind)> {
        // TODO: 实现高亮
        Vec::new()
    }
}
