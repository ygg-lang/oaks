#![doc = include_str!("readme.md")]
//! APL 语法高亮器

/// 高亮类型的本地定义
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    /// 关键字 (APL 中通常是特殊符号)
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

/// APL 语法高亮器
pub struct AplHighlighter {
    /// 是否使用基于解析器的高亮
    pub use_parser: bool,
}

impl Default for AplHighlighter {
    fn default() -> Self {
        Self { use_parser: false }
    }
}

impl AplHighlighter {
    /// 创建一个新的 APL 高亮器实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 高亮 APL 符号 (作为关键字)
    fn highlight_symbols(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let symbols = ['←', '→', '⋄', '⎕', '⍞', '⍴', '⍳', '⍬', '⍣', '⍤', '⍛', '⍢', '⌸', '⌺', '⌼', '⍠', '⌻', '⍃', '⍄', '⍈', '⍐', '⍗', '⍇', '⍈', '⍌', '⍍', '⍏', '⍖'];

        for (i, ch) in text.char_indices() {
            if symbols.contains(&ch) {
                highlights.push((i, i + ch.len_utf8(), HighlightKind::Keyword));
            }
        }
        highlights
    }
}

impl Highlighter for AplHighlighter {
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        // 简单实现：仅高亮符号
        self.highlight_symbols(text)
    }
}
