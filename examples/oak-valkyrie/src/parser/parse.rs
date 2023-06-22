use crate::{kind::ValkyrieSyntaxKind::*, language::ValkyrieLanguage, lexer::ValkyrieKeywords};
use oak_core::{GreenNode, OakError, parser::ParserState, source::Source};

type State<'a, S> = ParserState<'a, ValkyrieLanguage, S>;

impl<'config> super::ValkyrieParser<'config> {
    pub(crate) fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        while let Some(t) = state.current() {
            match t.kind {
                Whitespace | Newline | LineComment | BlockComment => {
                    state.advance();
                }
                _ => break,
            }
        }
    }

    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, ValkyrieLanguage> {
        let cp = state.checkpoint();
        self.skip_trivia(state);

        while state.not_at_end() {
            let start_index = state.tokens.index();
            if self.parse_source_file(state).is_err() && state.tokens.index() == start_index {
                state.bump();
            }
            self.skip_trivia(state);
        }

        state.finish_at(cp, ValkyrieRoot)
    }

    fn parse_source_file<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ValkyrieLanguage>, OakError> {
        if let Some(t) = state.current() {
            match t.kind {
                Keyword(ValkyrieKeywords::Namespace) => self.parse_namespace(state),
                Keyword(ValkyrieKeywords::Micro) => self.parse_micro(state),
                Keyword(ValkyrieKeywords::Widget) => self.parse_widget(state),
                Keyword(ValkyrieKeywords::Class) => self.parse_class(state),
                Keyword(ValkyrieKeywords::Let) => self.parse_let_statement(state),
                _ => self.parse_expression_statement(state),
            }
        }
        else {
            Err(OakError::custom_error("Unexpected end of file"))
        }
    }

    fn parse_namespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ValkyrieLanguage>, OakError> {
        let cp = state.checkpoint();

        state.expect(Keyword(ValkyrieKeywords::Namespace))?;
        self.skip_trivia(state);

        state.expect(Identifier)?;
        self.skip_trivia(state);

        state.expect(LeftBrace)?;
        self.skip_trivia(state);

        while let Some(t) = state.current() {
            if t.kind == RightBrace {
                break;
            }
            self.parse_source_file(state)?;
            self.skip_trivia(state);

            if state.at(Comma) {
                state.bump();
                self.skip_trivia(state);
            }
        }

        state.expect(RightBrace)?;

        Ok(state.finish_at(cp, Namespace))
    }

    fn parse_micro<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ValkyrieLanguage>, OakError> {
        let cp = state.checkpoint();

        state.expect(Keyword(ValkyrieKeywords::Micro))?;
        self.skip_trivia(state);

        state.expect(Identifier)?;
        self.skip_trivia(state);

        if state.at(LeftParen) {
            self.parse_parameter_list(state)?;
        }

        self.skip_trivia(state);

        if state.at(Arrow) {
            state.bump();
            self.skip_trivia(state);
            let cp_type = state.checkpoint();
            state.expect(Identifier)?;
            state.finish_at(cp_type, Type);
            self.skip_trivia(state);
        }

        if state.at(LeftBrace) {
            self.parse_block_expr_node(state)?;
        }

        Ok(state.finish_at(cp, Micro))
    }

    fn parse_mezzo<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ValkyrieLanguage>, OakError> {
        let cp = state.checkpoint();

        state.expect(Keyword(ValkyrieKeywords::Mezzo))?;
        self.skip_trivia(state);

        state.expect(Identifier)?;
        self.skip_trivia(state);

        if state.at(LeftParen) {
            self.parse_parameter_list(state)?;
        }

        self.skip_trivia(state);

        if state.at(LeftBrace) {
            self.parse_block_expr_node(state)?;
        }

        Ok(state.finish_at(cp, Mezzo))
    }

    fn parse_widget<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ValkyrieLanguage>, OakError> {
        let cp = state.checkpoint();

        state.expect(Keyword(ValkyrieKeywords::Widget))?;
        self.skip_trivia(state);

        state.expect(Identifier)?;
        self.skip_trivia(state);

        if state.at(LeftBrace) {
            self.parse_block_expr_node(state)?;
        }

        Ok(state.finish_at(cp, Widget))
    }

    fn parse_class<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ValkyrieLanguage>, OakError> {
        let cp = state.checkpoint();

        state.expect(Keyword(ValkyrieKeywords::Class))?;
        self.skip_trivia(state);

        state.expect(Identifier)?;
        self.skip_trivia(state);

        if state.at(LeftBrace) {
            self.parse_block_expr_node(state)?;
        }

        Ok(state.finish_at(cp, Class))
    }

    fn parse_parameter_list<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ValkyrieLanguage>, OakError> {
        let cp = state.checkpoint();

        state.expect(LeftParen)?;
        self.skip_trivia(state);

        while let Some(t) = state.current() {
            if t.kind == RightParen {
                break;
            }

            if t.kind == Identifier {
                self.parse_parameter(state)?;
                self.skip_trivia(state);

                if state.at(Comma) {
                    state.bump();
                    self.skip_trivia(state);
                }
            }
            else {
                state.bump();
                self.skip_trivia(state);
            }
        }

        state.expect(RightParen)?;

        Ok(state.finish_at(cp, ParameterList))
    }

    fn parse_parameter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ValkyrieLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(Identifier)?;
        self.skip_trivia(state);
        if state.at(Colon) {
            state.bump();
            self.skip_trivia(state);
            state.expect(Identifier)?;
            self.skip_trivia(state);
        }
        Ok(state.finish_at(cp, Parameter))
    }

    fn parse_let_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ValkyrieLanguage>, OakError> {
        let cp = state.checkpoint();

        state.expect(Keyword(ValkyrieKeywords::Let))?;
        self.skip_trivia(state);

        if state.at(Keyword(ValkyrieKeywords::Mut)) {
            state.bump();
            self.skip_trivia(state);
        }

        state.expect(Identifier)?;
        self.skip_trivia(state);

        state.expect(Eq)?;
        self.skip_trivia(state);

        let expr = self.parse_expression_internal(state, 0);
        state.push_child(expr);
        self.skip_trivia(state);

        state.expect(Semicolon)?;

        Ok(state.finish_at(cp, LetStatement))
    }

    fn parse_expression_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ValkyrieLanguage>, OakError> {
        let cp = state.checkpoint();

        let expr = self.parse_expression_internal(state, 0);
        state.push_child(expr);
        self.skip_trivia(state);

        if state.at(Semicolon) {
            state.bump();
        }

        Ok(state.finish_at(cp, ExpressionStatement))
    }

    pub(crate) fn parse_expression_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, min_precedence: u8) -> &'a GreenNode<'a, ValkyrieLanguage> {
        oak_core::parser::PrattParser::parse(state, min_precedence, self)
    }

    pub(crate) fn parse_primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ValkyrieLanguage>, OakError> {
        let cp = state.checkpoint();
        let t = state.current().ok_or_else(|| OakError::custom_error("Unexpected end of file"))?;
        println!("parse_primary: token={:?}, text={:?}", t.kind, state.source.get_text_in(t.span.clone().into()));

        match t.kind {
            Identifier => {
                state.bump();
                Ok(state.finish_at(cp, IdentifierExpression))
            }
            IntegerLiteral | FloatLiteral | StringLiteral | CharLiteral | BoolLiteral => {
                state.bump();
                Ok(state.finish_at(cp, LiteralExpression))
            }
            LeftParen => {
                state.bump();
                let expr = self.parse_expression_internal(state, 0);
                state.push_child(expr);
                state.expect(RightParen)?;
                Ok(state.finish_at(cp, ParenthesizedExpression))
            }
            LeftBrace => self.parse_block_expr_node(state),
            Keyword(ValkyrieKeywords::Class) => self.parse_anonymous_class(state),
            Keyword(ValkyrieKeywords::If) => self.parse_if(state),
            Keyword(ValkyrieKeywords::Match) => self.parse_match(state),
            Keyword(ValkyrieKeywords::While) | Keyword(ValkyrieKeywords::For) => self.parse_loop(state),
            Keyword(ValkyrieKeywords::Return) => self.parse_return(state),
            Keyword(ValkyrieKeywords::Break) => self.parse_break(state),
            Keyword(ValkyrieKeywords::Continue) => self.parse_continue(state),
            Keyword(ValkyrieKeywords::Yield) => self.parse_yield(state),
            Keyword(ValkyrieKeywords::Raise) => self.parse_raise(state),
            Keyword(ValkyrieKeywords::Try) => self.parse_catch(state),
            _ => Err(OakError::custom_error(format!("Unexpected token: {:?}", t.kind))),
        }
    }

    fn parse_if<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ValkyrieLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(Keyword(ValkyrieKeywords::If))?;
        self.skip_trivia(state);
        let cond = self.parse_expression_internal(state, 0);
        state.push_child(cond);
        self.skip_trivia(state);
        self.parse_block_expr_node(state)?;
        self.skip_trivia(state);
        if state.at(Keyword(ValkyrieKeywords::Else)) {
            state.bump();
            self.skip_trivia(state);
            if state.at(Keyword(ValkyrieKeywords::If)) {
                self.parse_if(state)?;
            }
            else {
                self.parse_block_expr_node(state)?;
            }
        }
        Ok(state.finish_at(cp, IfExpression))
    }

    fn parse_match<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ValkyrieLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(Keyword(ValkyrieKeywords::Match))?;
        self.skip_trivia(state);
        let expr = self.parse_expression_internal(state, 0);
        state.push_child(expr);
        self.skip_trivia(state);
        state.expect(LeftBrace)?;
        self.skip_trivia(state);
        while state.not_at_end() && !state.at(RightBrace) {
            self.parse_match_arm(state)?;
            self.skip_trivia(state);
        }
        state.expect(RightBrace)?;
        Ok(state.finish_at(cp, MatchExpression))
    }

    fn parse_match_arm<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ValkyrieLanguage>, OakError> {
        let cp = state.checkpoint();
        self.parse_pattern(state)?;
        self.skip_trivia(state);
        if state.at(Keyword(ValkyrieKeywords::When)) {
            state.bump();
            self.skip_trivia(state);
            let cond = self.parse_expression_internal(state, 0);
            state.push_child(cond);
            self.skip_trivia(state);
        }
        state.expect(Arrow)?;
        self.skip_trivia(state);
        let expr = self.parse_expression_internal(state, 0);
        state.push_child(expr);
        Ok(state.finish_at(cp, MatchArm))
    }

    fn parse_pattern<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ValkyrieLanguage>, OakError> {
        let cp = state.checkpoint();
        if state.at(Identifier) {
            state.bump();
        }
        else if state.at(IntegerLiteral) || state.at(StringLiteral) {
            state.bump();
        }
        else if state.at(Underscore) {
            state.bump();
        }
        else {
            return Err(OakError::custom_error("Expected pattern"));
        }
        Ok(state.finish_at(cp, Identifier)) // Reuse Identifier kind for simplicity in pattern for now
    }

    fn parse_loop<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ValkyrieLanguage>, OakError> {
        let cp = state.checkpoint();
        state.bump(); // while or for
        self.skip_trivia(state);
        let expr = self.parse_expression_internal(state, 0);
        state.push_child(expr);
        self.skip_trivia(state);
        self.parse_block_expr_node(state)?;
        Ok(state.finish_at(cp, LoopExpression))
    }

    fn parse_return<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ValkyrieLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(Keyword(ValkyrieKeywords::Return))?;
        self.skip_trivia(state);
        if state.not_at_end() && !state.at(Semicolon) && !state.at(RightBrace) {
            let expr = self.parse_expression_internal(state, 0);
            state.push_child(expr);
        }
        Ok(state.finish_at(cp, ReturnExpression))
    }

    fn parse_break<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ValkyrieLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(Keyword(ValkyrieKeywords::Break))?;
        self.skip_trivia(state);
        if state.at(At) {
            state.bump();
            state.expect(Identifier)?;
            self.skip_trivia(state);
        }
        if state.not_at_end() && !state.at(Semicolon) && !state.at(RightBrace) {
            let expr = self.parse_expression_internal(state, 0);
            state.push_child(expr);
        }
        Ok(state.finish_at(cp, BreakExpression))
    }

    fn parse_continue<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ValkyrieLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(Keyword(ValkyrieKeywords::Continue))?;
        self.skip_trivia(state);
        if state.at(At) {
            state.bump();
            state.expect(Identifier)?;
        }
        Ok(state.finish_at(cp, ContinueExpression))
    }

    fn parse_yield<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ValkyrieLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(Keyword(ValkyrieKeywords::Yield))?;
        self.skip_trivia(state);
        if state.at(Star) {
            state.bump();
            self.skip_trivia(state);
        }
        if state.not_at_end() && !state.at(Semicolon) && !state.at(RightBrace) {
            let expr = self.parse_expression_internal(state, 0);
            state.push_child(expr);
        }
        Ok(state.finish_at(cp, YieldExpression))
    }

    fn parse_raise<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ValkyrieLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(Keyword(ValkyrieKeywords::Raise))?;
        self.skip_trivia(state);
        let expr = self.parse_expression_internal(state, 0);
        state.push_child(expr);
        Ok(state.finish_at(cp, RaiseExpression))
    }

    fn parse_catch<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ValkyrieLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(Keyword(ValkyrieKeywords::Try))?;
        self.skip_trivia(state);
        let expr = self.parse_expression_internal(state, 0);
        state.push_child(expr);
        self.skip_trivia(state);
        while state.at(Keyword(ValkyrieKeywords::Catch)) {
            state.bump();
            self.skip_trivia(state);
            state.expect(LeftBrace)?;
            self.skip_trivia(state);
            while state.not_at_end() && !state.at(RightBrace) {
                self.parse_match_arm(state)?;
                self.skip_trivia(state);
            }
            state.expect(RightBrace)?;
            self.skip_trivia(state);
        }
        Ok(state.finish_at(cp, CatchExpression))
    }

    fn parse_anonymous_class<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ValkyrieLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(Keyword(ValkyrieKeywords::Class))?;
        self.skip_trivia(state);

        if state.at(LeftParen) {
            state.bump();
            self.skip_trivia(state);
            while state.not_at_end() && !state.at(RightParen) {
                state.expect(Identifier)?;
                self.skip_trivia(state);
                if state.at(Comma) {
                    state.bump();
                    self.skip_trivia(state);
                }
            }
            state.expect(RightParen)?;
            self.skip_trivia(state);
        }

        self.parse_block_expr_node(state)?;

        Ok(state.finish_at(cp, AnonymousClass))
    }

    pub(crate) fn parse_block_expr_node<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ValkyrieLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(LeftBrace)?;
        self.skip_trivia(state);

        while let Some(t) = state.current() {
            if t.kind == RightBrace {
                break;
            }
            let start_index = state.tokens.index();
            if let Err(_) = self.parse_statement(state) {
                if state.tokens.index() == start_index {
                    state.bump();
                }
            }
            self.skip_trivia(state);

            if state.at(Comma) {
                state.bump();
                self.skip_trivia(state);
            }
        }
        state.expect(RightBrace)?;
        Ok(state.finish_at(cp, BlockExpression))
    }

    fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ValkyrieLanguage>, OakError> {
        if state.at(Keyword(ValkyrieKeywords::Let)) { self.parse_let_statement(state) } else { self.parse_expression_statement(state) }
    }
}
