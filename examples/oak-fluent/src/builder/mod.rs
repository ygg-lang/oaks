/// Fluent builder module.
use oak_core::{Builder, BuilderCache};
use oak_core::{
    builder::BuildOutput,
    source::{Source, TextEdit},
};

use crate::{ast::FluentRoot, language::FluentLanguage};

/// Fluent builder.
#[derive(Debug, Clone, Default)]
pub struct FluentBuilder;

impl Builder<FluentLanguage> for FluentBuilder {
    fn build<'a, S: Source + ?Sized>(&self, text: &S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<FluentLanguage>) -> BuildOutput<FluentLanguage> {
        // Implementation will be added here
        // For now, return an empty result
        let diagnostics = oak_core::errors::OakDiagnostics { result: Err(oak_core::errors::OakError::custom_error("Not implemented")), diagnostics: vec![] };
        diagnostics
    }
}
