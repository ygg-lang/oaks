use std::range::Range;

/// Typst AST 根节点
#[derive(Debug, Clone)]
pub struct TypstRoot {
    pub span: Range<usize>,
}

impl TypstRoot {
    pub fn new(span: Range<usize>) -> Self {
        Self { span }
    }
}
