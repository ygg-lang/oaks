use crate::{DjangoLanguage, DjangoSyntaxKind};
use oak_core::{
    GreenBuilder, GreenNode, IncrementalCache, Lexer, OakError, Parser, SyntaxKind,
    errors::OakDiagnostics,
    parser::ParserState,
    source::Source,
    tree::{Arc, GreenLeaf, GreenTree},
};

type State<'a, S: Source> = ParserState<'a, S, DjangoLanguage>;

/// Django 模板解析器
pub struct DjangoParser<'config> {
    /// 语言配置
    config: &'config DjangoLanguage,
}

impl<'config> DjangoParser<'config> {
    pub fn new(config: &'config DjangoLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<DjangoLanguage> for DjangoParser<'config> {
    fn parse_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<DjangoLanguage>,
    ) -> OakDiagnostics<Arc<GreenNode<DjangoSyntaxKind>>> {
        let mut state = ParserState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> DjangoParser<'config> {
    fn run<S: Source>(&self, state: &mut State<S>) -> Result<(), OakError> {
        // 简单的实现：创建一个根节点
        let root = GreenBuilder::<DjangoLanguage>::new(1).finish(DjangoSyntaxKind::Root);

        state.cache.last_parse = Some(root);
        Ok(())
    }
}

impl<'config> Clone for DjangoParser<'config> {
    fn clone(&self) -> Self {
        Self::new(self.config)
    }
}
