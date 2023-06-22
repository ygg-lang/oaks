use crate::{AdaParser, ast::*, language::AdaLanguage};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, Parser, SourceText, TextEdit, source::Source};

/// Ada 语言的 AST 构建器
#[derive(Clone)]
pub struct AdaBuilder<'config> {
    /// 语言配置
    config: &'config AdaLanguage,
}

impl<'config> AdaBuilder<'config> {
    /// 创建新的 Ada 构建器
    pub fn new(config: &'config AdaLanguage) -> Self {
        Self { config }
    }

    /// 从语法树构建 AST 根节点
    pub fn build_root(&self, _green: &GreenNode<AdaLanguage>, _source: &SourceText) -> Result<AdaRoot, oak_core::OakError> {
        // 简化的 AST 构建逻辑
        Ok(AdaRoot::new(vec![]))
    }
}

impl<'config> Builder<AdaLanguage> for AdaBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<AdaLanguage>) -> oak_core::builder::BuildOutput<AdaLanguage> {
        let parser = AdaParser::new(self.config);
        let mut cache = oak_core::parser::ParseSession::<AdaLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut cache);

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
