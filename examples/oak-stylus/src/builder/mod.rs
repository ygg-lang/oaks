use crate::language::StylusLanguage;
use oak_core::{Builder, BuilderCache, OakDiagnostics, TextEdit, source::Source};

#[derive(Clone)]
pub struct StylusBuilder<'config> {
    config: &'config StylusLanguage,
}

impl<'config> StylusBuilder<'config> {
    pub fn new(config: &'config StylusLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<StylusLanguage> for StylusBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<StylusLanguage>) -> oak_core::builder::BuildOutput<StylusLanguage> {
        let parser = crate::parser::StylusParser::new(self.config);
        let lexer = crate::lexer::StylusLexer::new(&self.config);
        let mut cache = oak_core::parser::session::ParseSession::<StylusLanguage>::default();
        let parse_result = oak_core::parser::parse(&parser, &lexer, source, edits, &mut cache);
        match parse_result.result {
            Ok(_) => OakDiagnostics { result: Ok(()), diagnostics: parse_result.diagnostics },
            Err(e) => OakDiagnostics { result: Err(e), diagnostics: parse_result.diagnostics },
        }
    }
}
