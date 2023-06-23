use core::range::Range;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents the provenance of a token, describing how its text was composed.
///
/// This is used for advanced IDE features like renaming synthetic identifiers.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TokenProvenance {
    /// The parts that compose this token's text.
    pub parts: Vec<ProvenancePart>,
}

/// A single component of a token's provenance.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ProvenancePart {
    /// Part of the text comes directly from a source range.
    Source(#[cfg_attr(feature = "serde", serde(with = "crate::serde_range"))] Range<usize>),
    /// Part of the text is synthesized (e.g., by a macro).
    /// The string is typically an interned ID or a small literal.
    Synthesized(String),
    /// An opaque tag for language-specific transformations (e.g., case conversion).
    /// Oak doesn't understand these tags, but passes them to the LSP/IDE.
    OpaqueTag(String),
}

impl TokenProvenance {
    /// Creates a new provenance from a single source range.
    pub fn from_source(range: Range<usize>) -> Self {
        Self { parts: vec![ProvenancePart::Source(range)] }
    }

    /// Creates a new provenance from a synthesized string.
    pub fn from_synthesized(s: impl Into<String>) -> Self {
        Self { parts: vec![ProvenancePart::Synthesized(s.into())] }
    }
}
