use oak_core::{
    Builder,
    source::{Source, TextEdit},
};

use crate::language::MetisLanguage;

/// Builder for Metis island-language CST.
pub struct MetisBuilder;

impl Builder<MetisLanguage> for MetisBuilder {
    fn build<'a, S: Source + ?Sized>(&self, text: &S, edits: &[TextEdit], cache: &'a mut impl oak_core::builder::BuilderCache<MetisLanguage>) -> oak_core::builder::BuildOutput<MetisLanguage> {
        use oak_core::parser::Parser;

        let language = MetisLanguage::default();
        let parser = language.parser();
        let green_tree = parser.parse(text, edits, cache);
        oak_core::errors::OakDiagnostics { result: Err(oak_core::errors::OakError::custom_error("Metis CST builder not implemented yet")), diagnostics: green_tree.diagnostics }
    }
}

impl Default for MetisBuilder {
    fn default() -> Self {
        Self
    }
}
