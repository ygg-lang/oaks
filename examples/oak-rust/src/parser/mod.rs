use crate::{RustLanguage, RustLexer, RustSyntaxKind};
use oak_core::{GreenNode, SourceText, Token, Lexer, SyntaxKind, parser::{Parser, ParserState, ParseOutput, PrattParser}, tree::{GreenBuilder, GreenTree}, errors::OakDiagnostics, IncrementalParser};
use alloc::{rc::Rc, vec::Vec};

/// Rust 语言解析器（不可变），通过 &mut ParserState 推进
pub struct RustParser<'config> {
    /// 语言配置
    config: &'config RustLanguage,
    /// 表达式解析器（占位，后续启用）
    pratt: PrattParser<RustSyntaxKind>,
}


impl<'config> Parser<RustLanguage> for RustParser<'config> {
    fn parse(&self, source: &SourceText) -> ParseOutput<RustSyntaxKind> {
        // 先词法，再用 &mut ParserState 推进解析
        let lexer = RustLexer::new(self.config);
        let OakDiagnostics { result, diagnostics: lex_diags } = lexer.lex(source);
        let tokens = match result {
            Ok(t) => t,
            Err(e) => {
                return OakDiagnostics { result: Err(e), diagnostics: lex_diags };
            }
        };
        let mut state = ParserState::new(source, &tokens);
        let root = self.parse_source_file_impl(&mut state);
        OakDiagnostics { result: Ok(root), diagnostics: lex_diags }
    }

    fn parse_tokens(&self, source: &SourceText, tokens: &[Token<RustSyntaxKind>]) -> ParseOutput<RustSyntaxKind> {
        let mut state = ParserState::new(source, tokens);
        let root = self.parse_source_file_impl(&mut state);
        OakDiagnostics { result: Ok(root), diagnostics: Vec::new() }
    }
}


impl<'config> IncrementalParser<RustLanguage> for RustParser<'config> {
    fn parse_incremental(&self, cache: Option<Rc<GreenNode<RustLanguage::SyntaxKind>>>, source: &SourceText, changed: usize) -> ParseOutput<RustLanguage::SyntaxKind> {
        todo!()
    }
}


impl<'config> RustParser<'config> {
    /// 创建不可变的 Rust 解析器，仅保存配置与表达式解析器
    pub fn new(config: &'config RustLanguage) -> Self {
        let mut pratt = PrattParser::<RustSyntaxKind>::new();
        Self::configure_operators(&mut pratt);
        Self { config, pratt }
    }

    /// 配置操作符优先级（占位：等待 PrattParser 暴露注册 API）
    fn configure_operators(_pratt: &mut PrattParser<RustSyntaxKind>) {
        // TODO: 当 PrattParser 提供 infix/prefix 注册 API 后在此配置
        // use RustSyntaxKind::*;
        // 示例：赋值（右结合，最低优先级）
        // _pratt.infix(Eq, OperatorInfo::right(1));
    }

    /// 解析源文件（便捷方法）
    pub fn parse_source_file(source: &str) -> Option<Rc<GreenNode<RustSyntaxKind>>> {
        let source_text = SourceText::new(source);
        let language = RustLanguage {};
        let parser = RustParser::new(&language);
        let OakDiagnostics { result, .. } = parser.parse(&source_text);
        result.ok()
    }

    /// 跳过空白和注释
    fn skip_trivia(&self, state: &mut ParserState<'_, RustSyntaxKind>) {
        while let Some(kind) = state.peek_kind() {
            if kind.is_trivia() {
                state.advance();
            } else {
                break;
            }
        }
    }

    /// 错误恢复：跳到下一个项目开始
    fn recover_to_item_start(&self, state: &mut ParserState<'_, RustSyntaxKind>) -> bool {
        use RustSyntaxKind::*;
        while !state.is_at_end() {
            match state.peek_kind() {
                Some(Fn | Struct | Enum | Impl | Mod | Use | Const | Static | Type | Trait) => return true,
                Some(Eof) => return false,
                _ => {
                    state.advance();
                }
            }
        }
        false
    }

    /// 错误恢复：跳到下一个语句开始
    fn recover_to_statement_start(&self, state: &mut ParserState<'_, RustSyntaxKind>) -> bool {
        use RustSyntaxKind::*;
        while !state.is_at_end() {
            match state.peek_kind() {
                Some(Let | RightBrace | Semicolon) => return true,
                Some(Eof) => return false,
                _ => {
                    state.advance();
                }
            }
        }
        false
    }

    /// 解析源文件实现：不可变解析器 + 可变状态
    fn parse_source_file_impl(&self, state: &mut ParserState<'_, RustSyntaxKind>) -> Rc<GreenNode<RustSyntaxKind>> {
        let mut builder = GreenBuilder::new();
        self.skip_trivia(state);
        while !state.is_at_end() && state.peek_kind() != Some(RustSyntaxKind::Eof) {
            let item = self.parse_item(state);
            builder = builder.push(GreenTree::Node(item));
            self.skip_trivia(state);
        }
        builder.finish(RustSyntaxKind::SourceFile)
    }

    /// 解析项目 (函数、结构体、模块等)
    fn parse_item(&self, state: &mut ParserState<'_, RustSyntaxKind>) -> Rc<GreenNode<RustSyntaxKind>> {
        use RustSyntaxKind::*;
        match state.peek_kind() {
            Some(Fn) => self.parse_function(state),
            _ => {
                let builder = GreenBuilder::new();
                if let Some(token) = state.advance() {
                    builder
                        .token(token.kind, token.span.end - token.span.start)
                        .finish(RustSyntaxKind::Error)
                } else {
                    builder.finish(RustSyntaxKind::Error)
                }
            }
        }
    }

    /// 解析函数
    fn parse_function(&self, state: &mut ParserState<'_, RustSyntaxKind>) -> Rc<GreenNode<RustSyntaxKind>> {
        let mut builder = GreenBuilder::new();
        if let Some(token) = state.advance() {
            builder = builder.token(token.kind, token.span.end - token.span.start); // fn
        }
        if let Some(token) = state.advance() {
            builder = builder.token(token.kind, token.span.end - token.span.start); // name
        }
        let params = self.parse_parameter_list(state);
        builder = builder.push(GreenTree::Node(params));
        let body = self.parse_block_expression(state);
        builder = builder.push(GreenTree::Node(body));
        builder.finish(RustSyntaxKind::Function)
    }

    /// 解析参数列表
    fn parse_parameter_list(&self, state: &mut ParserState<'_, RustSyntaxKind>) -> Rc<GreenNode<RustSyntaxKind>> {
        let mut builder = GreenBuilder::new();
        if let Some(token) = state.advance() {
            builder = builder.token(token.kind, token.span.end - token.span.start); // '('
        }
        while !state.is_at_end() && state.peek_kind() != Some(RustSyntaxKind::RightParen) {
            let param = self.parse_parameter(state);
            builder = builder.push(GreenTree::Node(param));
            if state.peek_kind() == Some(RustSyntaxKind::Comma) {
                if let Some(token) = state.advance() {
                    builder = builder.token(token.kind, token.span.end - token.span.start);
                }
            }
        }
        if let Some(token) = state.advance() {
            builder = builder.token(token.kind, token.span.end - token.span.start); // ')'
        }
        builder.finish(RustSyntaxKind::ParameterList)
    }

    /// 解析单个参数
    fn parse_parameter(&self, state: &mut ParserState<'_, RustSyntaxKind>) -> Rc<GreenNode<RustSyntaxKind>> {
        let mut builder = GreenBuilder::new();
        if let Some(token) = state.advance() {
            builder = builder.token(token.kind, token.span.end - token.span.start); // ident
        }
        if let Some(token) = state.advance() {
            builder = builder.token(token.kind, token.span.end - token.span.start); // ':'
        }
        if let Some(token) = state.advance() {
            builder = builder.token(token.kind, token.span.end - token.span.start); // type ident
        }
        builder.finish(RustSyntaxKind::Parameter)
    }

    /// 解析块表达式
    fn parse_block_expression(&self, state: &mut ParserState<'_, RustSyntaxKind>) -> Rc<GreenNode<RustSyntaxKind>> {
        let mut builder = GreenBuilder::new();
        if let Some(token) = state.advance() {
            builder = builder.token(token.kind, token.span.end - token.span.start); // '{'
        }
        while state.peek_kind() != Some(RustSyntaxKind::RightBrace) && !state.is_at_end() {
            let stmt = self.parse_statement(state);
            builder = builder.push(GreenTree::Node(stmt));
        }
        if let Some(token) = state.advance() {
            builder = builder.token(token.kind, token.span.end - token.span.start); // '}'
        }
        builder.finish(RustSyntaxKind::BlockExpression)
    }

    /// 解析语句
    fn parse_statement(&self, state: &mut ParserState<'_, RustSyntaxKind>) -> Rc<GreenNode<RustSyntaxKind>> {
        use RustSyntaxKind::*;
        match state.peek_kind() {
            Some(Let) => self.parse_let_statement(state),
            _ => {
                let mut builder = GreenBuilder::new();
                let expr = self.parse_expression(state);
                builder = builder.push(GreenTree::Node(expr));
                if state.peek_kind() == Some(Semicolon) {
                    if let Some(token) = state.advance() {
                        builder = builder.token(token.kind, token.span.end - token.span.start);
                    }
                }
                builder.finish(ExpressionStatement)
            }
        }
    }

    /// 解析 let 语句
    fn parse_let_statement(&self, state: &mut ParserState<'_, RustSyntaxKind>) -> Rc<GreenNode<RustSyntaxKind>> {
        let mut builder = GreenBuilder::new();
        if let Some(token) = state.advance() {
            builder = builder.token(token.kind, token.span.end - token.span.start); // let
        }
        if let Some(token) = state.advance() {
            builder = builder.token(token.kind, token.span.end - token.span.start); // ident
        }
        if state.peek_kind() == Some(RustSyntaxKind::Colon) {
            if let Some(token) = state.advance() {
                builder = builder.token(token.kind, token.span.end - token.span.start);
            }
            if let Some(token) = state.advance() {
                builder = builder.token(token.kind, token.span.end - token.span.start); // type ident
            }
        }
        if let Some(token) = state.advance() {
            builder = builder.token(token.kind, token.span.end - token.span.start); // '='
        }
        let expr = self.parse_expression(state);
        builder = builder.push(GreenTree::Node(expr));
        if let Some(token) = state.advance() {
            builder = builder.token(token.kind, token.span.end - token.span.start); // ';'
        }
        builder.finish(RustSyntaxKind::LetStatement)
    }

    /// 解析表达式（占位：当前仅主表达式）
    fn parse_expression(&self, state: &mut ParserState<'_, RustSyntaxKind>) -> Rc<GreenNode<RustSyntaxKind>> {
        self.parse_primary_expression(state)
    }

    /// 解析主表达式
    fn parse_primary_expression(&self, state: &mut ParserState<'_, RustSyntaxKind>) -> Rc<GreenNode<RustSyntaxKind>> {
        use RustSyntaxKind::*;
        match state.peek_kind() {
            Some(Identifier) => self.parse_identifier_expression(state),
            Some(IntegerLiteral | FloatLiteral | StringLiteral | CharLiteral) => self.parse_literal_expression(state),
            Some(True | False) => self.parse_boolean_literal(state),
            Some(LeftParen) => self.parse_parenthesized_expression(state),
            _ => {
                GreenBuilder::new().finish(RustSyntaxKind::Error)
            }
        }
    }

    /// 解析标识符表达式
    fn parse_identifier_expression(&self, state: &mut ParserState<'_, RustSyntaxKind>) -> Rc<GreenNode<RustSyntaxKind>> {
        let mut builder = GreenBuilder::new();
        if let Some(token) = state.advance() {
            builder = builder.token(token.kind, token.span.end - token.span.start);
        }
        builder.finish(RustSyntaxKind::IdentifierExpression)
    }

    /// 解析字面量表达式
    fn parse_literal_expression(&self, state: &mut ParserState<'_, RustSyntaxKind>) -> Rc<GreenNode<RustSyntaxKind>> {
        let mut builder = GreenBuilder::new();
        if let Some(token) = state.advance() {
            builder = builder.token(token.kind, token.span.end - token.span.start);
        }
        builder.finish(RustSyntaxKind::LiteralExpression)
    }

    /// 解析布尔字面量
    fn parse_boolean_literal(&self, state: &mut ParserState<'_, RustSyntaxKind>) -> Rc<GreenNode<RustSyntaxKind>> {
        let mut builder = GreenBuilder::new();
        if let Some(token) = state.advance() {
            builder = builder.token(token.kind, token.span.end - token.span.start);
        }
        builder.finish(RustSyntaxKind::BooleanLiteral)
    }

    /// 解析括号表达式
    fn parse_parenthesized_expression(&self, state: &mut ParserState<'_, RustSyntaxKind>) -> Rc<GreenNode<RustSyntaxKind>> {
        let mut builder = GreenBuilder::new();
        if let Some(token) = state.advance() {
            builder = builder.token(token.kind, token.span.end - token.span.start); // '('
        }
        let expr = self.parse_expression(state);
        builder = builder.push(GreenTree::Node(expr));
        if let Some(token) = state.advance() {
            builder = builder.token(token.kind, token.span.end - token.span.start); // ')'
        }
        builder.finish(RustSyntaxKind::ParenthesizedExpression)
    }
}
