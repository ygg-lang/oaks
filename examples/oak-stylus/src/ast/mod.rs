use core::range::Range;

/// Stylus 文档根节点
#[derive(Debug, Clone)]
pub struct StylusRoot {
    pub span: Range<usize>,
    pub items: Vec<StylusItem>,
}

/// Stylus 顶级项目
#[derive(Debug, Clone)]
pub enum StylusItem {
    Rule(StylusRule),
    Comment(StylusComment),
}

/// Stylus 规则
#[derive(Debug, Clone)]
pub struct StylusRule {
    pub span: Range<usize>,
    pub selector: String,
    pub properties: Vec<StylusProperty>,
}

/// Stylus 注释
#[derive(Debug, Clone)]
pub struct StylusComment {
    pub span: Range<usize>,
    pub text: String,
}

/// Stylus 属性
#[derive(Debug, Clone)]
pub struct StylusProperty {
    pub span: Range<usize>,
    pub name: String,
    pub value: String,
}

impl StylusRoot {
    pub fn new(span: Range<usize>) -> Self {
        Self { span, items: Vec::new() }
    }
}
