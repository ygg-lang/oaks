use crate::{ast::TailwindRoot, language::TailwindLanguage, lexer::TailwindLexer, parser::TailwindParser};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, RedNode, SourceText, TextEdit, source::Source};

/// Tailwind 语言的 AST 构建器
#[derive(Clone)]
pub struct TailwindBuilder<'config> {
    /// 语言配置
    config: &'config TailwindLanguage,
}

impl<'config> TailwindBuilder<'config> {
    /// 创建新的 Tailwind 构建器
    pub fn new(config: &'config TailwindLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<TailwindLanguage> for TailwindBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<TailwindLanguage>) -> oak_core::builder::BuildOutput<TailwindLanguage> {
        let parser = TailwindParser::new(self.config);
        let lexer = TailwindLexer::new(&self.config);

        let mut cache = oak_core::parser::session::ParseSession::<TailwindLanguage>::default();
        let parse_result = oak_core::parser::parse(&parser, &lexer, source, edits, &mut cache);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()).into_owned());
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

impl<'config> TailwindBuilder<'config> {
    /// 构建根节点
    pub(crate) fn build_root(&self, green_tree: &GreenNode<TailwindLanguage>, _source: &SourceText) -> Result<TailwindRoot, OakError> {
        let red_root = RedNode::new(green_tree, 0);
        let span = red_root.span();

        Ok(TailwindRoot { span: span.into() })
    }
}
