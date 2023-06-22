use crate::{ast::ProtobufRoot, language::ProtobufLanguage, parser::ProtobufParser};

use oak_core::{
    OakDiagnostics,
    builder::{BuildOutput, Builder, BuilderCache},
    parser::Parser,
    source::{Source, TextEdit},
};

pub struct ProtobufBuilder<'config> {
    config: &'config ProtobufLanguage,
}

impl<'config> ProtobufBuilder<'config> {
    pub fn new(config: &'config ProtobufLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<ProtobufLanguage> for ProtobufBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<ProtobufLanguage>) -> BuildOutput<ProtobufLanguage> {
        let parser = ProtobufParser::new(self.config);
        let mut cache = oak_core::parser::session::ParseSession::<ProtobufLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut cache);

        match parse_result.result {
            Ok(_) => {
                // Placeholder for actual AST building
                OakDiagnostics { result: Ok(ProtobufRoot {}), diagnostics: parse_result.diagnostics }
            }
            Err(e) => OakDiagnostics { result: Err(e), diagnostics: parse_result.diagnostics },
        }
    }
}
