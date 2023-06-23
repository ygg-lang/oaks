use crate::language::SmalltalkLanguage;
use oak_core::{Builder, BuilderCache, OakDiagnostics, Parser, TextEdit, source::Source};

/// Smalltalk 语言的 AST 构建器
#[derive(Clone)]
pub struct SmalltalkBuilder<'config> {
    config: &'config SmalltalkLanguage,
}

impl<'config> SmalltalkBuilder<'config> {
    pub fn new(config: &'config SmalltalkLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<SmalltalkLanguage> for SmalltalkBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<SmalltalkLanguage>) -> OakDiagnostics<()> {
        let parser = crate::parser::SmalltalkParser::new(self.config);
        let parse_result = parser.parse(source, edits, cache);

        match parse_result.result {
            Ok(_) => OakDiagnostics { result: Ok(()), diagnostics: parse_result.diagnostics },
            Err(e) => OakDiagnostics { result: Err(e), diagnostics: parse_result.diagnostics },
        }
    }
}
