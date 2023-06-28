/// Liquid Builder module
///
/// This module defines the builder for Liquid templates, used to construct the AST.
use oak_core::{
    builder::{BuildOutput, Builder, BuilderCache},
    source::{Source, TextEdit},
};

use crate::language::LiquidLanguage;

/// Builder for Liquid templates
#[derive(Debug, Clone)]
pub struct LiquidBuilder<'a> {
    /// The language instance
    language: &'a LiquidLanguage,
}

impl<'a> LiquidBuilder<'a> {
    /// Create a new Liquid builder
    pub fn new(language: &'a LiquidLanguage) -> Self {
        Self { language }
    }
}

impl<'a> Builder<LiquidLanguage> for LiquidBuilder<'a> {
    fn build<'b, S: Source + ?Sized>(&self, _text: &S, _edits: &[TextEdit], _cache: &'b mut impl BuilderCache<LiquidLanguage>) -> BuildOutput<LiquidLanguage> {
        // TODO: Implement actual building logic
        unimplemented!()
    }
}
