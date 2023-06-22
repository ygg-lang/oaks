use crate::{ast::*, language::ValaLanguage, lexer::ValaLexer, parser::ValaParser};
use oak_core::{Builder, BuilderCache, GreenNode, Lexer, OakDiagnostics, OakError, Parser, RedNode, SourceText, TextEdit, source::Source};

/// Vala 语言的 AST 构建器
pub struct ValaBuilder<'config> {
    config: &'config ValaLanguage,
}

impl<'config> ValaBuilder<'config> {
    pub fn new(config: &'config ValaLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<ValaLanguage> for ValaBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, text: &S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<ValaLanguage>) -> oak_core::builder::BuildOutput<ValaLanguage> {
        let parser = ValaParser::new(self.config);
        let lexer = ValaLexer::new(self.config);

        lexer.lex(text, edits, cache);
        let parse_result = parser.parse(text, edits, cache);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(text.get_text_in((0..text.length()).into()));
                match self.build_root(green_tree.clone(), &source_text) {
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

impl<'config> ValaBuilder<'config> {
    pub(crate) fn build_root(&self, green_tree: GreenNode<ValaLanguage>, _source: &SourceText) -> Result<ValaRoot, OakError> {
        let red_root = RedNode::new(&green_tree, 0);
        let span = red_root.span();
        Ok(ValaRoot { span: span.into(), items: Vec::new() })
    }
}
