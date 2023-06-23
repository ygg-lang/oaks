use crate::language::ScssLanguage;
use oak_core::{Builder, BuilderCache, Lexer, OakDiagnostics, Parser, TextEdit, builder::BuildOutput, source::Source};

/// SCSS 语言的 AST 构建器
#[derive(Clone)]
pub struct ScssBuilder<'config> {
    config: &'config ScssLanguage,
}

impl<'config> ScssBuilder<'config> {
    pub fn new(config: &'config ScssLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<ScssLanguage> for ScssBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<ScssLanguage>) -> BuildOutput<ScssLanguage> {
        let parser = crate::parser::ScssParser::new(self.config);
        let lexer = crate::lexer::ScssLexer::new(&self.config);

        let mut cache = oak_core::parser::session::ParseSession::<ScssLanguage>::default();
        lexer.lex(source, edits, &mut cache);
        let parse_result = parser.parse(source, edits, &mut cache);

        match parse_result.result {
            Ok(_) => OakDiagnostics { result: Ok(()), diagnostics: parse_result.diagnostics },
            Err(e) => OakDiagnostics { result: Err(e), diagnostics: parse_result.diagnostics },
        }
    }
}
