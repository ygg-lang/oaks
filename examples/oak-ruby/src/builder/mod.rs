use crate::{ast::ProgramNode, language::RubyLanguage, parser::RubyParser};
use oak_core::{Builder, BuilderCache, Lexer, OakDiagnostics, Parser, TextEdit, source::Source};

pub struct RubyBuilder<'config> {
    config: &'config RubyLanguage,
}

impl<'config> RubyBuilder<'config> {
    pub fn new(config: &'config RubyLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<RubyLanguage> for RubyBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<RubyLanguage>) -> oak_core::builder::BuildOutput<RubyLanguage> {
        let parser = RubyParser::new(self.config);
        let lexer = crate::lexer::RubyLexer::new(self.config);

        let mut cache = oak_core::parser::session::ParseSession::<RubyLanguage>::default();
        lexer.lex(source, edits, &mut cache);
        let parse_result = parser.parse(source, edits, &mut cache);

        match parse_result.result {
            Ok(_green_tree) => {
                // Simple conversion for now
                OakDiagnostics { result: Ok(ProgramNode { statements: vec![], span: core::range::Range { start: 0, end: source.length() } }), diagnostics: parse_result.diagnostics }
            }
            Err(parse_error) => OakDiagnostics { result: Err(parse_error), diagnostics: parse_result.diagnostics },
        }
    }
}
