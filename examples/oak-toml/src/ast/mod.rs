#![doc = include_str!("readme.md")]
use core::range::Range;

/// TOML document root node.
///
/// Represents the entire TOML document, consisting of a list of top-level items.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TomlRoot {
    /// Range of the node in the source code.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// The items in the TOML document.
    pub items: Vec<TomlItem>,
}

/// TOML top-level item.
///
/// Can be a key-value pair, a standard table, or an array of tables.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TomlItem {
    /// A key-value pair.
    KeyValue(TomlKeyValue),
    /// A standard table `[table]`.
    Table(TomlTable),
    /// An array of tables `[[table]]`.
    ArrayOfTables(TomlArrayOfTables),
}

/// TOML key-value pair.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TomlKeyValue {
    /// Range of the key-value pair in the source code.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// The key part.
    pub key: TomlKey,
    /// The value part.
    pub value: TomlValueNode,
}

/// TOML table `[table]`.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TomlTable {
    /// Range of the table in the source code.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// Table header.
    pub header: TomlTableHeader,
    /// Key-value pairs within the table.
    pub items: Vec<TomlKeyValue>,
}

/// TOML table header `[key]`.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TomlTableHeader {
    /// Range of the header in the source code.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// The key of the table header.
    pub key: TomlKey,
}

/// TOML array of tables `[[table]]`.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TomlArrayOfTables {
    /// Range of the array of tables in the source code.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// Header for the array of tables.
    pub header: TomlArrayOfTablesHeader,
    /// Key-value pairs within each table entry.
    pub items: Vec<TomlKeyValue>,
}

/// TOML array of tables header `[[key]]`.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TomlArrayOfTablesHeader {
    /// Range of the header in the source code.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// The key of the array of tables header.
    pub key: TomlKey,
}

/// TOML key.
///
/// Supports dotted composite keys (e.g., `a.b.c`).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TomlKey {
    /// Range of the key in the source code.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// Segments of the key.
    pub segments: Vec<TomlKeySegment>,
}

/// TOML key segment.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TomlKeySegment {
    /// A bare key (unquoted).
    Bare(TomlBareKey),
    /// A quoted key.
    Quoted(TomlQuotedKey),
}

/// TOML bare key (unquoted).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TomlBareKey {
    /// Range of the key in the source code.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// Name of the key.
    pub name: String,
}

/// TOML quoted key.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TomlQuotedKey {
    /// Range of the key in the source code.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// Value of the key.
    pub value: String,
}

/// TOML value.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TomlValueNode {
    /// String value.
    String(TomlString),
    /// Integer value.
    Integer(TomlInteger),
    /// Floating-point value.
    Float(TomlFloat),
    /// Boolean value.
    Boolean(TomlBoolean),
    /// Date-time value.
    DateTime(TomlDateTime),
    /// Array value.
    Array(TomlArray),
    /// Inline table `{ a = 1, b = 2 }`.
    InlineTable(TomlInlineTable),
}

/// TOML string.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TomlString {
    /// Range of the string in the source code.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// Value of the string.
    pub value: String,
    /// Whether it's a multi-line string.
    pub is_multiline: bool,
    /// Whether it's a literal string (using single quotes).
    pub is_literal: bool,
}

/// TOML integer.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TomlInteger {
    /// Range of the integer in the source code.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// Value of the integer.
    pub value: i64,
    /// Format of the integer (decimal, hex, etc.).
    pub format: IntegerFormat,
}

/// Integer format.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum IntegerFormat {
    /// Decimal format.
    Decimal,
    /// Hexadecimal format.
    Hex,
    /// Octal format.
    Octal,
    /// Binary format.
    Binary,
}

/// TOML floating-point number.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TomlFloat {
    /// Range of the float in the source code.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// Value of the float.
    pub value: f64,
}

/// TOML boolean value.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TomlBoolean {
    /// Range of the boolean in the source code.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// The boolean value.
    pub value: bool,
}

/// TOML date-time.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TomlDateTime {
    /// Range of the date-time in the source code.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// String representation of the date-time.
    pub value: String,
}

/// TOML array.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TomlArray {
    /// Range of the array in the source code.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// List of array items.
    pub items: Vec<TomlValueNode>,
}

/// TOML inline table `{ a = 1, b = 2 }`.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TomlInlineTable {
    /// Range of the inline table in the source code.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// Key-value pairs in the inline table.
    pub items: Vec<TomlKeyValue>,
}

impl TomlTable {
    /// Creates a new TOML table.
    pub fn new(span: Range<usize>, header: TomlTableHeader) -> Self {
        Self { span, header, items: Vec::new() }
    }

    /// Gets a value from the table by key name.
    pub fn get(&self, key: &str) -> Option<&TomlValueNode> {
        for kv in &self.items {
            if kv.key.to_string() == key {
                return Some(&kv.value);
            }
        }
        None
    }
}

impl TomlKey {
    /// Converts a composite key to its string representation (e.g., `a.b.c`).
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

impl TomlValueNode {
    /// Returns the string slice if the value is a string.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            TomlValueNode::String(s) => Some(&s.value),
            _ => None,
        }
    }

    /// Returns the integer value if the value is an integer.
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            TomlValueNode::Integer(i) => Some(i.value),
            _ => None,
        }
    }

    /// Returns the boolean value if the value is a boolean.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            TomlValueNode::Boolean(b) => Some(b.value),
            _ => None,
        }
    }

    /// Returns a reference to the inline table if the value is an inline table.
    pub fn as_inline_table(&self) -> Option<&TomlInlineTable> {
        match self {
            TomlValueNode::InlineTable(t) => Some(t),
            _ => None,
        }
    }

    /// Returns a reference to the array if the value is an array.
    pub fn as_array(&self) -> Option<&TomlArray> {
        match self {
            TomlValueNode::Array(a) => Some(a),
            _ => None,
        }
    }

    /// Gets the source span of the value.
    pub fn span(&self) -> Range<usize> {
        match self {
            TomlValueNode::String(s) => s.span.clone(),
            TomlValueNode::Integer(i) => i.span.clone(),
            TomlValueNode::Float(f) => f.span.clone(),
            TomlValueNode::Boolean(b) => b.span.clone(),
            TomlValueNode::DateTime(dt) => dt.span.clone(),
            TomlValueNode::Array(a) => a.span.clone(),
            TomlValueNode::InlineTable(t) => t.span.clone(),
        }
    }
}

impl std::fmt::Display for TomlValueNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TomlValueNode::String(s) => write!(f, "\"{}\"", s.value),
            TomlValueNode::Integer(i) => write!(f, "{}", i.value),
            TomlValueNode::Float(fl) => write!(f, "{}", fl.value),
            TomlValueNode::Boolean(b) => write!(f, "{}", b.value),
            TomlValueNode::DateTime(dt) => write!(f, "{}", dt.value),
            TomlValueNode::Array(a) => {
                write!(f, "[")?;
                for (i, item) in a.items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            TomlValueNode::InlineTable(t) => {
                write!(f, "{{")?;
                for (i, item) in t.items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{} = {}", item.key.to_string(), item.value)?;
                }
                write!(f, "}}")
            }
        }
    }
}

impl TomlRoot {
    /// Converts the TomlRoot into a TomlValueNode (inline table).
    pub fn into_value(self) -> TomlValueNode {
        let mut items = Vec::new();
        
        // Only include key-value pairs in the root
        for item in self.items {
            if let TomlItem::KeyValue(kv) = item {
                items.push(kv);
            }
        }
        
        TomlValueNode::InlineTable(TomlInlineTable {
            span: self.span,
            items,
        })
    }
}
