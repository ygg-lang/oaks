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
    fn build<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<ValaLanguage>) -> oak_core::builder::BuildOutput<ValaLanguage> {
        let parser = ValaParser::new(self.config);
        let mut cache = oak_core::parser::ParseSession::<ValaLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut cache);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()).into_owned());
                match self.build_root(&green_tree, &source_text) {
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
    /// 从语法树构建 AST 根节点
    pub fn build_root(&self, _green: &GreenNode<ValaLanguage>, _source: &SourceText) -> Result<ValaRoot, oak_core::OakError> {
        // 简化的 AST 构建逻辑
        Ok(ValaRoot { span: (0.._source.len()).into(), items: vec![] })
    }
}
