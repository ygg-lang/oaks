use crate::{language::WolframLanguage, parser::WolframParser};
use oak_core::{Builder, BuilderCache, Lexer, OakDiagnostics, Parser, TextEdit, source::Source};

/// Wolfram 语言的 AST 构建器
#[derive(Clone)]
pub struct WolframBuilder<'config> {
    config: &'config WolframLanguage,
}

impl<'config> WolframBuilder<'config> {
    pub fn new(config: &'config WolframLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<WolframLanguage> for WolframBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<WolframLanguage>) -> OakDiagnostics<()> {
        let parser = WolframParser::new(self.config);
        let lexer = crate::lexer::WolframLexer::new(&self.config);

        lexer.lex(source, edits, cache);
        let parse_result = parser.parse(source, edits, cache);

        match parse_result.result {
            Ok(_) => OakDiagnostics { result: Ok(()), diagnostics: parse_result.diagnostics },
            Err(e) => OakDiagnostics { result: Err(e), diagnostics: parse_result.diagnostics },
        }
    }
}
