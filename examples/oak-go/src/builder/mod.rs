//! Go 语言构建器

use crate::{ast::GoRoot, language::GoLanguage, lexer::GoLexer, parser::GoParser};
use oak_core::{Builder, BuilderCache, GreenNode, Lexer, OakDiagnostics, OakError, ParseSession, Parser, SourceText, TextEdit, builder::BuildOutput, source::Source};

/// Go 语言构建器
pub struct GoBuilder<'config> {
    pub(crate) config: &'config GoLanguage,
}

impl<'config> GoBuilder<'config> {
    pub fn new(config: &'config GoLanguage) -> Self {
        Self { config }
    }

    fn build_root<'a>(&self, _green: &'a GreenNode<'a, GoLanguage>, _source: &SourceText) -> Result<GoRoot, OakError> {
        // TODO: 从 GreenNode 构建 AST
        Ok(GoRoot { package: None, imports: vec![], declarations: vec![] })
    }
}

impl<'config> Builder<GoLanguage> for GoBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<GoLanguage>) -> BuildOutput<GoLanguage> {
        let parser = GoParser::new(self.config);
        let lexer = GoLexer::new(self.config);

        let mut session = ParseSession::<GoLanguage>::default();
        lexer.lex(source, edits, &mut session);
        let parse_result = parser.parse(source, edits, &mut session);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()));
                match self.build_root(green_tree, &source_text) {
                    Ok(ast_root) => OakDiagnostics { result: Ok(ast_root), diagnostics: parse_result.diagnostics },
                    Err(build_error) => {
                        let mut diagnostics = parse_result.diagnostics;
                        diagnostics.push(build_error.clone());
                        OakDiagnostics { result: Err(build_error), diagnostics }
                    }
                }
            }
            Err(parse_error) => OakDiagnostics { result: Err(parse_error), diagnostics: parse_result.diagnostics },
        }
    }
}
