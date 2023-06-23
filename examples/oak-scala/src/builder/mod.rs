#![doc = include_str!("readme.md")]
use crate::{ast::ScalaRoot, language::ScalaLanguage, parser::ScalaParser};
use oak_core::{Builder, BuilderCache, Lexer, OakDiagnostics, Parser, RedNode, TextEdit, source::Source};

/// Scala 语言的 AST 构建器
#[derive(Clone)]
pub struct ScalaBuilder<'config> {
    config: &'config ScalaLanguage,
}

impl<'config> ScalaBuilder<'config> {
    pub fn new(config: &'config ScalaLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<ScalaLanguage> for ScalaBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<ScalaLanguage>) -> OakDiagnostics<ScalaRoot> {
        let parser = ScalaParser::new(self.config);
        let lexer = crate::lexer::ScalaLexer::new(&self.config);

        lexer.lex(source, edits, cache);
        let parse_result = parser.parse(source, edits, cache);

        match parse_result.result {
            Ok(green_tree) => {
                let ast_root = ScalaRoot::new(RedNode::new(green_tree, 0).span());
                OakDiagnostics { result: Ok(ast_root), diagnostics: parse_result.diagnostics }
            }
            Err(parse_error) => OakDiagnostics { result: Err(parse_error), diagnostics: parse_result.diagnostics },
        }
    }
}
