use crate::{CParser, ast::*, language::CLanguage, lexer::CTokenType, parser::CElementType};
use core::range::Range;
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, Parser, RedNode, RedTree, SourceText, TextEdit, builder::BuildOutput, source::Source};

/// C 语言的 AST 构建器
#[derive(Clone, Copy)]
pub struct CBuilder<'config> {
    /// 语言配置
    config: &'config CLanguage,
}

impl<'config> CBuilder<'config> {
    /// 创建新的 C 构建器
    pub fn new(config: &'config CLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<CLanguage> for CBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<CLanguage>) -> BuildOutput<CLanguage> {
        // 解析源代码获得语法树
        let parser = CParser::new(self.config);

        // TODO: 真正的增量构建需要利用 BC
        let mut cache = oak_core::parser::session::ParseSession::<CLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut cache);

        // 检查解析是否成功
        match parse_result.result {
            Ok(green_tree) => {
                // 构建 AST
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

impl<'config> CBuilder<'config> {
    /// 构建根节点
    pub(crate) fn build_root<'a>(&self, green_tree: &'a GreenNode<'a, CLanguage>, _source: &SourceText) -> Result<CRoot, OakError> {
        let root_node = RedNode::new(green_tree, 0);

        // TODO: 实现真正的 Green -> AST 转换
        // 这里暂时返回一个空的 CRoot 以通过编译
        Ok(CRoot { translation_unit: TranslationUnit { external_declarations: vec![], span: root_node.span() }, span: root_node.span() })
    }
}
