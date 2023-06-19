use std::range::Range;

/// TypeScript AST 根节点
#[derive(Debug, Clone)]
pub struct TypeScriptRoot {
    pub span: Range<usize>,
}

impl TypeScriptRoot {
    pub fn new(span: Range<usize>) -> Self {
        Self { span }
    }
}
