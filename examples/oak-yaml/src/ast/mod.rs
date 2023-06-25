#![doc = include_str!("readme.md")]

use core::range::Range;

/// YAML root node.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct YamlRoot {
    /// Range of the node in the source code.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
    /// Items in the YAML document.
    pub items: Vec<YamlValue>,
}

/// YAML value.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum YamlValue {
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
    pub items: Vec<YamlValue>,
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
    pub key: YamlValue,
    /// Value of the entry.
    pub value: YamlValue,
}
