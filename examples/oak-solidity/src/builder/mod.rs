use crate::{SolidityParser, language::SolidityLanguage};
use oak_core::{Builder, BuilderCache, OakDiagnostics, Parser, TextEdit, source::Source};

#[derive(Clone)]
pub struct SolidityBuilder<'config> {
    config: &'config SolidityLanguage,
}

impl<'config> SolidityBuilder<'config> {
    pub fn new(config: &'config SolidityLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<SolidityLanguage> for SolidityBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<SolidityLanguage>) -> OakDiagnostics<()> {
        let parser = SolidityParser::new(self.config);
        let parse_result = parser.parse(source, edits, cache);
        match parse_result.result {
            Ok(_) => OakDiagnostics { result: Ok(()), diagnostics: parse_result.diagnostics },
            Err(e) => OakDiagnostics { result: Err(e), diagnostics: parse_result.diagnostics },
        }
    }
}
