use crate::{ast::*, language::TypeScriptLanguage, lexer::TypeScriptLexer, parser::TypeScriptParser};
use oak_core::{Builder, BuilderCache, GreenNode, Lexer, OakDiagnostics, OakError, Parser, RedNode, SourceText, TextEdit, source::Source};

/// TypeScript 语言的 AST 构建器
#[derive(Clone)]
pub struct TypeScriptBuilder<'config> {
    config: &'config TypeScriptLanguage,
}

impl<'config> TypeScriptBuilder<'config> {
    pub fn new(config: &'config TypeScriptLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<TypeScriptLanguage> for TypeScriptBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<TypeScriptLanguage>) -> OakDiagnostics<TypeScriptRoot> {
        let parser = TypeScriptParser::new(self.config);
        let lexer = TypeScriptLexer::new(self.config);

        let mut session = oak_core::parser::session::ParseSession::<TypeScriptLanguage>::default();
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

impl<'config> TypeScriptBuilder<'config> {
    pub(crate) fn build_root(&self, green_tree: &GreenNode<TypeScriptLanguage>, _source: &SourceText) -> Result<TypeScriptRoot, OakError> {
        let red_root = RedNode::new(green_tree, 0);
        let span = red_root.span();
        Ok(TypeScriptRoot { span: span.into() })
    }
}
