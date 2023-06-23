use crate::language::RbqLanguage;
use oak_core::{Builder, BuilderCache, Lexer, Parser, TextEdit, source::Source, tree::RedNode};

#[derive(Clone)]
pub struct RbqBuilder<'config> {
    config: &'config RbqLanguage,
}

impl<'config> RbqBuilder<'config> {
    pub fn new(config: &'config RbqLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<RbqLanguage> for RbqBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<RbqLanguage>) -> oak_core::builder::BuildOutput<RbqLanguage> {
        let parser = crate::parser::RbqParser::new(self.config);
        let lexer = crate::lexer::RbqLexer::new(&self.config);
        let mut parse_cache = oak_core::parser::session::ParseSession::<RbqLanguage>::default();
        lexer.lex(source, edits, &mut parse_cache);
        let parse_result = parser.parse(source, edits, &mut parse_cache);

        let source_text = source.get_text_in((0..source.length()).into());
        let result = parse_result.result.map(|green| {
            let red = RedNode::new(green, 0);
            crate::ast::RbqRoot::lower(red, &source_text)
        });

        oak_core::errors::OakDiagnostics { result, diagnostics: parse_result.diagnostics }
    }
}
