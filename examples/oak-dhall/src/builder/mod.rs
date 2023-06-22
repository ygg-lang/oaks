use crate::parser::DHallParser;
#[doc = include_str!("../readme.md")]
use crate::{ast::*, language::DHallLanguage};
use oak_core::{Builder, BuilderCache, GreenNode, OakDiagnostics, OakError, Parser, RedNode, SourceText, TextEdit, builder::BuildOutput, source::Source};

/// DHall AST 构建器
#[derive(Clone)]
pub struct DHallBuilder<'config> {
    config: &'config DHallLanguage,
}

impl<'config> DHallBuilder<'config> {
    pub fn new(config: &'config DHallLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<DHallLanguage> for DHallBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<DHallLanguage>) -> BuildOutput<DHallLanguage> {
        let parser = DHallParser::new(self.config);
        let mut cache = oak_core::parser::session::ParseSession::<DHallLanguage>::default();
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

impl<'config> DHallBuilder<'config> {
    pub(crate) fn build_root(&self, green_tree: GreenNode<DHallLanguage>, _source: &SourceText) -> Result<DHallRoot, OakError> {
        let _red_root = RedNode::new(&green_tree, 0);
        // TODO: 实现真正的构建逻辑
        Ok(DHallRoot { expressions: Vec::new() })
    }
}
