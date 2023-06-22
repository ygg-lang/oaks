use crate::{language::SwiftLanguage, parser::SwiftParser};
use oak_core::{Builder, BuilderCache, OakDiagnostics, Parser, TextEdit, source::Source};

#[derive(Clone)]
pub struct SwiftBuilder<'config> {
    config: &'config SwiftLanguage,
}

impl<'config> SwiftBuilder<'config> {
    pub fn new(config: &'config SwiftLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<SwiftLanguage> for SwiftBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<SwiftLanguage>) -> oak_core::builder::BuildOutput<SwiftLanguage> {
        let parser = SwiftParser::new(self.config);
        let _lexer = crate::lexer::SwiftLexer::new(&self.config);
        let mut cache = oak_core::parser::ParseSession::<SwiftLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut cache);
        match parse_result.result {
            Ok(_) => OakDiagnostics { result: Ok(()), diagnostics: parse_result.diagnostics },
            Err(e) => OakDiagnostics { result: Err(e), diagnostics: parse_result.diagnostics },
        }
    }
}
