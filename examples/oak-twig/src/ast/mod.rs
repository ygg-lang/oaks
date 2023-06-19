use core::range::Range;

/// Twig 文档根节点
#[derive(Debug, Clone)]
pub struct TwigRoot {
    pub span: Range<usize>,
}

impl TwigRoot {
    pub fn new(span: Range<usize>) -> Self {
        Self { span }
    }
}
