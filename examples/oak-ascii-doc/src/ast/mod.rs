use std::{range::Range, string::String, vec::Vec};

/// AsciiDoc 文档的根节点
#[derive(Debug, PartialEq, Clone)]
pub struct SourceFile {
    pub elements: Vec<Element>,
}

/// AsciiDoc 文档元素
#[derive(Debug, PartialEq, Clone)]
pub enum Element {
    Header { level: u8, text: String, span: Range<usize> },
    Text { content: String, span: Range<usize> },
    Bold { content: String, span: Range<usize> },
    Italic { content: String, span: Range<usize> },
    Monospace { content: String, span: Range<usize> },
    CodeBlock { content: String, span: Range<usize> },
    Link { url: String, text: Option<String>, span: Range<usize> },
    ListItem { content: String, span: Range<usize> },
    Comment { content: String, span: Range<usize> },
}
