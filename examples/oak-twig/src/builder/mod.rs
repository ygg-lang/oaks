use crate::{ast::TwigRoot, language::TwigLanguage, lexer::TwigLexer, parser::TwigParser};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, RedNode, SourceText, TextEdit, source::Source};

/// Twig 语言的 AST 构建器
#[derive(Clone)]
pub struct TwigBuilder<'config> {
    /// 语言配置
    config: &'config TwigLanguage,
}

impl<'config> TwigBuilder<'config> {
    /// 创建新的 Twig 构建器
    pub fn new(config: &'config TwigLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<TwigLanguage> for TwigBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<TwigLanguage>) -> oak_core::builder::BuildOutput<TwigLanguage> {
        let parser = TwigParser::new(self.config);
        let lexer = TwigLexer::new(self.config);

        let mut cache = oak_core::parser::session::ParseSession::<TwigLanguage>::default();
        let parse_result = oak_core::parser::parse(&parser, &lexer, source, edits, &mut cache);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()));
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

impl<'config> TwigBuilder<'config> {
    /// 构建根节点
    pub(crate) fn build_root(&self, green_tree: GreenNode<TwigLanguage>, _source: &SourceText) -> Result<TwigRoot, OakError> {
        let red_root = RedNode::new(&green_tree, 0);
        let span = red_root.span();

        Ok(TwigRoot { span: span.into() })
    }
}
