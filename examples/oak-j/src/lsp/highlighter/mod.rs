#![doc = include_str!("readme.md")]
//! J 语法高亮器

/// 高亮类型的本地定义
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    /// 关键字 (J 中对应原始操作符)
    Keyword,
    /// 字符串
    String,
    /// 数字
    Number,
    /// 注释
    Comment,
    /// 标识符
    Identifier,
}

/// 高亮器 trait
pub trait Highlighter {
    /// 对给定的文本进行高亮处理
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
}

/// J 语法高亮器
pub struct JHighlighter {
    /// 是否使用基于解析器的高亮
    pub use_parser: bool,
}

impl Default for JHighlighter {
    fn default() -> Self {
        Self { use_parser: false }
    }
}

impl JHighlighter {
    /// 创建一个一个新的 J 高亮器实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 高亮 J 原始操作符 (对应其他语言的关键字)
    fn highlight_keywords(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        // J 的赋值符号
        for op in ["=:", "=."] {
            let mut start = 0;
            while let Some(pos) = text[start..].find(op) {
                highlights.push((start + pos, start + pos + op.len(), HighlightKind::Keyword));
                start += pos + op.len();
            }
        }
        highlights
    }
}

impl Highlighter for JHighlighter {
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = self.highlight_keywords(text);

        // 简单的正则表达式或手动扫描逻辑可以在这里添加
        // 为了保持简单，我们目前主要依赖词法分析器进行高亮

        highlights.sort_by_key(|h| h.0);
        highlights
    }
}

#[cfg(feature = "oak-highlight")]
impl oak_highlight::Highlighter for JHighlighter {
    fn highlight<'a>(&self, _source: &'a str, _language: &str, _theme: oak_highlight::Theme) -> oak_core::errors::ParseResult<oak_highlight::HighlightResult<'a>> {
        // TODO: Implement proper highlighting using the new oak-highlight API
        Ok(oak_highlight::HighlightResult { segments: Vec::new(), source: std::borrow::Cow::Borrowed(_source) })
    }
}
