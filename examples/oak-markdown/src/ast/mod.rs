use core::range::Range;
use serde::{Deserialize, Serialize};

/// Markdown 抽象语法树的根节点
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MarkdownRoot {
    /// 文档中的块列表
    pub blocks: Vec<Block>,
}

/// Markdown 中的块级元素
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Block {
    /// 标题 (h1-h6)
    Heading(Heading),
    /// 段落
    Paragraph(Paragraph),
    /// 代码块
    CodeBlock(CodeBlock),
    /// 列表
    List(List),
    /// 引用
    Blockquote(Blockquote),
    /// 水平分割线
    HorizontalRule(HorizontalRule),
    /// 表格
    Table(Table),
    /// HTML 块
    Html(Html),
}

/// 标题
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Heading {
    /// 标题级别 (1-6)
    pub level: u32,
    /// 标题内容
    pub content: String,
    /// 源代码范围
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// 段落
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Paragraph {
    /// 段落内容
    pub content: String,
    /// 源代码范围
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// 代码块
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CodeBlock {
    /// 编程语言标识符
    pub language: Option<String>,
    /// 代码内容
    pub content: String,
    /// 源代码范围
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// 列表
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct List {
    /// 是否为有序列表
    pub is_ordered: bool,
    /// 列表项
    pub items: Vec<ListItem>,
    /// 源代码范围
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// 列表项
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ListItem {
    /// 列表项内容
    pub content: Vec<Block>,
    /// 是否为任务列表项
    pub is_task: bool,
    /// 任务完成状态 (如果 is_task 为 true)
    pub is_checked: Option<bool>,
    /// 源代码范围
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// 引用
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Blockquote {
    /// 引用内容
    pub content: Vec<Block>,
    /// 源代码范围
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// 水平分割线
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HorizontalRule {
    /// 源代码范围
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// 表格
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Table {
    /// 表头
    pub header: TableRow,
    /// 表行
    pub rows: Vec<TableRow>,
    /// 源代码范围
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// 表格行
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TableRow {
    /// 单元格列表
    pub cells: Vec<TableCell>,
    /// 源代码范围
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// 表格单元格
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TableCell {
    /// 单元格内容
    pub content: String,
    /// 源代码范围
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// HTML 块
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Html {
    /// HTML 内容
    pub content: String,
    /// 源代码范围
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}
