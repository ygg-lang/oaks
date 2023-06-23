#![doc = include_str!("readme.md")]

use crate::{ast::*, language::AplLanguage, parser::AplParser};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, Parser, SourceText, TextEdit, source::Source};

/// APL 语言的 AST 构建器
#[derive(Clone)]
pub struct AplBuilder<'config> {
    /// 语言配置
    config: &'config AplLanguage,
}

impl<'config> AplBuilder<'config> {
    /// 创建新的 APL 构建器
    pub fn new(config: &'config AplLanguage) -> Self {
        Self { config }
    }

    /// 从语法树构建 AST 根节点
    pub fn build_root(&self, _green: &GreenNode<AplLanguage>, _source: &SourceText) -> Result<AplRoot, oak_core::OakError> {
        // 简化的 AST 构建逻辑
        Ok(AplRoot::new(vec![]))
    }
}

impl<'config> Builder<AplLanguage> for AplBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<AplLanguage>) -> oak_core::builder::BuildOutput<AplLanguage> {
        let parser = AplParser::new(self.config);
        let mut cache = oak_core::parser::ParseSession::<AplLanguage>::default();
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
