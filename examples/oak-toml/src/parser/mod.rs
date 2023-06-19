use crate::{TomlSyntaxKind, ast::*, language::TomlLanguage};
use oak_core::{
    Builder, GreenBuilder, GreenNode, GreenTree, IncrementalCache, Lexer, OakError, Parser, SyntaxKind, errors::OakDiagnostics,
    parser::ParserState, source::Source, tree::Arc,
};

/// TOML 语言解析器（不可变），通过 &mut ParserState 推进
pub struct TomlParser<'config> {
    /// 语言配置
    config: &'config TomlLanguage,
}

type State<'a, S: Source> = ParserState<'a, S, TomlLanguage>;

impl<'config> TomlParser<'config> {
    pub fn new(config: &'config TomlLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Clone for TomlParser<'config> {
    fn clone(&self) -> Self {
        TomlParser::new(self.config)
    }
}

impl<'config> Parser<TomlLanguage> for TomlParser<'config> {
    fn parse_incremental(
        &self,
        text: impl Source,
        changed: usize,
        cache: IncrementalCache<TomlLanguage>,
    ) -> OakDiagnostics<Arc<GreenNode<TomlSyntaxKind>>> {
        let mut state = ParserState::new_with_cache(text, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> Builder<TomlLanguage> for TomlParser<'config> {
    fn build_incremental(
        &self,
        _text: impl Source,
        _changed: usize,
        _cache: IncrementalCache<TomlLanguage>,
    ) -> OakDiagnostics<crate::ast::TomlRoot> {
        todo!()
    }
}

impl<'config> TomlParser<'config> {
    /// 主要的解析循环
    fn run<S: Source>(&self, state: &mut State<S>) -> Result<(), OakError> {
        // 创建根节点
        let root = GreenBuilder::<TomlLanguage>::new(1).token(TomlSyntaxKind::Eof, 0).finish(TomlSyntaxKind::Root);

        state.cache.last_parse = Some(root);
        Ok(())
    }
}
