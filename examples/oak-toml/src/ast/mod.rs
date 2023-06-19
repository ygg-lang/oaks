use core::range::Range;

/// TOML 文档根节
#[derive(Debug, Clone)]
pub struct TomlRoot {
    pub span: Range<usize>,
    pub items: Vec<TomlItem>,
}

/// TOML 顶级项目
#[derive(Debug, Clone)]
pub enum TomlItem {
    KeyValue(TomlKeyValue),
    Table(TomlTable),
    ArrayOfTables(TomlArrayOfTables),
}

/// TOML 键值对
#[derive(Debug, Clone)]
pub struct TomlKeyValue {
    pub span: Range<usize>,
    pub key: TomlKey,
    pub value: TomlValue,
}

/// TOML
#[derive(Debug, Clone)]
pub struct TomlTable {
    pub span: Range<usize>,
    pub header: TomlTableHeader,
    pub items: Vec<TomlKeyValue>,
}

/// TOML 表头
#[derive(Debug, Clone)]
pub struct TomlTableHeader {
    pub span: Range<usize>,
    pub key: TomlKey,
}

/// TOML 表数
#[derive(Debug, Clone)]
pub struct TomlArrayOfTables {
    pub span: Range<usize>,
    pub header: TomlArrayOfTablesHeader,
    pub items: Vec<TomlKeyValue>,
}

/// TOML 表数组头
#[derive(Debug, Clone)]
pub struct TomlArrayOfTablesHeader {
    pub span: Range<usize>,
    pub key: TomlKey,
}

/// TOML
#[derive(Debug, Clone)]
pub struct TomlKey {
    pub span: Range<usize>,
    pub segments: Vec<TomlKeySegment>,
}

/// TOML 键段
#[derive(Debug, Clone)]
pub enum TomlKeySegment {
    Bare(TomlBareKey),
    Quoted(TomlQuotedKey),
}

/// TOML 裸键
#[derive(Debug, Clone)]
pub struct TomlBareKey {
    pub span: Range<usize>,
    pub name: String,
}

/// TOML 引用
#[derive(Debug, Clone)]
pub struct TomlQuotedKey {
    pub span: Range<usize>,
    pub value: String,
}

/// TOML
#[derive(Debug, Clone)]
pub enum TomlValue {
    String(TomlString),
    Integer(TomlInteger),
    Float(TomlFloat),
    Boolean(TomlBoolean),
    DateTime(TomlDateTime),
    Array(TomlArray),
    InlineTable(TomlInlineTable),
}

/// TOML 字符
#[derive(Debug, Clone)]
pub struct TomlString {
    pub span: Range<usize>,
    pub value: String,
    pub kind: TomlStringKind,
}

/// TOML 字符串类
#[derive(Debug, Clone)]
pub enum TomlStringKind {
    Basic,            // "string"
    Literal,          // 'string'
    MultilineBasic,   // """string"""
    MultilineLiteral, // '''string'''
}

/// TOML 整数
#[derive(Debug, Clone)]
pub struct TomlInteger {
    pub span: Range<usize>,
    pub value: i64,
}

/// TOML 浮点
#[derive(Debug, Clone)]
pub struct TomlFloat {
    pub span: Range<usize>,
    pub value: f64,
}

/// TOML 布尔
#[derive(Debug, Clone)]
pub struct TomlBoolean {
    pub span: Range<usize>,
    pub value: bool,
}

/// TOML 日期时间
#[derive(Debug, Clone)]
pub struct TomlDateTime {
    pub span: Range<usize>,
    pub value: String, // 暂时用字符串表示，后续可以用专门的日期时间类
}

/// TOML 数组
#[derive(Debug, Clone)]
pub struct TomlArray {
    pub span: Range<usize>,
    pub elements: Vec<TomlValue>,
}

/// TOML 内联
#[derive(Debug, Clone)]
pub struct TomlInlineTable {
    pub span: Range<usize>,
    pub pairs: Vec<TomlKeyValue>,
}

impl TomlRoot {
    pub fn new(span: Range<usize>) -> Self {
        Self { span, items: Vec::new() }
    }
}

impl TomlValue {
    /// 获取值的跨度
    pub fn span(&self) -> Range<usize> {
        match self {
            TomlValue::String(s) => s.span.clone(),
            TomlValue::Integer(i) => i.span.clone(),
            TomlValue::Float(f) => f.span.clone(),
            TomlValue::Boolean(b) => b.span.clone(),
            TomlValue::DateTime(dt) => dt.span.clone(),
            TomlValue::Array(a) => a.span.clone(),
            TomlValue::InlineTable(t) => t.span.clone(),
        }
    }
}
