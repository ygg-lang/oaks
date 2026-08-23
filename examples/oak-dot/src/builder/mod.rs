#![doc = include_str!("readme.md")]
use crate::language::DotLanguage;
use oak_core::{Builder, BuilderCache, Source, TextEdit, builder::BuildOutput};

/// A builder for DOT language structures.
pub struct DotBuilder {}

impl DotBuilder {
    /// Creates a new instance of the DOT builder.
    pub fn new(_lang: &DotLanguage) -> Self {
        Self {}
    }
}

impl Builder<DotLanguage> for DotBuilder {
    fn build<'a, S: Source + ?Sized>(&self, _text: &S, _edits: &[TextEdit], _cache: &'a mut impl BuilderCache<DotLanguage>) -> BuildOutput<DotLanguage> {
        oak_core::errors::OakDiagnostics { result: Err(oak_core::errors::OakError::custom_error("Not implemented")), diagnostics: vec![] }
    }
}
