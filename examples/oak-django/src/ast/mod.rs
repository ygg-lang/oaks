use core::range::Range;

/// Django 模板根节点
#[derive(Debug, Clone)]
pub struct DjangoRoot {
    pub elements: Vec<DjangoElement>,
    pub span: Range<usize>,
}

/// Django 模板元素
#[derive(Debug, Clone)]
pub enum DjangoElement {
    /// HTML 文本
    HtmlText { content: String, span: Range<usize> },
    /// Django 变量 {{ variable }}
    Variable { name: String, filters: Vec<String>, span: Range<usize> },
    /// Django 标签 {% tag %}
    Tag { name: String, args: Vec<String>, span: Range<usize> },
    /// Django 注释 {# comment #}
    Comment { content: String, span: Range<usize> },
}
