use crate::{ast::*, language::TclLanguage, lexer::TclLexer, parser::TclParser};
use oak_core::{Builder, BuilderCache, GreenNode, Lexer, OakDiagnostics, OakError, Parser, RedNode, SourceText, TextEdit, source::Source};

/// Tcl 语言的 AST 构建器
#[derive(Clone)]
pub struct TclBuilder<'config> {
    /// 语言配置
    config: &'config TclLanguage,
}

impl<'config> TclBuilder<'config> {
    /// 创建新的 Tcl 构建器
    pub fn new(config: &'config TclLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<TclLanguage> for TclBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<TclLanguage>) -> oak_core::builder::BuildOutput<TclLanguage> {
        let parser = TclParser::new(self.config);
        let lexer = TclLexer::new(&self.config);

        let mut cache = oak_core::parser::session::ParseSession::<TclLanguage>::default();
        lexer.lex(source, edits, &mut cache);
        let parse_result = parser.parse(source, edits, &mut cache);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()).into_owned());
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

impl<'config> TclBuilder<'config> {
    /// 构建根节点
    pub(crate) fn build_root(&self, green_tree: GreenNode<TclLanguage>, _source: &SourceText) -> Result<TclRoot, OakError> {
        let red_root = RedNode::new(&green_tree, 0);
        let span = red_root.span();
        let items = Vec::new();

        // TODO: 遍历 red_root.children() 并构建 TclItem

        Ok(TclRoot { span: span.into(), items })
    }
}
