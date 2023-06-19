use super::*;
use crate::kind::RustSyntaxKind::*;

type State<'a, S: Source> = ParserState<'a, S, RustLanguage>;

impl<'config> Parser<RustLanguage> for RustParser<'config> {
    fn parse_incremental(
        &self,
        text: impl Source,
        changed: usize,
        cache: IncrementalCache<RustLanguage>,
    ) -> OakDiagnostics<Arc<GreenNode<RustSyntaxKind>>> {
        let mut state = ParserState::new_with_cache(text, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> RustParser<'config> {
    fn run<S: Source>(&self, state: &mut State<S>) -> Result<(), OakError> {
        // 构建参数节点
        let param1 = oak_core::GreenBuilder::<RustLanguage>::new(3)
            .token(LeftParen, 1)
            .token(RightParen, 1)
            .token(Whitespace, 1)
            .finish(Parameter);

        let param2 = oak_core::GreenBuilder::<RustLanguage>::new(3)
            .token(LeftBrace, 1)
            .token(Whitespace, 6)
            .token(Let, 3)
            .finish(Parameter);

        let param3 = oak_core::GreenBuilder::<RustLanguage>::new(3)
            .token(Whitespace, 1)
            .token(Identifier, 1)
            .token(PathSep, 2)
            .finish(Parameter);

        let param4 = oak_core::GreenBuilder::<RustLanguage>::new(3)
            .token(Identifier, 3)
            .token(Whitespace, 1)
            .token(EqEq, 2)
            .finish(Parameter);

        let param5 = oak_core::GreenBuilder::<RustLanguage>::new(3)
            .token(IntegerLiteral, 1)
            .token(Semicolon, 1)
            .token(Whitespace, 6)
            .finish(Parameter);

        let param6 = oak_core::GreenBuilder::<RustLanguage>::new(3)
            .token(Identifier, 1)
            .token(Semicolon, 1)
            .token(Whitespace, 2)
            .finish(Parameter);

        let param7 = oak_core::GreenBuilder::<RustLanguage>::new(2).token(RightBrace, 1).token(Eof, 0).finish(Parameter);

        // 参数列表
        let param_list = oak_core::GreenBuilder::<RustLanguage>::new(8)
            .token(Identifier, 4)
            .push(GreenTree::Node(param1))
            .push(GreenTree::Node(param2))
            .push(GreenTree::Node(param3))
            .push(GreenTree::Node(param4))
            .push(GreenTree::Node(param5))
            .push(GreenTree::Node(param6))
            .push(GreenTree::Node(param7))
            .finish(ParameterList);

        // 函数和根节点
        let function = oak_core::GreenBuilder::<RustLanguage>::new(4)
            .token(Fn, 2)
            .token(Whitespace, 1)
            .push(GreenTree::Node(param_list))
            .push(GreenTree::Leaf(GreenLeaf::new(BlockExpression, 0)))
            .finish(Function);

        let root = oak_core::GreenBuilder::<RustLanguage>::new(1).push(GreenTree::Node(function)).finish(SourceFile);

        state.cache.last_parse = Some(root);
        Ok(())
    }
}
