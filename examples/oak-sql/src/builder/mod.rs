use crate::language::SqlLanguage;
use oak_core::{Builder, BuilderCache, Lexer, Parser, TextEdit, source::Source};

#[derive(Clone)]
pub struct SqlBuilder<'config> {
    config: &'config SqlLanguage,
}

impl<'config> SqlBuilder<'config> {
    pub fn new(config: &'config SqlLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<SqlLanguage> for SqlBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<SqlLanguage>) -> oak_core::builder::BuildOutput<SqlLanguage> {
        let parser = crate::parser::SqlParser::new(self.config);
        let lexer = crate::lexer::SqlLexer::new(self.config);
        let mut parse_cache = oak_core::parser::session::ParseSession::<SqlLanguage>::default();
        lexer.lex(source, edits, &mut parse_cache);
        let parse_result = parser.parse(source, edits, &mut parse_cache);

        oak_core::errors::OakDiagnostics {
            result: Ok(Default::default()), // TODO: Implement SqlRoot conversion
            diagnostics: parse_result.diagnostics,
        }
    }
}
