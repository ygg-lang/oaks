#![doc = include_str!("readme.md")]
pub mod element_type;

use crate::{language::TomlLanguage, lexer::token_type::TomlTokenKind as TomlSyntaxKind};
use oak_core::{Parser, source::Source};

/// TOML 语言解析器
pub struct TomlParser<'config> {
    /// 语言配置
    pub(crate) config: &'config TomlLanguage,
}

impl<'config> TomlParser<'config> {
    pub fn new(config: &'config TomlLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<TomlLanguage> for TomlParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[oak_core::TextEdit], cache: &'a mut impl oak_core::ParseCache<TomlLanguage>) -> oak_core::ParseOutput<'a, TomlLanguage> {
        let lexer = crate::lexer::TomlLexer::new(&self.config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            // 简单的解析逻辑：消耗所有 token 并放入 Root 节点
            while state.current().is_some() {
                state.bump()
            }

            Ok(state.finish_at(checkpoint, TomlSyntaxKind::Root.into()))
        })
    }
}
