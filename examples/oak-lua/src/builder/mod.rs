use crate::{ast::LuaRoot, language::LuaLanguage, lexer::LuaLexer, parser::LuaParser};
use oak_core::{Builder, BuilderCache, Lexer, Parser, TextEdit, builder::BuildOutput, parser::session::ParseSession, source::Source};

/// Lua AST 构建器
#[derive(Clone)]
pub struct LuaBuilder<'config> {
    config: &'config LuaLanguage,
}

impl<'config> LuaBuilder<'config> {
    pub fn new(config: &'config LuaLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<LuaLanguage> for LuaBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<LuaLanguage>) -> BuildOutput<LuaLanguage> {
        let parser = LuaParser::new(self.config);
        let lexer = LuaLexer::new(self.config);

        let mut session = ParseSession::<LuaLanguage>::default();
        lexer.lex(source, edits, &mut session);
        let parse_result = parser.parse(source, edits, &mut session);

        match parse_result.result {
            Ok(green_tree) => match self.build_root(green_tree.clone(), source) {
                Ok(ast_root) => oak_core::errors::OakDiagnostics { result: Ok(ast_root), diagnostics: parse_result.diagnostics },
                Err(build_error) => {
                    let mut diagnostics = parse_result.diagnostics;
                    diagnostics.push(build_error.clone());
                    oak_core::errors::OakDiagnostics { result: Err(build_error), diagnostics }
                }
            },
            Err(parse_error) => oak_core::errors::OakDiagnostics { result: Err(parse_error), diagnostics: parse_result.diagnostics },
        }
    }
}

impl<'config> LuaBuilder<'config> {
    fn build_root<S: Source + ?Sized>(&self, _green_tree: oak_core::GreenNode<LuaLanguage>, _source: &S) -> Result<LuaRoot, oak_core::OakError> {
        // TODO: 从 GreenNode 构建 AST
        Ok(LuaRoot { statements: vec![], span: (0..0).into() })
    }
}
