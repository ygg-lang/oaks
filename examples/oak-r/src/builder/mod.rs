use crate::{RParser, language::RLanguage};
use oak_core::{Builder, BuilderCache, Parser, TextEdit, source::Source};

#[derive(Clone)]
pub struct RBuilder<'config> {
    config: &'config RLanguage,
}

impl<'config> RBuilder<'config> {
    pub fn new(config: &'config RLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<RLanguage> for RBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<RLanguage>) -> oak_core::builder::BuildOutput<RLanguage> {
        let parser = RParser::new(self.config);
        let mut parse_cache = oak_core::parser::session::ParseSession::<RLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut parse_cache);
        match parse_result.result {
            Ok(_) => oak_core::errors::OakDiagnostics { result: Ok(()), diagnostics: parse_result.diagnostics },
            Err(e) => oak_core::errors::OakDiagnostics { result: Err(e), diagnostics: parse_result.diagnostics },
        }
    }
}
