#![doc = include_str!("readme.md")]

use core::{fmt, range::Range};

/// YAML root node.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct YamlRoot {
    /// Range of the node in the source code.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// Items in the YAML document.
    pub items: Vec<YamlValueNode>,
}

/// YAML value.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum YamlValueNode {
    /// Scalar value.
    Scalar(YamlScalar),
    /// Sequence value.
    Sequence(YamlSequence),
    /// Mapping value.
    Mapping(YamlMapping),
}

/// YAML scalar value.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct YamlScalar {
    /// Range of the scalar in the source code.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// Value of the scalar.
    pub value: String,
}

/// YAML sequence value.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct YamlSequence {
    /// Range of the sequence in the source code.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// Items in the sequence.
    pub items: Vec<YamlValueNode>,
}

/// YAML mapping value.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct YamlMapping {
    /// Range of the mapping in the source code.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// Entries in the mapping.
    pub entries: Vec<YamlMappingEntry>,
}

/// YAML mapping entry.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct YamlMappingEntry {
    /// Range of the entry in the source code.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// Key of the entry.
    pub key: YamlValueNode,
    /// Value of the entry.
    pub value: YamlValueNode,
}

impl fmt::Display for YamlValueNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}

impl YamlValueNode {
    fn fmt_with_indent(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        let indent_str = "  ".repeat(indent);
        match self {
            YamlValueNode::Scalar(scalar) => {
                write!(f, "{}", scalar.value)
            }
            YamlValueNode::Sequence(seq) => {
                for (i, item) in seq.items.iter().enumerate() {
                    if i > 0 {
                        writeln!(f)?;
                    }
                    write!(f, "{indent_str}-")?;
                    match item {
                        YamlValueNode::Scalar(_) => {
                            write!(f, " ")?;
                            item.fmt_with_indent(f, indent)
                        }
                        _ => {
                            writeln!(f)?;
                            item.fmt_with_indent(f, indent + 1)
                        }
                    }?;
                }
                Ok(())
            }
            YamlValueNode::Mapping(map) => {
                for (i, entry) in map.entries.iter().enumerate() {
                    if i > 0 {
                        writeln!(f)?;
                    }
                    write!(f, "{indent_str}")?;
                    entry.key.fmt_with_indent(f, 0)?;
                    write!(f, ":")?;
                    match &entry.value {
                        YamlValueNode::Scalar(_) => {
                            write!(f, " ")?;
                            entry.value.fmt_with_indent(f, 0)
                        }
                        _ => {
                            writeln!(f)?;
                            entry.value.fmt_with_indent(f, indent + 1)
                        }
                    }?;
                }
                Ok(())
            }
        }
    }
}
