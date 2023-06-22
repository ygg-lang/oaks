use core::range::Range;
use serde::{Deserialize, Serialize};

/// TOML 文档根节
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TomlRoot {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub items: Vec<TomlItem>,
}

/// TOML 顶级项目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TomlItem {
    KeyValue(TomlKeyValue),
    Table(TomlTable),
    ArrayOfTables(TomlArrayOfTables),
}

/// TOML 键值对
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TomlKeyValue {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub key: TomlKey,
    pub value: TomlValue,
}

/// TOML
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TomlTable {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub header: TomlTableHeader,
    pub items: Vec<TomlKeyValue>,
}

/// TOML 表头
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TomlTableHeader {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub key: TomlKey,
}

/// TOML 表数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TomlArrayOfTables {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub header: TomlArrayOfTablesHeader,
    pub items: Vec<TomlKeyValue>,
}

/// TOML 表数组头
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TomlArrayOfTablesHeader {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub key: TomlKey,
}

/// TOML
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TomlKey {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub segments: Vec<TomlKeySegment>,
}

/// TOML 键段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TomlKeySegment {
    Bare(TomlBareKey),
    Quoted(TomlQuotedKey),
}

/// TOML 裸键
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TomlBareKey {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub name: String,
}

/// TOML 引用
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TomlQuotedKey {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub value: String,
}

/// TOML
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TomlString {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub value: String,
    pub is_multiline: bool,
    pub is_literal: bool,
}

/// TOML 整数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TomlInteger {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub value: i64,
    pub format: IntegerFormat,
}

/// 整数格式
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum IntegerFormat {
    Decimal,
    Hex,
    Octal,
    Binary,
}

/// TOML 浮点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TomlFloat {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub value: f64,
}

/// TOML 布尔
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TomlBoolean {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub value: bool,
}

/// TOML 日期时间
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TomlDateTime {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub value: String, // 简化处理
}

/// TOML 数组
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TomlArray {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub items: Vec<TomlValue>,
}

/// TOML 内联表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TomlInlineTable {
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
    pub items: Vec<TomlKeyValue>,
}

impl TomlTable {
    pub fn new(span: Range<usize>, header: TomlTableHeader) -> Self {
        Self { span, header, items: Vec::new() }
    }

    pub fn get(&self, key: &str) -> Option<&TomlValue> {
        for kv in &self.items {
            if kv.key.to_string() == key {
                return Some(&kv.value);
            }
        }
        None
    }
}

impl TomlKey {
    pub fn to_string(&self) -> String {
        self.segments
            .iter()
            .map(|s| match s {
                TomlKeySegment::Bare(b) => b.name.clone(),
                TomlKeySegment::Quoted(q) => q.value.clone(),
            })
            .collect::<Vec<_>>()
            .join(".")
    }
}

impl TomlValue {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            TomlValue::String(s) => Some(&s.value),
            _ => None,
        }
    }

    pub fn as_integer(&self) -> Option<i64> {
        match self {
            TomlValue::Integer(i) => Some(i.value),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            TomlValue::Boolean(b) => Some(b.value),
            _ => None,
        }
    }

    pub fn as_inline_table(&self) -> Option<&TomlInlineTable> {
        match self {
            TomlValue::InlineTable(t) => Some(t),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&TomlArray> {
        match self {
            TomlValue::Array(a) => Some(a),
            _ => None,
        }
    }
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
