use oak_core::{
    Builder,
    source::{Source, TextEdit},
};

use crate::{ast::GlobRoot, language::GlobLanguage};

/// Builder for glob pattern AST.
pub struct GlobBuilder;

impl Builder<GlobLanguage> for GlobBuilder {
    fn build<'a, S: Source + ?Sized>(&self, text: &S, edits: &[TextEdit], cache: &'a mut impl oak_core::builder::BuilderCache<GlobLanguage>) -> oak_core::builder::BuildOutput<GlobLanguage> {
        use oak_core::parser::Parser;

        let language = GlobLanguage::default();
        let parser = language.parser();

        let green_tree = parser.parse(text, edits, cache);
        oak_core::errors::OakDiagnostics { result: Err(oak_core::errors::OakError::custom_error("Not implemented")), diagnostics: green_tree.diagnostics }
    }
}

impl Default for GlobBuilder {
    fn default() -> Self {
        Self
    }
}
