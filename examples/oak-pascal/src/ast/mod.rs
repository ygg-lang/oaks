use std::range::Range;

/// Pascal AST 根节点
#[derive(Debug, Clone)]
pub struct PascalRoot {
    pub items: Vec<PascalItem>,
    pub range: Range<usize>,
}

/// Pascal 项目
#[derive(Debug, Clone)]
pub enum PascalItem {
    Program { name: String, range: Range<usize> },
    Declaration { content: String, range: Range<usize> },
    Statement { content: String, range: Range<usize> },
}
