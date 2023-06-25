use crate::{language::JavaLanguage, parser::JavaParser};
use oak_core::{
    Parser, Source,
    builder::{BuildOutput, Builder, BuilderCache},
    source::TextEdit,
};

mod build_expressions;
mod build_inheritance;
mod build_items;
mod build_members;
mod build_statements;
mod build_utils;

/// Java AST builder.
pub struct JavaBuilder<'config> {
    language: &'config JavaLanguage,
}

impl<'config> JavaBuilder<'config> {
    /// Create a new Java builder.
    pub fn new(language: &'config JavaLanguage) -> Self {
        Self { language }
    }
}

impl<'config> Builder<JavaLanguage> for JavaBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, text: &S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<JavaLanguage>) -> BuildOutput<JavaLanguage> {
        let parser = JavaParser::new(self.language);
        let output = parser.parse(text, edits, cache);
        let result = output.result.map(|green| self.build_root(green, &text.get_text_in((0..text.length()).into())));
        oak_core::errors::OakDiagnostics { result, diagnostics: output.diagnostics }
    }
}
