use oak_core::{language::Language, tree::RedNode};
use serde::{Deserialize, Serialize};

/// Represents a semantic token for syntax highlighting.
///
/// Benchmarked against LSP's Semantic Tokens.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticToken {
    pub delta_line: u32,
    pub delta_start: u32,
    pub length: u32,
    pub token_type: u32,
    pub token_modifiers_bitmask: u32,
}

/// Trait for languages that support semantic highlighting.
pub trait SemanticTokensProvider<L: Language> {
    /// Returns the semantic tokens for the given document.
    fn semantic_tokens(&self, root: &RedNode<L>) -> Vec<SemanticToken>;
}
