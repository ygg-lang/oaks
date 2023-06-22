use crate::{ast::*, language::TexLanguage, parser::TexParser};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, Parser, RedNode, TextEdit, source::Source};

/// TeX 语言的 AST 构建器
#[derive(Clone)]
pub struct TexBuilder<'config> {
    /// 语言配置
    config: &'config TexLanguage,
}

impl<'config> TexBuilder<'config> {
    /// 创建新的 TeX 构建器
    pub fn new(config: &'config TexLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<TexLanguage> for TexBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], cache: &'a mut impl BuilderCache<TexLanguage>) -> OakDiagnostics<TexRoot> {
        let parser = TexParser::new(self.config);

        let parse_result = parser.parse(source, edits, cache);

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

impl<'config> TexBuilder<'config> {
    /// 构建根节点
    pub(crate) fn build_root(&self, green_tree: &GreenNode<TexLanguage>) -> Result<TexRoot, OakError> {
        let red_root = RedNode::new(green_tree, 0);
        let span = red_root.span();
        let items = Vec::new();

        // TODO: 遍历 red_root.children() 并构建 TexItem

        Ok(TexRoot { span: span.into(), items })
    }
}
