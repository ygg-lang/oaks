use crate::{ast::*, language::NginxLanguage};
use oak_core::{Builder, BuilderCache, GreenNode, Lexer, OakDiagnostics, OakError, Parser, RedNode, TextEdit, source::Source};

pub struct NginxBuilder<'config> {
    config: &'config NginxLanguage,
}

impl<'config> NginxBuilder<'config> {
    pub fn new(config: &'config NginxLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<NginxLanguage> for NginxBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<NginxLanguage>) -> OakDiagnostics<NginxRoot> {
        let parser = crate::parser::NginxParser::new(self.config);
        let lexer = crate::lexer::NginxLexer::new(self.config);

        let mut session = oak_core::parser::session::ParseSession::<NginxLanguage>::default();
        lexer.lex(source, edits, &mut session);
        let parse_result = parser.parse(source, edits, &mut session);

        match parse_result.result {
            Ok(green_tree) => match self.build_root(green_tree) {
                Ok(ast_root) => OakDiagnostics { result: Ok(ast_root), diagnostics: parse_result.diagnostics },
                Err(build_error) => {
                    let mut diagnostics = parse_result.diagnostics;
                    diagnostics.push(build_error.clone());
                    OakDiagnostics { result: Err(build_error), diagnostics }
                }
            },
            Err(parse_error) => OakDiagnostics { result: Err(parse_error), diagnostics: parse_result.diagnostics },
        }
    }
}

impl<'config> NginxBuilder<'config> {
    pub(crate) fn build_root(&self, green_tree: &GreenNode<NginxLanguage>) -> Result<NginxRoot, OakError> {
        let red_root = RedNode::new(green_tree, 0);
        Ok(NginxRoot { range: red_root.span() })
    }
}
