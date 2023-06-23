#![doc = include_str!("readme.md")]
use crate::{ast::*, language::CmdLanguage, parser::CmdParser};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, Parser, SourceText, TextEdit, source::Source};

/// Windows Command (CMD) 语言的 AST 构建器
#[derive(Clone)]
pub struct CmdBuilder<'config> {
    config: &'config CmdLanguage,
}

impl<'config> CmdBuilder<'config> {
    pub fn new(config: &'config CmdLanguage) -> Self {
        Self { config }
    }

    pub fn build_root(&self, _green: &GreenNode<CmdLanguage>, _source: &SourceText) -> Result<CmdRoot, oak_core::OakError> {
        // 简化的 AST 构建逻辑
        Ok(CmdRoot { elements: vec![] })
    }
}

impl<'config> Builder<CmdLanguage> for CmdBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<CmdLanguage>) -> oak_core::builder::BuildOutput<CmdLanguage> {
        let parser = CmdParser::new(self.config);
        let mut cache = oak_core::parser::ParseSession::<CmdLanguage>::default();
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
