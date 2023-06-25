#![doc = include_str!("readme.md")]
use crate::{ast::NoteDocument, language::NotedownLanguage};
use oak_core::{Builder, BuilderCache, TextEdit, source::Source};

/// Notedown AST builder
pub struct NoteBuilder<'config> {
    pub(crate) config: &'config NotedownLanguage,
}

impl<'config> NoteBuilder<'config> {
    /// Create a new builder with the given language configuration
    pub fn new(config: &'config NotedownLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<NotedownLanguage> for NoteBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, _source: &S, _edits: &[TextEdit], _cache: &'a mut impl BuilderCache<NotedownLanguage>) -> oak_core::builder::BuildOutput<NotedownLanguage> {
        // Simple implementation
        oak_core::errors::OakDiagnostics { result: Ok(NoteDocument {}), diagnostics: Vec::new() }
    }
}
