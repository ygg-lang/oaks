#![warn(missing_docs)]
//! Semantic tokens support for the Oak language framework.
//!
//! This crate provides traits and structures for semantic syntax highlighting,
//! compatible with the LSP Semantic Tokens specification.
use oak_core::{language::Language, source::Source, tree::RedNode};
use oak_vfs::LineMap;
use serde::{Deserialize, Serialize};

/// Represents a semantic token for syntax highlighting.
///
/// Benchmarked against LSP's Semantic Tokens.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticToken {
    /// The delta line from the previous token.
    pub delta_line: u32,
    /// The delta start character from the previous token or line start.
    pub delta_start: u32,
    /// The length of the token in characters.
    pub length: u32,
    /// The type of the token (index into the legend).
    pub token_type: u32,
    /// The modifiers of the token (bitset).
    pub token_modifiers_bitset: u32,
}

/// Trait for languages that support semantic highlighting.
pub trait SemanticTokensProvider<L: Language> {
    /// Returns the semantic tokens for the given document.
    fn semantic_tokens<S: Source + ?Sized>(&self, root: &RedNode<L>, source: &S, line_map: &LineMap) -> Vec<SemanticToken>;
}
