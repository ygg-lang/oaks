use crate::{language::SchemeLanguage, parser::SchemeParser};
use oak_core::{Builder, BuilderCache, Lexer, OakDiagnostics, Parser, TextEdit, source::Source};

/// Scheme 语言的 AST 构建器
#[derive(Clone)]
pub struct SchemeBuilder<'config> {
    config: &'config SchemeLanguage,
}

impl<'config> SchemeBuilder<'config> {
    pub fn new(config: &'config SchemeLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<SchemeLanguage> for SchemeBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<SchemeLanguage>) -> OakDiagnostics<()> {
        let parser = SchemeParser::new(self.config);
        let lexer = crate::lexer::SchemeLexer::new(self.config);

        lexer.lex(source, edits, cache);
        let parse_result = parser.parse(source, edits, cache);

        match parse_result.result {
            Ok(_) => OakDiagnostics { result: Ok(()), diagnostics: parse_result.diagnostics },
            Err(e) => OakDiagnostics { result: Err(e), diagnostics: parse_result.diagnostics },
        }
    }
}
