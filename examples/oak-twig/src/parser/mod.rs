use crate::{language::TwigLanguage, lexer::TwigLexer, syntax::TwigSyntaxKind};
use alloc::{rc::Rc, vec::Vec};
use oak_core::{
    IncrementalParser, Lexer, Parser, SourceText, SyntaxKind, Token,
    errors::OakDiagnostics,
    parser::ParserState,
    tree::{GreenBuilder, GreenNode, GreenTree},
};

/// Twig 解析错误
#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: alloc::string::String,
    pub position: usize,
}

/// Twig 解析输出
#[derive(Debug)]
pub struct TwigParseOutput {
    pub tree: Rc<GreenNode<TwigSyntaxKind>>,
    pub errors: Vec<ParseError>,
}

/// Twig 解析器
pub struct TwigParser<'config> {
    pub(crate) config: &'config TwigLanguage,
}

impl<'config> TwigParser<'config> {
    pub fn new(config: &'config TwigLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<TwigLanguage> for TwigParser<'config> {
    fn parse(&self, source: &SourceText) -> OakDiagnostics<Rc<GreenNode<TwigSyntaxKind>>> {
        let lexer = TwigLexer::new(self.config);
        let tokens_result = lexer.lex(source);
        match tokens_result.result {
            Ok(tokens) => self.parse_tokens(source, &tokens),
            Err(e) => OakDiagnostics { result: Err(e), diagnostics: tokens_result.diagnostics },
        }
    }

    fn parse_tokens(
        &self,
        source: &SourceText,
        tokens: &[Token<TwigSyntaxKind>],
    ) -> OakDiagnostics<Rc<GreenNode<TwigSyntaxKind>>> {
        let mut parser = TwigParserImpl::new(source, tokens, self.config);
        let tree = parser.parse_root();

        parser.st.diagnostics(tree)
    }
}

impl<'config> IncrementalParser<TwigLanguage> for TwigParser<'config> {
    fn parse_incremental(
        &self,
        _cache: Option<Rc<GreenNode<TwigSyntaxKind>>>,
        source: &SourceText,
        _changed: usize,
    ) -> OakDiagnostics<Rc<GreenNode<TwigSyntaxKind>>> {
        // 简单实现：忽略缓存，重新解析
        self.parse(source)
    }
}

struct TwigParserImpl<'config> {
    st: ParserState<'config, TwigSyntaxKind>,
    #[allow(dead_code)]
    config: &'config TwigLanguage,
}

impl<'config> TwigParserImpl<'config> {
    fn parse_root(&mut self) -> Rc<GreenNode<TwigSyntaxKind>> {
        let mut b = GreenBuilder::<TwigSyntaxKind>::new();

        while !self.at(TwigSyntaxKind::Eof) {
            let item = self.parse_item();
            b = b.push(GreenTree::Node(item));
        }

        b.finish(TwigSyntaxKind::Root)
    }

    fn parse_item(&mut self) -> Rc<GreenNode<TwigSyntaxKind>> {
        let mut b = GreenBuilder::<TwigSyntaxKind>::new();

        // 简单的 Twig 解析逻辑
        if self.at(TwigSyntaxKind::DoubleLeftBrace) {
            return self.parse_variable();
        }
        if self.at(TwigSyntaxKind::LeftBrace) {
            return self.parse_block();
        }
        if self.at(TwigSyntaxKind::Identifier) || self.at(TwigSyntaxKind::String) || self.at(TwigSyntaxKind::Number) {
            return self.parse_expression();
        }

        // 错误恢复
        self.skip_until_any(&[TwigSyntaxKind::DoubleLeftBrace, TwigSyntaxKind::LeftBrace, TwigSyntaxKind::Identifier]);
        b.finish(TwigSyntaxKind::ErrorNode)
    }

    fn parse_variable(&mut self) -> Rc<GreenNode<TwigSyntaxKind>> {
        let mut b = GreenBuilder::<TwigSyntaxKind>::new();
        b = self.expect_token(TwigSyntaxKind::DoubleLeftBrace, b);

        // 解析变量内容
        let expr = self.parse_expression();
        b = b.push(GreenTree::Node(expr));

        b = self.expect_token(TwigSyntaxKind::DoubleRightBrace, b);
        b.finish(TwigSyntaxKind::Variable)
    }

    fn parse_block(&mut self) -> Rc<GreenNode<TwigSyntaxKind>> {
        let mut b = GreenBuilder::<TwigSyntaxKind>::new();
        b = self.expect_token(TwigSyntaxKind::LeftBrace, b);

        // 解析块内容
        let expr = self.parse_expression();
        b = b.push(GreenTree::Node(expr));

        b = self.expect_token(TwigSyntaxKind::RightBrace, b);
        b.finish(TwigSyntaxKind::Block)
    }

    fn parse_expression(&mut self) -> Rc<GreenNode<TwigSyntaxKind>> {
        if self.at(TwigSyntaxKind::String) {
            return self.wrap_single(TwigSyntaxKind::String);
        }
        if self.at(TwigSyntaxKind::Number) {
            return self.wrap_single(TwigSyntaxKind::Number);
        }
        if self.at(TwigSyntaxKind::Boolean) {
            return self.wrap_single(TwigSyntaxKind::Boolean);
        }
        if self.at(TwigSyntaxKind::Null) {
            return self.wrap_single(TwigSyntaxKind::Null);
        }
        if self.at(TwigSyntaxKind::Identifier) {
            return self.wrap_single(TwigSyntaxKind::Identifier);
        }
        if self.at(TwigSyntaxKind::LeftBracket) {
            return self.parse_array();
        }
        if self.at(TwigSyntaxKind::LeftBrace) {
            return self.parse_object();
        }

        // 错误恢复
        let mut b = GreenBuilder::<TwigSyntaxKind>::new();
        self.skip_until_any(&[TwigSyntaxKind::Comma, TwigSyntaxKind::RightBracket, TwigSyntaxKind::RightBrace]);
        b.finish(TwigSyntaxKind::ErrorNode)
    }

    fn parse_array(&mut self) -> Rc<GreenNode<TwigSyntaxKind>> {
        let mut b = GreenBuilder::<TwigSyntaxKind>::new();
        b = self.expect_token(TwigSyntaxKind::LeftBracket, b);

        // 解析数组元素
        if self.at(TwigSyntaxKind::RightBracket) {
            let (nb, _) = self.consume_token(TwigSyntaxKind::RightBracket, b);
            return nb.finish(TwigSyntaxKind::Array);
        }

        loop {
            let expr = self.parse_expression();
            b = b.push(GreenTree::Node(expr));

            if self.at(TwigSyntaxKind::Comma) {
                let (nb, _) = self.consume_token(TwigSyntaxKind::Comma, b);
                b = nb;
                continue;
            }
            else if self.at(TwigSyntaxKind::RightBracket) {
                let (nb, _) = self.consume_token(TwigSyntaxKind::RightBracket, b);
                b = nb;
                break;
            }
            else if self.at(TwigSyntaxKind::Eof) {
                break;
            }
            else {
                // 错误恢复
                self.skip_until_any(&[TwigSyntaxKind::Comma, TwigSyntaxKind::RightBracket]);
            }
        }

        b.finish(TwigSyntaxKind::Array)
    }

    fn parse_object(&mut self) -> Rc<GreenNode<TwigSyntaxKind>> {
        let mut b = GreenBuilder::<TwigSyntaxKind>::new();
        b = self.expect_token(TwigSyntaxKind::LeftBrace, b);

        // 解析对象属性
        if self.at(TwigSyntaxKind::RightBrace) {
            let (nb, _) = self.consume_token(TwigSyntaxKind::RightBrace, b);
            return nb.finish(TwigSyntaxKind::Object);
        }

        loop {
            // 解析键值对
            let key = self.parse_expression();
            b = b.push(GreenTree::Node(key));

            b = self.expect_token(TwigSyntaxKind::Equal, b);

            let value = self.parse_expression();
            b = b.push(GreenTree::Node(value));

            if self.at(TwigSyntaxKind::Comma) {
                let (nb, _) = self.consume_token(TwigSyntaxKind::Comma, b);
                b = nb;
                continue;
            }
            else if self.at(TwigSyntaxKind::RightBrace) {
                let (nb, _) = self.consume_token(TwigSyntaxKind::RightBrace, b);
                b = nb;
                break;
            }
            else if self.at(TwigSyntaxKind::Eof) {
                break;
            }
            else {
                // 错误恢复
                self.skip_until_any(&[TwigSyntaxKind::Comma, TwigSyntaxKind::RightBrace]);
            }
        }

        b.finish(TwigSyntaxKind::Object)
    }

    fn consume_token(&mut self, kind: TwigSyntaxKind, b: GreenBuilder<TwigSyntaxKind>) -> (GreenBuilder<TwigSyntaxKind>, bool) {
        if self.at(kind) {
            let b = self.add_current_token(b);
            self.st.advance();
            (b, true)
        }
        else {
            (b, false)
        }
    }

    fn expect_token(&mut self, kind: TwigSyntaxKind, b: GreenBuilder<TwigSyntaxKind>) -> GreenBuilder<TwigSyntaxKind> {
        let (b, found) = self.consume_token(kind, b);
        if !found {
            // 添加错误节点
            let mut eb = GreenBuilder::<TwigSyntaxKind>::new();
            eb = self.add_current_token(eb);
            self.st.advance();
            let en = eb.finish(TwigSyntaxKind::ErrorNode);
            b.push(GreenTree::Node(en))
        }
        else {
            b
        }
    }

    fn wrap_single(&mut self, node_kind: TwigSyntaxKind) -> Rc<GreenNode<TwigSyntaxKind>> {
        let mut b = GreenBuilder::<TwigSyntaxKind>::new();
        b = self.add_current_token(b);
        self.st.advance();
        b.finish(node_kind)
    }

    fn add_current_token(&mut self, b: GreenBuilder<TwigSyntaxKind>) -> GreenBuilder<TwigSyntaxKind> {
        if let Some(token) = self.st.current() {
            let kind = token.kind;
            let len = token.span.end - token.span.start;
            self.st.advance();
            b.token(kind, len)
        }
        else {
            b
        }
    }

    fn new(source: &'config SourceText, tokens: &'config [Token<TwigSyntaxKind>], config: &'config TwigLanguage) -> Self {
        Self { st: ParserState::new(source, tokens), config }
    }

    fn consume_trivia_into(&mut self, mut b: GreenBuilder<TwigSyntaxKind>) -> GreenBuilder<TwigSyntaxKind> {
        while let Some(token) = self.st.current() {
            if token.kind.is_trivia() {
                let len = token.span.end - token.span.start;
                b = b.token(token.kind, len);
                self.st.advance();
            }
            else {
                break;
            }
        }
        b
    }

    fn skip_until_any(&mut self, kinds: &[TwigSyntaxKind]) {
        while !self.at(TwigSyntaxKind::Eof) {
            if kinds.contains(&self.current_kind()) {
                break;
            }
            self.st.advance();
        }
    }

    fn at(&self, kind: TwigSyntaxKind) -> bool {
        self.current_kind() == kind
    }

    fn current_kind(&self) -> TwigSyntaxKind {
        self.st.peek_kind().unwrap_or(TwigSyntaxKind::Eof)
    }
}
