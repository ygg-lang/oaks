#![doc = include_str!("readme.md")]
use core::range::Range;

/// Root of the INI AST.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IniRoot {
    /// Sections in the INI file.
    pub sections: Vec<Section>,
    /// Global properties in the INI file.
    pub properties: Vec<Property>,
}

/// A section in the INI file.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Section {
    /// Name of the section.
    pub name: String,
    /// Properties in the section.
    pub properties: Vec<Property>,
    /// Span of the section in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// A key-value property in the INI file.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Property {
    /// Key of the property.
    pub key: String,
    /// Value of the property.
    pub value: String,
    /// Span of the property in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}
