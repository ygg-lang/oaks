use crate::{
    syntax::SqlSyntaxKind,
    language::SqlLanguage,
};
use alloc::{rc::Rc, vec, vec::Vec};
use oak_core::{
    IncrementalParser, Parser, SourceText,
    errors::OakDiagnostics,
    parser::{ParseOutput, ParserState},
    tree::{GreenBuilder, GreenLeaf, GreenNode, GreenTree},
};

/// JSON 解析
pub struct SqlParser<'config> {
    pub(crate) config: &'config SqlLanguage,
}

impl<'config> SqlParser<'config> {
    pub fn new(config: &'config SqlLanguage) -> Self {
        Self { config }
    }

    /// 尝试增量解析，返回新的语法树
    fn try_incremental_parse(
        &self,
        cached_tree: &GreenNode<SqlSyntaxKind>,
        state: &mut ParserCtx<'config>,
        changed_offset: usize,
    ) -> Result<Rc<GreenNode<SqlSyntaxKind>>, ()> {
        use oak_core::tree::RedNode;

        // 创建红树包装以便查找受影响的节点
        let red_root = RedNode::new(Rc::new(cached_tree.clone()), 0);

        // 查找包含变更位置的子节点
        let affected_indices = red_root.overlapping_indices((changed_offset..changed_offset + 1).into());

        if affected_indices.is_empty() {
            // 没有受影响的子节点，可能是在末尾添加内容
            return self.try_append_incremental(cached_tree);
        }

        // 找到第一个受影响的子节点
        let start_idx = affected_indices[0];

        // 确定需要重新解析的范围
        let reparse_start = red_root.offset_of_child(start_idx).unwrap_or(0);

        // 简单策略：重新解析从受影响节点开始到末尾的所有内
        state.advance_to_offset(reparse_start);

        // 解析新的内容
        let new_children = self.parse_remaining_children(state)?;

        // 构建新的根节点，保留未受影响的前缀
        let mut children = Vec::new();
        children.extend_from_slice(&cached_tree.children[..start_idx]);
        children.extend(new_children);

        let new_root = GreenNode::new(cached_tree.kind, children);
        Ok(new_root)
    }

    /// 解析从当前位置到末尾的所有子节点（用于增量重建）
    fn parse_remaining_children(&self, state: &mut ParserCtx<'config>) -> Result<Vec<GreenTree<SqlSyntaxKind>>, ()> {
        // 直接复用完整解析以简化实现：从当前位置解析为 Root，然后拿到其 children
        let new_root = state.parse_root();
        Ok(new_root.children.clone())
    }

    /// 尝试在末尾追加内容的增量解析
    fn try_append_incremental(&self, cached_tree: &GreenNode<SqlSyntaxKind>) -> Result<Rc<GreenNode<SqlSyntaxKind>>, ()> {
        // 简单实现：如果原树只有一个值节点，且新内容在末尾，则可能是追加
        // 这里为了简化，直接返回错误，回退到完整解
        Err(())
    }
}

impl<'config> IncrementalParser<SqlLanguage> for SqlParser<'config> {
    fn parse_incremental(
        &self,
        cache: Option<Rc<GreenNode<SqlSyntaxKind>>>,
        source: &SourceText,
        changed: usize,
    ) -> ParseOutput<SqlSyntaxKind> {
        // 如果没有缓存，直接进行完整解
        let Some(cached_tree) = cache
        else {
            return <Self as Parser<SqlLanguage>>::parse(self, source);
        };

        // 词法分析新的源代
        let lexer = JsonLexer::new(self.config);
        let lex: OakDiagnostics<Vec<JsonToken>> = lexer.tokenize_source(source);
        let OakDiagnostics { result: Ok(tokens), diagnostics: lex_diags } = lex
        else {
            // 词法分析失败，回退到完整解
            return <Self as Parser<SqlLanguage>>::parse(self, source);
        };

        // 创建解析状态用于增量解
        let mut state = ParserCtx { st: ParserState::new(source, &tokens), config: self.config };

        // 尝试增量解析
        match self.try_incremental_parse(&cached_tree, &mut state, changed) {
            Ok(new_root) => {
                let mut out = state.st.diagnostics(new_root);
                out.diagnostics.extend(lex_diags);
                out
            }
            Err(_) => {
                // 增量解析失败，回退到完整解
                let mut out = <Self as Parser<SqlLanguage>>::parse_tokens(self, source, &tokens);
                out.diagnostics.extend(lex_diags);
                out
            }
        }
    }
}

/// 具体解析过程
struct ParserCtx<'config> {
    st: ParserState<'config, SqlSyntaxKind>,
    config: &'config SqlLanguage,
}

impl<'config> ParserCtx<'config> {
    fn parse_root(&mut self) -> Rc<GreenNode<SqlSyntaxKind>> {
        let mut b = GreenBuilder::<SqlSyntaxKind>::new();
        b = self.consume_trivia_into(b);
        let value = self.parse_value();
        b = b.push(GreenTree::Node(value));
        b = self.consume_trivia_into(b);
        while !self.at(SqlSyntaxKind::Eof) {
            self.st.record_unexpected("Unexpected kind");
            let mut eb = GreenBuilder::new();
            eb = self.add_current_token(eb);
            let en = eb.finish(SqlSyntaxKind::ErrorNode);
            b = b.push(GreenTree::Node(en));
            b = self.consume_trivia_into(b);
        }
        b.finish(SqlSyntaxKind::Root)
    }

    fn parse_value(&mut self) -> Rc<GreenNode<SqlSyntaxKind>> {
        self.consume_trivia();
        if self.at(SqlSyntaxKind::LeftBrace) {
            return self.parse_object();
        }
        if self.at(SqlSyntaxKind::LeftBracket) {
            return self.parse_array();
        }
        if self.at(SqlSyntaxKind::String) {
            return self.wrap_single(SqlSyntaxKind::String);
        }
        if self.at(SqlSyntaxKind::Number) {
            return self.wrap_single(SqlSyntaxKind::Number);
        }
        if self.at(SqlSyntaxKind::Boolean) {
            return self.wrap_single(SqlSyntaxKind::Boolean);
        }
        if self.at(SqlSyntaxKind::Null) {
            return self.wrap_single(SqlSyntaxKind::Null);
        }
        // 错误恢复：生成错误节点，跳到下一个值起始或分隔
        let pos = self.current_start();
        self.st.record_error_at(pos, "Unexpected kind");
        let mut b = GreenBuilder::new();
        b = self.add_current_token(b);
        // 跳过直到稳定
        self.skip_until_any(&[SqlSyntaxKind::Comma, SqlSyntaxKind::RightBrace, SqlSyntaxKind::RightBracket]);
        b.finish(SqlSyntaxKind::ErrorNode)
    }

    fn parse_object(&mut self) -> Rc<GreenNode<SqlSyntaxKind>> {
        let mut b = GreenBuilder::new();
        b = self.expect_token(SqlSyntaxKind::LeftBrace, b);
        loop {
            b = self.consume_trivia_into(b);
            if self.at(SqlSyntaxKind::RightBrace) {
                let (nb, _) = self.consume_token(SqlSyntaxKind::RightBrace, b);
                b = nb;
                break;
            }
            let entry = self.parse_object_entry();
            b = b.push(GreenTree::Node(entry));
            b = self.consume_trivia_into(b);
            if self.at(SqlSyntaxKind::Comma) {
                let (nb, _) = self.consume_token(SqlSyntaxKind::Comma, b);
                b = nb;
                b = self.consume_trivia_into(b);
                if self.at(SqlSyntaxKind::RightBrace) && !self.config.trailing_comma {
                    let pos = self.current_start();
                    self.st.record_error_at(pos, "Trailing comma not allowed");
                }
            }
            else if self.at(SqlSyntaxKind::RightBrace) {
                let (nb, _) = self.consume_token(SqlSyntaxKind::RightBrace, b);
                b = nb;
                break;
            }
            else if self.at(SqlSyntaxKind::Eof) {
                self.st.record_error_at(self.current_start(), "Unexpected end of file");
                break;
            }
            else {
                let pos = self.current_start();
                self.st.record_error_at(pos, "Invalid kind");
                self.skip_until_any(&[SqlSyntaxKind::Comma, SqlSyntaxKind::RightBrace]);
            }
        }
        b.finish(SqlSyntaxKind::Object)
    }

    fn parse_object_entry(&mut self) -> Rc<GreenNode<SqlSyntaxKind>> {
        let mut eb = GreenBuilder::new();
        eb = self.consume_trivia_into(eb);
        if self.at(SqlSyntaxKind::String) {
            let (nb, _) = self.consume_token(SqlSyntaxKind::String, eb);
            eb = nb;
        }
        else {
            let pos = self.current_start();
            self.st.record_error_at(pos, "Unexpected kind");
            self.skip_until_any(&[SqlSyntaxKind::Colon, SqlSyntaxKind::Comma, SqlSyntaxKind::RightBrace]);
        }
        eb = self.consume_trivia_into(eb);
        eb = self.expect_token(SqlSyntaxKind::Colon, eb);
        eb = self.consume_trivia_into(eb);
        let val = self.parse_value();
        eb = eb.push(GreenTree::Node(val));
        eb.finish(SqlSyntaxKind::ObjectEntry)
    }

    fn parse_array(&mut self) -> Rc<GreenNode<SqlSyntaxKind>> {
        let mut b = GreenBuilder::new();
        b = self.expect_token(SqlSyntaxKind::LeftBracket, b);
        loop {
            b = self.consume_trivia_into(b);
            if self.at(SqlSyntaxKind::RightBracket) {
                let (nb, _) = self.consume_token(SqlSyntaxKind::RightBracket, b);
                b = nb;
                break;
            }
            let val = self.parse_value();
            b = b.push(GreenTree::Node(val));
            b = self.consume_trivia_into(b);
            if self.at(SqlSyntaxKind::Comma) {
                let (nb, _) = self.consume_token(SqlSyntaxKind::Comma, b);
                b = nb;
                b = self.consume_trivia_into(b);
                if self.at(SqlSyntaxKind::RightBracket) && !self.config.trailing_comma {
                    let pos = self.current_start();
                    self.st.record_error_at(pos, "Trailing comma not allowed");
                }
            }
            else if self.at(SqlSyntaxKind::RightBracket) {
                let (nb, _) = self.consume_token(SqlSyntaxKind::RightBracket, b);
                b = nb;
                break;
            }
            else if self.at(SqlSyntaxKind::Eof) {
                self.st.record_error_at(self.current_start(), "Unexpected end of file");
                break;
            }
            else {
                let pos = self.current_start();
                self.st.record_error_at(pos, "Invalid kind");
                self.skip_until_any(&[SqlSyntaxKind::Comma, SqlSyntaxKind::RightBracket]);
            }
        }
        b.finish(SqlSyntaxKind::Array)
    }

    fn consume_token(&mut self, kind: SqlSyntaxKind, b: GreenBuilder<SqlSyntaxKind>) -> (GreenBuilder<SqlSyntaxKind>, bool) {
        if self.at(kind) {
            let len = self.current_len();
            self.st.advance();
            (b.token(SqlSyntaxKind::from(kind), len), true)
        }
        else {
            (b, false)
        }
    }

    fn expect_token(&mut self, kind: SqlSyntaxKind, b: GreenBuilder<SqlSyntaxKind>) -> GreenBuilder<SqlSyntaxKind> {
        let (b2, ok) = self.consume_token(kind, b);
        if ok {
            b2
        }
        else {
            self.st.record_unexpected("Unexpected kind");
            let mut eb = GreenBuilder::new();
            eb = self.add_current_token(eb);
            let en = eb.finish(SqlSyntaxKind::ErrorNode);
            b2.push(GreenTree::Node(en))
        }
    }

    fn wrap_single(&mut self, node_kind: SqlSyntaxKind) -> Rc<GreenNode<SqlSyntaxKind>> {
        let mut b = GreenBuilder::new();
        b = self.add_current_token(b);
        b.finish(node_kind)
    }

    fn add_current_token(&mut self, b: GreenBuilder<SqlSyntaxKind>) -> GreenBuilder<SqlSyntaxKind> {
        if let Some(k) = self.st.peek_kind() {
            let len = self.current_len();
            self.st.advance();
            b.token(SqlSyntaxKind::from(k), len)
        }
        else {
            b
        }
    }

    fn consume_trivia_into(&mut self, mut b: GreenBuilder<SqlSyntaxKind>) -> GreenBuilder<SqlSyntaxKind> {
        loop {
            let k = self.current_kind();
            if matches!(k, SqlSyntaxKind::Whitespace | SqlSyntaxKind::Comment) {
                let len = self.current_len();
                self.st.advance();
                b = b.token(SqlSyntaxKind::from(k), len);
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
            if matches!(k, SqlSyntaxKind::Whitespace | SqlSyntaxKind::Comment) {
                self.st.advance();
            }
            else {
                break;
            }
        }
    }

    fn skip_until_any(&mut self, kinds: &[SqlSyntaxKind]) {
        loop {
            let k = self.current_kind();
            if kinds.contains(&k) || k == SqlSyntaxKind::Eof {
                break;
            }
            self.st.advance();
        }
    }

    fn advance_to_offset(&mut self, offset: usize) {
        while let Some(tok) = self.st.current() {
            if tok.span.start < offset {
                self.st.advance();
            }
            else {
                break;
            }
        }
    }

    #[inline]
    fn at(&self, kind: SqlSyntaxKind) -> bool {
        // 使用 current_kind() 以确保在越界时识EOF
        self.current_kind() == kind
    }
    #[inline]
    fn current_kind(&self) -> SqlSyntaxKind {
        self.st.peek_kind().unwrap_or(SqlSyntaxKind::Eof)
    }
    #[inline]
    fn current_len(&self) -> usize {
        self.st.current().map(|t| t.length()).unwrap_or(0)
    }
    #[inline]
    fn current_start(&self) -> usize {
        self.st.current().map(|t| t.span.start).or_else(|| self.st.previous().map(|t| t.span.end)).unwrap_or(0)
    }
}

impl<'config> Parser<SqlLanguage> for SqlParser<'config> {
    fn parse(&self, source: &SourceText) -> ParseOutput<SqlSyntaxKind> {
        // 先进行词法分析并收集词法诊断
        let lexer = JsonLexer::new(self.config);
        let diag: OakDiagnostics<Vec<JsonToken>> = lexer.tokenize_source(source);
        let OakDiagnostics { result: lex_result, diagnostics: mut lex_diags } = diag;
        match lex_result {
            Ok(tokens_vec) => {
                let mut state = ParserCtx { st: ParserState::new(source, &tokens_vec), config: self.config };
                let root = state.parse_root();
                // 将解析结果与词法诊断统一打包返回
                let mut out = state.st.diagnostics(root);
                out.diagnostics.extend(lex_diags);
                out
            }
            Err(e) => OakDiagnostics { result: Err(e), diagnostics: lex_diags },
        }
    }

    fn parse_tokens(&self, source: &SourceText, tokens: &[oak_core::Token<SqlSyntaxKind>]) -> ParseOutput<SqlSyntaxKind> {
        let mut state = ParserCtx { st: ParserState::new(source, tokens), config: self.config };
        let root = state.parse_root();
        state.st.diagnostics(root)
    }
}
