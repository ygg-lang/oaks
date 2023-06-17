use alloc::{string::String, vec::Vec};
use core::range::Range;

/// Ini 文档根节
#[derive(Debug, Clone)]
pub struct IniRoot {
    pub span: Range<usize>,
    pub items: Vec<IniItem>,
}

/// Ini 顶级项目
#[derive(Debug, Clone)]
pub enum IniItem {
    KeyValue(IniKeyValue),
    Table(IniTable),
    ArrayOfTables(IniArrayOfTables),
}

/// Ini 键值对
#[derive(Debug, Clone)]
pub struct IniKeyValue {
    pub span: Range<usize>,
    pub key: IniKey,
    pub value: IniValue,
}

/// Ini
#[derive(Debug, Clone)]
pub struct IniTable {
    pub span: Range<usize>,
    pub header: IniTableHeader,
    pub items: Vec<IniKeyValue>,
}

/// Ini 表头
#[derive(Debug, Clone)]
pub struct IniTableHeader {
    pub span: Range<usize>,
    pub key: IniKey,
}

/// Ini 表数
#[derive(Debug, Clone)]
pub struct IniArrayOfTables {
    pub span: Range<usize>,
    pub header: IniArrayOfTablesHeader,
    pub items: Vec<IniKeyValue>,
}

/// Ini 表数组头
#[derive(Debug, Clone)]
pub struct IniArrayOfTablesHeader {
    pub span: Range<usize>,
    pub key: IniKey,
}

/// Ini
#[derive(Debug, Clone)]
pub struct IniKey {
    pub span: Range<usize>,
    pub segments: Vec<IniKeySegment>,
}

/// Ini 键段
#[derive(Debug, Clone)]
pub enum IniKeySegment {
    Bare(IniBareKey),
    Quoted(IniQuotedKey),
}

/// Ini 裸键
#[derive(Debug, Clone)]
pub struct IniBareKey {
    pub span: Range<usize>,
    pub name: String,
}

/// Ini 引用
#[derive(Debug, Clone)]
pub struct IniQuotedKey {
    pub span: Range<usize>,
    pub value: String,
}

/// Ini
#[derive(Debug, Clone)]
pub enum IniValue {
    String(IniString),
    Integer(IniInteger),
    Float(IniFloat),
    Boolean(IniBoolean),
    DateTime(IniDateTime),
    Array(IniArray),
    InlineTable(IniInlineTable),
}

/// Ini 字符
#[derive(Debug, Clone)]
pub struct IniString {
    pub span: Range<usize>,
    pub value: String,
    pub kind: IniStringKind,
}

/// Ini 字符串类
#[derive(Debug, Clone)]
pub enum IniStringKind {
    Basic,            // "string"
    Literal,          // 'string'
    MultilineBasic,   // """string"""
    MultilineLiteral, // '''string'''
}

/// Ini 整数
#[derive(Debug, Clone)]
pub struct IniInteger {
    pub span: Range<usize>,
    pub value: i64,
}

/// Ini 浮点
#[derive(Debug, Clone)]
pub struct IniFloat {
    pub span: Range<usize>,
    pub value: f64,
}

/// Ini 布尔
#[derive(Debug, Clone)]
pub struct IniBoolean {
    pub span: Range<usize>,
    pub value: bool,
}

/// Ini 日期时间
#[derive(Debug, Clone)]
pub struct IniDateTime {
    pub span: Range<usize>,
    pub value: String, // 暂时用字符串表示，后续可以用专门的日期时间类
}

/// Ini 数组
#[derive(Debug, Clone)]
pub struct IniArray {
    pub span: Range<usize>,
    pub elements: Vec<IniValue>,
}

/// Ini 内联
#[derive(Debug, Clone)]
pub struct IniInlineTable {
    pub span: Range<usize>,
    pub pairs: Vec<IniKeyValue>,
}

impl IniRoot {
    pub fn new(span: Range<usize>) -> Self {
        Self { span, items: Vec::new() }
    }
}

impl IniValue {
    /// 获取值的跨度
    pub fn span(&self) -> Range<usize> {
        match self {
            IniValue::String(s) => s.span.clone(),
            IniValue::Integer(i) => i.span.clone(),
            IniValue::Float(f) => f.span.clone(),
            IniValue::Boolean(b) => b.span.clone(),
            IniValue::DateTime(dt) => dt.span.clone(),
            IniValue::Array(a) => a.span.clone(),
            IniValue::InlineTable(t) => t.span.clone(),
        }
    }
}
