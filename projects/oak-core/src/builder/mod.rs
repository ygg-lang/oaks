use crate::{
    Language,
    errors::OakDiagnostics,
    source::{Source, TextEdit},
};

/// Output of a builder operation.
pub type BuildOutput<L: Language> = OakDiagnostics<L::TypedRoot>;

/// Trait for building higher-level structures (like ASTs) from source text.
pub trait Builder<L: Language> {
    /// Builds the structure from the source text.
    fn build<'a, S: Source + ?Sized>(&self, text: &S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<L>) -> BuildOutput<L>;
}

/// Cache trait for builder operations used by lexers and parsers.
///
/// This trait defines the interface for building green tree nodes incrementally.
/// It provides methods for adding tokens and nodes to the tree structure.
pub trait BuilderCache<L: Language>: crate::parser::ParseCache<L> {}

impl<'a, L: Language, C: BuilderCache<L> + ?Sized> BuilderCache<L> for &'a mut C {}

impl<L: Language + Send + Sync + 'static> BuilderCache<L> for crate::parser::ParseSession<L> {}
