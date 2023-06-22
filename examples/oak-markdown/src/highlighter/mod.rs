//! Markdown 语法高亮器
//!
//! 这个模块提供了 Markdown 源代码的语法高亮功能，支持标题、强调、代码块、链接等的高亮显示。

/// 高亮类型的本地定义
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    /// 标题
    Heading,
    /// 强调 (斜体)
    Emphasis,
    /// 加粗
    Strong,
    /// 代码块或行内代码
    Code,
    /// 链接
    Link,
    /// 列表标记
    ListMarker,
    /// 引用标记
    BlockquoteMarker,
    /// 注释
    Comment,
}

/// 高亮器 trait (如果以后有通用的 Highlighter trait，可以在这里继承)
pub trait Highlighter {
    /// 对给定的文本进行高亮处理
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
}

/// Markdown 语法高亮器
pub struct MarkdownHighlighter;

impl Default for MarkdownHighlighter {
    fn default() -> Self {
        Self
    }
}

impl MarkdownHighlighter {
    /// 创建一个新的 Markdown 高亮器实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 高亮标题
    fn highlight_headings(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        for line in text.lines() {
            let trimmed = line.trim_start();
            if trimmed.starts_with('#') {
                let pos = text.find(line).unwrap();
                highlights.push((pos, pos + line.len(), HighlightKind::Heading));
            }
        }
        highlights
    }

    /// 高亮代码块
    fn highlight_code_blocks(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();
        let mut in_code_block = false;
        let mut start_pos = 0;

        for line in text.lines() {
            let pos = text.find(line).unwrap();
            if line.trim_start().starts_with("```") || line.trim_start().starts_with("~~~") {
                if in_code_block {
                    highlights.push((start_pos, pos + line.len(), HighlightKind::Code));
                    in_code_block = false;
                }
                else {
                    start_pos = pos;
                    in_code_block = true;
                }
            }
        }
        highlights
    }
}

impl Highlighter for MarkdownHighlighter {
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();

        highlights.extend(self.highlight_headings(text));
        highlights.extend(self.highlight_code_blocks(text));

        // 按位置排序
        highlights.sort_by_key(|&(start, _, _)| start);
        highlights
    }
}
