use crate::{kind::ValkyrieSyntaxKind::*, language::ValkyrieLanguage, lexer::ValkyrieKeywords};
use oak_core::{GreenNode, OakError, parser::ParserState, source::Source};

type State<'a, S> = ParserState<'a, ValkyrieLanguage, S>;

impl<'config> super::ValkyrieParser<'config> {
    fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        while let Some(t) = state.current() {
            match t.kind {
                Whitespace | Newline | LineComment | BlockComment => {
                    state.advance();
                }
                _ => break,
            }
        }
    }

    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ValkyrieLanguage>, OakError> {
        let cp = state.checkpoint();
        self.skip_trivia(state);

        while state.not_at_end() {
            self.parse_source_file(state)?;
            self.skip_trivia(state);
        }

        Ok(state.finish_at(cp, ValkyrieRoot))
    }

    fn parse_source_file<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ValkyrieLanguage>, OakError> {
        if let Some(t) = state.current() {
            match t.kind {
                Keyword(ValkyrieKeywords::Namespace) => self.parse_namespace(state),
                Keyword(ValkyrieKeywords::Micro) => self.parse_micro(state),
                Keyword(ValkyrieKeywords::Fn) => self.parse_function(state),
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

        if state.at(LeftBrace) {
            self.parse_block_expr_node(state)?;
        }

        Ok(state.finish_at(cp, Micro))
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
            }
        }

        state.expect(RightParen)?;

        Ok(state.finish_at(cp, ParameterList))
    }

    fn parse_parameter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ValkyrieLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(Identifier)?;
        Ok(state.finish_at(cp, Parameter))
    }

    fn parse_let_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ValkyrieLanguage>, OakError> {
        let cp = state.checkpoint();

        state.expect(Keyword(ValkyrieKeywords::Let))?;
        self.skip_trivia(state);

        state.expect(Identifier)?;
        self.skip_trivia(state);

        state.expect(Eq)?;
        self.skip_trivia(state);

        self.parse_expression_internal(state, 0);
        self.skip_trivia(state);

        state.expect(Semicolon)?;

        Ok(state.finish_at(cp, LetStatement))
    }

    fn parse_expression_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ValkyrieLanguage>, OakError> {
        let cp = state.checkpoint();

        self.parse_expression_internal(state, 0);
        self.skip_trivia(state);

        if state.at(Semicolon) {
            state.bump();
        }

        Ok(state.finish_at(cp, ExpressionStatement))
    }

    fn parse_function<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ValkyrieLanguage>, OakError> {
        let cp = state.checkpoint();

        state.expect(Keyword(ValkyrieKeywords::Fn))?;
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

        Ok(state.finish_at(cp, Function))
    }

    pub(crate) fn parse_expression_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, min_precedence: u8) -> &'a GreenNode<'a, ValkyrieLanguage> {
        oak_core::parser::PrattParser::parse(state, min_precedence, self)
    }

    pub(crate) fn parse_primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ValkyrieLanguage>, OakError> {
        let cp = state.checkpoint();
        let t = state.current().ok_or_else(|| OakError::custom_error("Unexpected end of file"))?;

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
                self.parse_expression_internal(state, 0);
                state.expect(RightParen)?;
                Ok(state.finish_at(cp, ParenthesizedExpression))
            }
            LeftBrace => self.parse_block_expr_node(state),
            _ => Err(OakError::custom_error(format!("Unexpected token: {:?}", t.kind))),
        }
    }

    fn parse_block_expr_node<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ValkyrieLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(LeftBrace)?;
        self.skip_trivia(state);

        while let Some(t) = state.current() {
            if t.kind == RightBrace {
                break;
            }
            self.parse_source_file(state)?;
            self.skip_trivia(state);
        }

        state.expect(RightBrace)?;
        Ok(state.finish_at(cp, BlockExpression))
    }
}
