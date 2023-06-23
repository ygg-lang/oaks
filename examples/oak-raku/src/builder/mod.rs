use crate::{ast::RakuRoot, language::RakuLanguage, parser::RakuParser};
use oak_core::{
    Parser,
    builder::{BuildOutput, Builder, BuilderCache},
    source::{Source, TextEdit},
};

/// Builder for Raku syntax trees.
pub struct RakuBuilder {
    language: RakuLanguage,
}

impl RakuBuilder {
    /// Creates a new `RakuBuilder`.
    pub fn new(language: RakuLanguage) -> Self {
        Self { language }
    }
}

impl Builder<RakuLanguage> for RakuBuilder {
    fn build<'a, S: Source + ?Sized>(&self, text: &S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<RakuLanguage>) -> BuildOutput<RakuLanguage> {
        let parser = RakuParser::new(self.language);
        let output = parser.parse(text, edits, cache);
        let result = output.result.map(|green| RakuRoot { span: (0..green.text_len as usize).into() });
        oak_core::errors::OakDiagnostics { result, diagnostics: output.diagnostics }
    }
}
