use crate::{kind::TwigSyntaxKind, language::TwigLanguage};
use oak_core::{
    GreenNode, IncrementalCache, OakError, Parser, errors::OakDiagnostics, parser::ParserState, source::Source, tree::Arc,
};

#[derive(Clone)]
pub struct TwigParser<'config> {
    config: &'config TwigLanguage,
}

type State<'a, S> = ParserState<'a, S, TwigLanguage>;

impl<'config> TwigParser<'config> {
    pub fn new(config: &'config TwigLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<TwigLanguage> for TwigParser<'config> {
    fn parse_incremental(
        &self,
        text: impl Source,
        changed: usize,
        cache: IncrementalCache<TwigLanguage>,
    ) -> OakDiagnostics<Arc<GreenNode<TwigSyntaxKind>>> {
        let mut state = ParserState::new_with_cache(text, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> TwigParser<'config> {
    fn run<S: Source>(&self, state: &mut State<S>) -> Result<(), OakError> {
        // 构建根节点
        let root = oak_core::GreenBuilder::<TwigLanguage>::new(1).token(TwigSyntaxKind::Eof, 0).finish(TwigSyntaxKind::Root);

        // 存储到 cache
        state.cache.last_parse = Some(root);

        Ok(())
    }
}
