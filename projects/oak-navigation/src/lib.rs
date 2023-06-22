use oak_core::{language::Language, tree::RedNode};
use oak_lsp::{Location, Position};

/// Trait for languages that support jumping to definition.
pub trait DefinitionProvider<L: Language> {
    /// Returns the definition(s) of the symbol at the given position.
    fn definition(&self, root: &RedNode<L>, position: Position) -> Vec<Location>;
}

/// Trait for languages that support finding references.
pub trait ReferencesProvider<L: Language> {
    /// Returns the references to the symbol at the given position.
    fn references(&self, root: &RedNode<L>, position: Position, include_declaration: bool) -> Vec<Location>;
}
