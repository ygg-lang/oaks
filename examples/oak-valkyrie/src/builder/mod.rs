#![doc = include_str!("readme.md")]
use crate::{ValkyrieLanguage, ValkyrieParser};
use core::range::Range;
use oak_core::{
    Builder, OakDiagnostics, Parser, SourceText,
    builder::{BuildOutput, BuilderCache},
    source::{Source, TextEdit},
};

mod build_class;
mod build_expr;
mod build_micro;
mod build_namespace;
mod build_root;
mod build_stmt;

/// A builder for the Valkyrie programming language.
#[derive(Clone)]
pub struct ValkyrieBuilder<'config> {
    /// Language configuration
    config: &'config ValkyrieLanguage,
}

impl<'config> ValkyrieBuilder<'config> {
    /// Creates a new Valkyrie builder.
    pub fn new(config: &'config ValkyrieLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<ValkyrieLanguage> for ValkyrieBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<ValkyrieLanguage>) -> BuildOutput<ValkyrieLanguage> {
        let parser = ValkyrieParser::new(self.config);

        // 使用解析器获取绿树
        let mut parse_cache = oak_core::parser::ParseSession::<ValkyrieLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut parse_cache);

        // 检查解析是否成功
        match parse_result.result {
            Ok(green_tree) => {
                // 提前构造 SourceText 引用以便后续 AST 构建
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()).into_owned());
                // 构建 AST
                match parser.build_root(green_tree, &source_text) {
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

#[inline]
pub(crate) fn text(source: &SourceText, span: Range<usize>) -> String {
    source.get_text_in(span.into()).trim().to_string()
}
