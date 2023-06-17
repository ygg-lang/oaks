use crate::{kind::StylusSyntaxKind, language::StylusLanguage, lexer::StylusLexer};
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
    pub tree: Rc<GreenNode<StylusSyntaxKind>>,
    pub errors: Vec<ParseError>,
}

/// TOML 解析
pub struct TomlParser<'config> {
    pub(crate) config: &'config StylusLanguage,
}

impl<'config> TomlParser<'config> {
    pub fn new(config: &'config StylusLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<StylusLanguage> for TomlParser<'config> {
    fn parse(&self, source: &SourceText) -> OakDiagnostics<Rc<GreenNode<StylusSyntaxKind>>> {
        let lexer = StylusLexer::new(self.config);
        let tokens = lexer.lex(source);
        self.parse_tokens(source, &tokens.result.unwrap_or_default())
    }

    fn parse_tokens(
        &self,
        source: &SourceText,
        tokens: &[Token<StylusSyntaxKind>],
    ) -> OakDiagnostics<Rc<GreenNode<StylusSyntaxKind>>> {
        let mut state = ParserCtx { st: ParserState::new(source, tokens), config: self.config };

        let root = state.parse_root();
        state.st.diagnostics(root)
    }
}

impl<'config> IncrementalParser<StylusLanguage> for TomlParser<'config> {
    fn parse_incremental(
        &self,
        _cache: Option<Rc<GreenNode<StylusSyntaxKind>>>,
        source: &SourceText,
        _changed: usize,
    ) -> OakDiagnostics<Rc<GreenNode<StylusSyntaxKind>>> {
        // 简化实现：暂时不支持增量解析，直接进行完整解析
        self.parse(source)
    }
}

/// 解析器上下文
struct ParserCtx<'config> {
    st: ParserState<'config, StylusSyntaxKind>,
    #[allow(dead_code)]
    config: &'config StylusLanguage,
}

impl<'config> ParserCtx<'config> {
    fn parse_root(&mut self) -> Rc<GreenNode<StylusSyntaxKind>> {
        let mut b = GreenBuilder::<StylusSyntaxKind>::new();
        b = self.consume_trivia_into(b);
        while !self.at(StylusSyntaxKind::Eof) {
            let item = self.parse_item();
            b = b.push(GreenTree::Node(item));
            b = self.consume_trivia_into(b);
        }
        b.finish(StylusSyntaxKind::Root)
    }

    fn parse_item(&mut self) -> Rc<GreenNode<StylusSyntaxKind>> {
        self.consume_trivia();
        if self.at(StylusSyntaxKind::DoubleLeftBracket) {
            return self.parse_array_of_tables();
        }
        if self.at(StylusSyntaxKind::LeftBracket) {
            return self.parse_table();
        }
        if self.at(StylusSyntaxKind::BareKey)
            || self.at(StylusSyntaxKind::BasicString)
            || self.at(StylusSyntaxKind::LiteralString)
            || self.at(StylusSyntaxKind::MultilineBasicString)
            || self.at(StylusSyntaxKind::MultilineLiteralString)
        {
            return self.parse_key_value();
        }
        let pos = self.current_start();
        self.st.record_error_at(pos, "Unexpected kind");
        let mut b = GreenBuilder::new();
        b = self.add_current_token(b);
        self.skip_until_any(&[
            StylusSyntaxKind::LeftBracket,
            StylusSyntaxKind::DoubleLeftBracket,
            StylusSyntaxKind::BareKey,
            StylusSyntaxKind::BasicString,
            StylusSyntaxKind::LiteralString,
            StylusSyntaxKind::MultilineBasicString,
            StylusSyntaxKind::MultilineLiteralString,
        ]);
        b.finish(StylusSyntaxKind::ErrorNode)
    }

    fn parse_array_of_tables(&mut self) -> Rc<GreenNode<StylusSyntaxKind>> {
        let mut b = GreenBuilder::new();
        b = self.expect_token(StylusSyntaxKind::DoubleLeftBracket, b);
        b = self.consume_trivia_into(b);
        let key = self.parse_key();
        b = b.push(GreenTree::Node(key));
        b = self.consume_trivia_into(b);
        b = self.expect_token(StylusSyntaxKind::DoubleRightBracket, b);
        b.finish(StylusSyntaxKind::ArrayOfTables)
    }

    fn parse_table(&mut self) -> Rc<GreenNode<StylusSyntaxKind>> {
        let mut b = GreenBuilder::new();
        b = self.expect_token(StylusSyntaxKind::LeftBracket, b);
        b = self.consume_trivia_into(b);
        let key = self.parse_key();
        b = b.push(GreenTree::Node(key));
        b = self.consume_trivia_into(b);
        b = self.expect_token(StylusSyntaxKind::RightBracket, b);
        b.finish(StylusSyntaxKind::Table)
    }

    fn parse_key_value(&mut self) -> Rc<GreenNode<StylusSyntaxKind>> {
        let mut b = GreenBuilder::new();
        let key = self.parse_key();
        b = b.push(GreenTree::Node(key));
        b = self.consume_trivia_into(b);
        b = self.expect_token(StylusSyntaxKind::Equal, b);
        b = self.consume_trivia_into(b);
        let value = self.parse_value();
        b = b.push(GreenTree::Node(value));
        b.finish(StylusSyntaxKind::KeyValue)
    }

    fn parse_key(&mut self) -> Rc<GreenNode<StylusSyntaxKind>> {
        let mut b = GreenBuilder::new();
        let segment = self.parse_key_segment();
        b = b.push(GreenTree::Node(segment));
        while self.at(StylusSyntaxKind::Dot) {
            let (nb, _) = self.consume_token(StylusSyntaxKind::Dot, b);
            b = nb;
            b = self.consume_trivia_into(b);
            let segment = self.parse_key_segment();
            b = b.push(GreenTree::Node(segment));
        }
        b.finish(StylusSyntaxKind::Key)
    }

    fn parse_key_segment(&mut self) -> Rc<GreenNode<StylusSyntaxKind>> {
        if self.at(StylusSyntaxKind::BareKey) {
            return self.wrap_single(StylusSyntaxKind::BareKey);
        }
        if self.at(StylusSyntaxKind::BasicString)
            || self.at(StylusSyntaxKind::LiteralString)
            || self.at(StylusSyntaxKind::MultilineBasicString)
            || self.at(StylusSyntaxKind::MultilineLiteralString)
        {
            return self.wrap_single(StylusSyntaxKind::QuotedKey);
        }
        let pos = self.current_start();
        self.st.record_error_at(pos, "Unexpected kind");
        let mut b = GreenBuilder::new();
        b = self.add_current_token(b);
        b.finish(StylusSyntaxKind::ErrorNode)
    }

    fn parse_value(&mut self) -> Rc<GreenNode<StylusSyntaxKind>> {
        self.consume_trivia();
        if self.at(StylusSyntaxKind::BasicString) {
            return self.wrap_single(StylusSyntaxKind::BasicString);
        }
        if self.at(StylusSyntaxKind::LiteralString) {
            return self.wrap_single(StylusSyntaxKind::LiteralString);
        }
        if self.at(StylusSyntaxKind::MultilineBasicString) {
            return self.wrap_single(StylusSyntaxKind::MultilineBasicString);
        }
        if self.at(StylusSyntaxKind::MultilineLiteralString) {
            return self.wrap_single(StylusSyntaxKind::MultilineLiteralString);
        }
        if self.at(StylusSyntaxKind::Integer) {
            return self.wrap_single(StylusSyntaxKind::Integer);
        }
        if self.at(StylusSyntaxKind::Float) {
            return self.wrap_single(StylusSyntaxKind::Float);
        }
        if self.at(StylusSyntaxKind::Boolean) {
            return self.wrap_single(StylusSyntaxKind::Boolean);
        }
        if self.at(StylusSyntaxKind::OffsetDateTime) {
            return self.wrap_single(StylusSyntaxKind::OffsetDateTime);
        }
        if self.at(StylusSyntaxKind::LocalDateTime) {
            return self.wrap_single(StylusSyntaxKind::LocalDateTime);
        }
        if self.at(StylusSyntaxKind::LocalDate) {
            return self.wrap_single(StylusSyntaxKind::LocalDate);
        }
        if self.at(StylusSyntaxKind::LocalTime) {
            return self.wrap_single(StylusSyntaxKind::LocalTime);
        }
        if self.at(StylusSyntaxKind::LeftBracket) {
            return self.parse_array();
        }
        if self.at(StylusSyntaxKind::LeftBrace) {
            return self.parse_inline_table();
        }
        let pos = self.current_start();
        self.st.record_error_at(pos, "Unexpected kind");
        let mut b = GreenBuilder::new();
        b = self.add_current_token(b);
        self.skip_until_any(&[StylusSyntaxKind::Comma, StylusSyntaxKind::RightBracket, StylusSyntaxKind::RightBrace]);
        b.finish(StylusSyntaxKind::ErrorNode)
    }

    fn parse_array(&mut self) -> Rc<GreenNode<StylusSyntaxKind>> {
        let mut b = GreenBuilder::new();
        b = self.expect_token(StylusSyntaxKind::LeftBracket, b);
        loop {
            b = self.consume_trivia_into(b);
            if self.at(StylusSyntaxKind::RightBracket) {
                let (nb, _) = self.consume_token(StylusSyntaxKind::RightBracket, b);
                b = nb;
                break;
            }
            let value = self.parse_value();
            b = b.push(GreenTree::Node(value));
            b = self.consume_trivia_into(b);
            if self.at(StylusSyntaxKind::Comma) {
                let (nb, _) = self.consume_token(StylusSyntaxKind::Comma, b);
                b = nb;
                b = self.consume_trivia_into(b);
            }
            else if self.at(StylusSyntaxKind::RightBracket) {
                let (nb, _) = self.consume_token(StylusSyntaxKind::RightBracket, b);
                b = nb;
                break;
            }
            else if self.at(StylusSyntaxKind::Eof) {
                self.st.record_unexpected("Unexpected end of file");
                break;
            }
            else {
                let pos = self.current_start();
                self.st.record_error_at(pos, "Invalid kind");
                self.skip_until_any(&[StylusSyntaxKind::Comma, StylusSyntaxKind::RightBracket]);
            }
        }
        b.finish(StylusSyntaxKind::Array)
    }

    fn parse_inline_table(&mut self) -> Rc<GreenNode<StylusSyntaxKind>> {
        let mut b = GreenBuilder::new();
        b = self.expect_token(StylusSyntaxKind::LeftBrace, b);
        loop {
            b = self.consume_trivia_into(b);
            if self.at(StylusSyntaxKind::RightBrace) {
                let (nb, _) = self.consume_token(StylusSyntaxKind::RightBrace, b);
                b = nb;
                break;
            }
            let key_value = self.parse_key_value();
            b = b.push(GreenTree::Node(key_value));
            b = self.consume_trivia_into(b);
            if self.at(StylusSyntaxKind::Comma) {
                let (nb, _) = self.consume_token(StylusSyntaxKind::Comma, b);
                b = nb;
                b = self.consume_trivia_into(b);
            }
            else if self.at(StylusSyntaxKind::RightBrace) {
                let (nb, _) = self.consume_token(StylusSyntaxKind::RightBrace, b);
                b = nb;
                break;
            }
            else if self.at(StylusSyntaxKind::Eof) {
                self.st.record_unexpected("Unexpected end of file");
                break;
            }
            else {
                let pos = self.current_start();
                self.st.record_error_at(pos, "Invalid kind");
                self.skip_until_any(&[StylusSyntaxKind::Comma, StylusSyntaxKind::RightBrace]);
            }
        }
        b.finish(StylusSyntaxKind::InlineTable)
    }

    fn consume_token(
        &mut self,
        kind: StylusSyntaxKind,
        b: GreenBuilder<StylusSyntaxKind>,
    ) -> (GreenBuilder<StylusSyntaxKind>, bool) {
        if self.at(kind) {
            let len = self.current_len();
            self.st.advance();
            (b.token(kind, len), true)
        }
        else {
            (b, false)
        }
    }

    fn expect_token(&mut self, kind: StylusSyntaxKind, b: GreenBuilder<StylusSyntaxKind>) -> GreenBuilder<StylusSyntaxKind> {
        let (b2, ok) = self.consume_token(kind, b);
        if ok {
            b2
        }
        else {
            self.st.record_unexpected("Unexpected kind");
            let mut eb = GreenBuilder::new();
            eb = self.add_current_token(eb);
            let en = eb.finish(StylusSyntaxKind::ErrorNode);
            b2.push(GreenTree::Node(en))
        }
    }

    fn wrap_single(&mut self, node_kind: StylusSyntaxKind) -> Rc<GreenNode<StylusSyntaxKind>> {
        let mut b = GreenBuilder::new();
        b = self.add_current_token(b);
        b.finish(node_kind)
    }

    fn add_current_token(&mut self, b: GreenBuilder<StylusSyntaxKind>) -> GreenBuilder<StylusSyntaxKind> {
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

    fn consume_trivia_into(&mut self, mut b: GreenBuilder<StylusSyntaxKind>) -> GreenBuilder<StylusSyntaxKind> {
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

    fn skip_until_any(&mut self, kinds: &[StylusSyntaxKind]) {
        while let Some(t) = self.st.current() {
            if kinds.contains(&t.kind) {
                break;
            }
            self.st.advance();
        }
    }

    #[inline]
    fn at(&self, kind: StylusSyntaxKind) -> bool {
        self.st.peek_kind().map_or(false, |k| k == kind)
    }
    #[inline]
    fn current_kind(&self) -> StylusSyntaxKind {
        self.st.peek_kind().unwrap_or(StylusSyntaxKind::Eof)
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
