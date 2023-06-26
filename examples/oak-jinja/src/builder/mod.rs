/// Jinja Builder module
///
/// This module defines the builder for Jinja templates, used to construct the AST.
use oak_core::{
    builder::{BuildOutput, Builder, BuilderCache},
    source::{Source, TextEdit},
};

use crate::language::JinjaLanguage;

/// Builder for Jinja templates
#[derive(Debug, Clone)]
pub struct JinjaBuilder<'a> {
    /// The language instance
    language: &'a JinjaLanguage,
}

impl<'a> JinjaBuilder<'a> {
    /// Create a new Jinja builder
    pub fn new(language: &'a JinjaLanguage) -> Self {
        Self { language }
    }
}

impl<'a> Builder<JinjaLanguage> for JinjaBuilder<'a> {
    fn build<'b, S: Source + ?Sized>(&self, _text: &S, _edits: &[TextEdit], _cache: &'b mut impl BuilderCache<JinjaLanguage>) -> BuildOutput<JinjaLanguage> {
        // TODO: Implement actual building logic
        unimplemented!()
    }
}
