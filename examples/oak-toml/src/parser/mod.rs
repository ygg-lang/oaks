use crate::{language::TomlLanguage, lexer::TomlLexer, syntax::TomlSyntaxKind};
use alloc::{rc::Rc, vec::Vec};
use oak_core::{
    IncrementalParser, Lexer, Parser, SourceText, SyntaxKind, Token,
    errors::OakDiagnostics,
    parser::ParserState,
    tree::{GreenBuilder, GreenNode, GreenTree},
};

/// TOML 解析错误
#[derive(Debug, Clone)]
pub enum ParseError {
    InvalidSyntax { start: usize, end: usize },
    UnexpectedToken { position: usize },
    UnexpectedEof,
}

/// TOML 解析输出
#[derive(Debug)]
pub struct TomlParseOutput {
    pub tree: Rc<GreenNode<TomlSyntaxKind>>,
    pub errors: Vec<ParseError>,
}

/// TOML 解析
pub struct TomlParser<'config> {
    pub(crate) config: &'config TomlLanguage,
}

impl<'config> TomlParser<'config> {
    pub fn new(config: &'config TomlLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<TomlLanguage> for TomlParser<'config> {
    fn parse(&self, source: &SourceText) -> OakDiagnostics<Rc<GreenNode<TomlSyntaxKind>>> {
        let lexer = TomlLexer::new(self.config);
        let tokens = lexer.lex(source);
        self.parse_tokens(source, &tokens.result.unwrap_or_default())
    }

    fn parse_tokens(
        &self,
        source: &SourceText,
        tokens: &[Token<TomlSyntaxKind>],
    ) -> OakDiagnostics<Rc<GreenNode<TomlSyntaxKind>>> {
        let mut state = ParserCtx { st: ParserState::new(source, tokens), config: self.config };

        let root = state.parse_root();
        state.st.diagnostics(root)
    }
}

impl<'config> IncrementalParser<TomlLanguage> for TomlParser<'config> {
    fn parse_incremental(
        &self,
        _cache: Option<Rc<GreenNode<TomlSyntaxKind>>>,
        source: &SourceText,
        _changed: usize,
    ) -> OakDiagnostics<Rc<GreenNode<TomlSyntaxKind>>> {
        // 简化实现：暂时不支持增量解析，直接进行完整解析
        self.parse(source)
    }
}

/// 解析器上下文
struct ParserCtx<'config> {
    st: ParserState<'config, TomlSyntaxKind>,
    #[allow(dead_code)]
    config: &'config TomlLanguage,
}

impl<'config> ParserCtx<'config> {
    fn parse_root(&mut self) -> Rc<GreenNode<TomlSyntaxKind>> {
        let mut b = GreenBuilder::<TomlSyntaxKind>::new();
        b = self.consume_trivia_into(b);
        while !self.at(TomlSyntaxKind::Eof) {
            let item = self.parse_item();
            b = b.push(GreenTree::Node(item));
            b = self.consume_trivia_into(b);
        }
        b.finish(TomlSyntaxKind::Root)
    }

    fn parse_item(&mut self) -> Rc<GreenNode<TomlSyntaxKind>> {
        self.consume_trivia();
        if self.at(TomlSyntaxKind::DoubleLeftBracket) {
            return self.parse_array_of_tables();
        }
        if self.at(TomlSyntaxKind::LeftBracket) {
            return self.parse_table();
        }
        if self.at(TomlSyntaxKind::BareKey)
            || self.at(TomlSyntaxKind::BasicString)
            || self.at(TomlSyntaxKind::LiteralString)
            || self.at(TomlSyntaxKind::MultilineBasicString)
            || self.at(TomlSyntaxKind::MultilineLiteralString)
        {
            return self.parse_key_value();
        }
        let pos = self.current_start();
        self.st.record_error_at(pos, "Unexpected kind");
        let mut b = GreenBuilder::new();
        b = self.add_current_token(b);
        self.skip_until_any(&[
            TomlSyntaxKind::LeftBracket,
            TomlSyntaxKind::DoubleLeftBracket,
            TomlSyntaxKind::BareKey,
            TomlSyntaxKind::BasicString,
            TomlSyntaxKind::LiteralString,
            TomlSyntaxKind::MultilineBasicString,
            TomlSyntaxKind::MultilineLiteralString,
        ]);
        b.finish(TomlSyntaxKind::ErrorNode)
    }

    fn parse_array_of_tables(&mut self) -> Rc<GreenNode<TomlSyntaxKind>> {
        let mut b = GreenBuilder::new();
        b = self.expect_token(TomlSyntaxKind::DoubleLeftBracket, b);
        b = self.consume_trivia_into(b);
        let key = self.parse_key();
        b = b.push(GreenTree::Node(key));
        b = self.consume_trivia_into(b);
        b = self.expect_token(TomlSyntaxKind::DoubleRightBracket, b);
        b.finish(TomlSyntaxKind::ArrayOfTables)
    }

    fn parse_table(&mut self) -> Rc<GreenNode<TomlSyntaxKind>> {
        let mut b = GreenBuilder::new();
        b = self.expect_token(TomlSyntaxKind::LeftBracket, b);
        b = self.consume_trivia_into(b);
        let key = self.parse_key();
        b = b.push(GreenTree::Node(key));
        b = self.consume_trivia_into(b);
        b = self.expect_token(TomlSyntaxKind::RightBracket, b);
        b.finish(TomlSyntaxKind::Table)
    }

    fn parse_key_value(&mut self) -> Rc<GreenNode<TomlSyntaxKind>> {
        let mut b = GreenBuilder::new();
        let key = self.parse_key();
        b = b.push(GreenTree::Node(key));
        b = self.consume_trivia_into(b);
        b = self.expect_token(TomlSyntaxKind::Equal, b);
        b = self.consume_trivia_into(b);
        let value = self.parse_value();
        b = b.push(GreenTree::Node(value));
        b.finish(TomlSyntaxKind::KeyValue)
    }

    fn parse_key(&mut self) -> Rc<GreenNode<TomlSyntaxKind>> {
        let mut b = GreenBuilder::new();
        let segment = self.parse_key_segment();
        b = b.push(GreenTree::Node(segment));
        while self.at(TomlSyntaxKind::Dot) {
            let (nb, _) = self.consume_token(TomlSyntaxKind::Dot, b);
            b = nb;
            b = self.consume_trivia_into(b);
            let segment = self.parse_key_segment();
            b = b.push(GreenTree::Node(segment));
        }
        b.finish(TomlSyntaxKind::Key)
    }

    fn parse_key_segment(&mut self) -> Rc<GreenNode<TomlSyntaxKind>> {
        if self.at(TomlSyntaxKind::BareKey) {
            return self.wrap_single(TomlSyntaxKind::BareKey);
        }
        if self.at(TomlSyntaxKind::BasicString)
            || self.at(TomlSyntaxKind::LiteralString)
            || self.at(TomlSyntaxKind::MultilineBasicString)
            || self.at(TomlSyntaxKind::MultilineLiteralString)
        {
            return self.wrap_single(TomlSyntaxKind::QuotedKey);
        }
        let pos = self.current_start();
        self.st.record_error_at(pos, "Unexpected kind");
        let mut b = GreenBuilder::new();
        b = self.add_current_token(b);
        b.finish(TomlSyntaxKind::ErrorNode)
    }

    fn parse_value(&mut self) -> Rc<GreenNode<TomlSyntaxKind>> {
        self.consume_trivia();
        if self.at(TomlSyntaxKind::BasicString) {
            return self.wrap_single(TomlSyntaxKind::BasicString);
        }
        if self.at(TomlSyntaxKind::LiteralString) {
            return self.wrap_single(TomlSyntaxKind::LiteralString);
        }
        if self.at(TomlSyntaxKind::MultilineBasicString) {
            return self.wrap_single(TomlSyntaxKind::MultilineBasicString);
        }
        if self.at(TomlSyntaxKind::MultilineLiteralString) {
            return self.wrap_single(TomlSyntaxKind::MultilineLiteralString);
        }
        if self.at(TomlSyntaxKind::Integer) {
            return self.wrap_single(TomlSyntaxKind::Integer);
        }
        if self.at(TomlSyntaxKind::Float) {
            return self.wrap_single(TomlSyntaxKind::Float);
        }
        if self.at(TomlSyntaxKind::Boolean) {
            return self.wrap_single(TomlSyntaxKind::Boolean);
        }
        if self.at(TomlSyntaxKind::OffsetDateTime) {
            return self.wrap_single(TomlSyntaxKind::OffsetDateTime);
        }
        if self.at(TomlSyntaxKind::LocalDateTime) {
            return self.wrap_single(TomlSyntaxKind::LocalDateTime);
        }
        if self.at(TomlSyntaxKind::LocalDate) {
            return self.wrap_single(TomlSyntaxKind::LocalDate);
        }
        if self.at(TomlSyntaxKind::LocalTime) {
            return self.wrap_single(TomlSyntaxKind::LocalTime);
        }
        if self.at(TomlSyntaxKind::LeftBracket) {
            return self.parse_array();
        }
        if self.at(TomlSyntaxKind::LeftBrace) {
            return self.parse_inline_table();
        }
        let pos = self.current_start();
        self.st.record_error_at(pos, "Unexpected kind");
        let mut b = GreenBuilder::new();
        b = self.add_current_token(b);
        self.skip_until_any(&[TomlSyntaxKind::Comma, TomlSyntaxKind::RightBracket, TomlSyntaxKind::RightBrace]);
        b.finish(TomlSyntaxKind::ErrorNode)
    }

    fn parse_array(&mut self) -> Rc<GreenNode<TomlSyntaxKind>> {
        let mut b = GreenBuilder::new();
        b = self.expect_token(TomlSyntaxKind::LeftBracket, b);
        loop {
            b = self.consume_trivia_into(b);
            if self.at(TomlSyntaxKind::RightBracket) {
                let (nb, _) = self.consume_token(TomlSyntaxKind::RightBracket, b);
                b = nb;
                break;
            }
            let value = self.parse_value();
            b = b.push(GreenTree::Node(value));
            b = self.consume_trivia_into(b);
            if self.at(TomlSyntaxKind::Comma) {
                let (nb, _) = self.consume_token(TomlSyntaxKind::Comma, b);
                b = nb;
                b = self.consume_trivia_into(b);
            }
            else if self.at(TomlSyntaxKind::RightBracket) {
                let (nb, _) = self.consume_token(TomlSyntaxKind::RightBracket, b);
                b = nb;
                break;
            }
            else if self.at(TomlSyntaxKind::Eof) {
                self.st.record_unexpected("Unexpected end of file");
                break;
            }
            else {
                let pos = self.current_start();
                self.st.record_error_at(pos, "Invalid kind");
                self.skip_until_any(&[TomlSyntaxKind::Comma, TomlSyntaxKind::RightBracket]);
            }
        }
        b.finish(TomlSyntaxKind::Array)
    }

    fn parse_inline_table(&mut self) -> Rc<GreenNode<TomlSyntaxKind>> {
        let mut b = GreenBuilder::new();
        b = self.expect_token(TomlSyntaxKind::LeftBrace, b);
        loop {
            b = self.consume_trivia_into(b);
            if self.at(TomlSyntaxKind::RightBrace) {
                let (nb, _) = self.consume_token(TomlSyntaxKind::RightBrace, b);
                b = nb;
                break;
            }
            let key_value = self.parse_key_value();
            b = b.push(GreenTree::Node(key_value));
            b = self.consume_trivia_into(b);
            if self.at(TomlSyntaxKind::Comma) {
                let (nb, _) = self.consume_token(TomlSyntaxKind::Comma, b);
                b = nb;
                b = self.consume_trivia_into(b);
            }
            else if self.at(TomlSyntaxKind::RightBrace) {
                let (nb, _) = self.consume_token(TomlSyntaxKind::RightBrace, b);
                b = nb;
                break;
            }
            else if self.at(TomlSyntaxKind::Eof) {
                self.st.record_unexpected("Unexpected end of file");
                break;
            }
            else {
                let pos = self.current_start();
                self.st.record_error_at(pos, "Invalid kind");
                self.skip_until_any(&[TomlSyntaxKind::Comma, TomlSyntaxKind::RightBrace]);
            }
        }
        b.finish(TomlSyntaxKind::InlineTable)
    }

    fn consume_token(&mut self, kind: TomlSyntaxKind, b: GreenBuilder<TomlSyntaxKind>) -> (GreenBuilder<TomlSyntaxKind>, bool) {
        if self.at(kind) {
            let len = self.current_len();
            self.st.advance();
            (b.token(kind, len), true)
        }
        else {
            (b, false)
        }
    }

    fn expect_token(&mut self, kind: TomlSyntaxKind, b: GreenBuilder<TomlSyntaxKind>) -> GreenBuilder<TomlSyntaxKind> {
        let (b2, ok) = self.consume_token(kind, b);
        if ok {
            b2
        }
        else {
            self.st.record_unexpected("Unexpected kind");
            let mut eb = GreenBuilder::new();
            eb = self.add_current_token(eb);
            let en = eb.finish(TomlSyntaxKind::ErrorNode);
            b2.push(GreenTree::Node(en))
        }
    }

    fn wrap_single(&mut self, node_kind: TomlSyntaxKind) -> Rc<GreenNode<TomlSyntaxKind>> {
        let mut b = GreenBuilder::new();
        b = self.add_current_token(b);
        b.finish(node_kind)
    }

    fn add_current_token(&mut self, b: GreenBuilder<TomlSyntaxKind>) -> GreenBuilder<TomlSyntaxKind> {
        if let Some(t) = self.st.current() {
            let len = t.length();
            let k = t.kind;
            self.st.advance();
            b.token(k, len)
        }
        else {
            b
        }
    }

    fn consume_trivia_into(&mut self, mut b: GreenBuilder<TomlSyntaxKind>) -> GreenBuilder<TomlSyntaxKind> {
        loop {
            let k = self.current_kind();
            if k.is_trivia() {
                let len = self.current_len();
                self.st.advance();
                b = b.token(k, len);
            }
            else {
                break;
            }
        }
        b
    }

    fn consume_trivia(&mut self) {
        loop {
            let k = self.current_kind();
            if k.is_trivia() {
                self.st.advance();
            }
            else {
                break;
            }
        }
    }

    fn skip_until_any(&mut self, kinds: &[TomlSyntaxKind]) {
        while let Some(t) = self.st.current() {
            if kinds.contains(&t.kind) {
                break;
            }
            self.st.advance();
        }
    }

    #[inline]
    fn at(&self, kind: TomlSyntaxKind) -> bool {
        self.st.peek_kind().map_or(false, |k| k == kind)
    }
    #[inline]
    fn current_kind(&self) -> TomlSyntaxKind {
        self.st.peek_kind().unwrap_or(TomlSyntaxKind::Eof)
    }
    #[inline]
    fn current_len(&self) -> usize {
        self.st.current().map(|t| t.length()).unwrap_or(0)
    }
    #[inline]
    fn current_start(&self) -> usize {
        self.st.current().map(|t| t.span.start).unwrap_or(0)
    }
}
