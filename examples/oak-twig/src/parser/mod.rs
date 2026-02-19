pub mod element_type;

use crate::{
    language::TwigLanguage,
    lexer::{TwigLexer, token_type::TwigTokenType},
    parser::element_type::TwigElementType,
};
use oak_core::{
    OakError,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, TwigLanguage, S>;

/// Parser for Twig templates.
pub struct TwigParser<'config> {
    pub(crate) config: &'config TwigLanguage,
}

impl<'config> TwigParser<'config> {
    /// Creates a new TwigParser with the given language configuration.
    pub fn new(config: &'config TwigLanguage) -> Self {
        Self { config }
    }

    fn parse_node<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        match state.peek_kind() {
            Some(TwigTokenType::DoubleLeftBrace) => self.parse_variable(state),
            Some(TwigTokenType::LeftBracePercent) => self.parse_tag_statement(state),
            Some(TwigTokenType::Comment) => {
                let cp = state.checkpoint();
                state.bump();
                state.finish_at(cp, TwigElementType::Comment);
                Ok(())
            }
            _ => {
                let cp = state.checkpoint();
                state.advance();
                state.finish_at(cp, TwigElementType::Text);
                Ok(())
            }
        }
    }

    fn parse_variable<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(TwigTokenType::DoubleLeftBrace)?;
        self.parse_expression(state)?;
        state.expect(TwigTokenType::DoubleRightBrace)?;
        state.finish_at(cp, TwigElementType::Variable);
        Ok(())
    }

    fn parse_tag_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(TwigTokenType::LeftBracePercent)?;

        let kind = state.peek_kind();
        match kind {
            Some(TwigTokenType::Identifier) => {
                let text = state.peek_text().unwrap_or_default();
                match text.as_ref() {
                    "if" => self.parse_if_statement(state, cp),
                    "for" => self.parse_for_statement(state, cp),
                    "block" => self.parse_block_statement(state, cp),
                    "macro" => self.parse_macro_definition(state, cp),
                    _ => {
                        while state.not_at_end() && !state.at(TwigTokenType::PercentRightBrace) {
                            state.advance();
                        }
                        state.expect(TwigTokenType::PercentRightBrace)?;
                        state.finish_at(cp, TwigElementType::Tag);
                        Ok(())
                    }
                }
            }
            _ => {
                while state.not_at_end() && !state.at(TwigTokenType::PercentRightBrace) {
                    state.advance();
                }
                state.expect(TwigTokenType::PercentRightBrace)?;
                state.finish_at(cp, TwigElementType::Tag);
                Ok(())
            }
        }
    }

    fn parse_if_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) -> Result<(), OakError> {
        state.expect(TwigTokenType::Identifier)?; // if
        self.parse_expression(state)?;
        state.expect(TwigTokenType::PercentRightBrace)?;

        while state.not_at_end() {
            if state.at(TwigTokenType::LeftBracePercent) {
                if let Some(TwigTokenType::Identifier) = state.peek_kind_at(1) {
                    let text = state.tokens.peek_at(1).map(|t| state.source.get_text_in(t.span)).unwrap_or_default();
                    if text == "endif" || text == "elif" || text == "else" {
                        break;
                    }
                }
            }
            self.parse_node(state)?;
        }

        if state.at(TwigTokenType::LeftBracePercent) {
            let text = state.tokens.peek_at(1).map(|t| state.source.get_text_in(t.span)).unwrap_or_default();
            if text == "elif" {
                state.expect(TwigTokenType::LeftBracePercent)?;
                self.parse_if_statement(state, state.checkpoint())?;
            }
            else if text == "else" {
                state.expect(TwigTokenType::LeftBracePercent)?;
                state.expect(TwigTokenType::Identifier)?; // else
                state.expect(TwigTokenType::PercentRightBrace)?;
                while state.not_at_end() && !(state.at(TwigTokenType::LeftBracePercent) && state.tokens.peek_at(1).map(|t| state.source.get_text_in(t.span)).map(|t| t == "endif").unwrap_or(false)) {
                    self.parse_node(state)?;
                }
            }
        }

        if state.at(TwigTokenType::LeftBracePercent) && state.tokens.peek_at(1).map(|t| state.source.get_text_in(t.span)).map(|t| t == "endif").unwrap_or(false) {
            state.expect(TwigTokenType::LeftBracePercent)?;
            state.expect(TwigTokenType::Identifier)?; // endif
            state.expect(TwigTokenType::PercentRightBrace)?;
        }

        state.finish_at(cp, TwigElementType::IfStatement);
        Ok(())
    }

    fn parse_for_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) -> Result<(), OakError> {
        state.expect(TwigTokenType::Identifier)?; // for
        self.parse_expression(state)?; // x
        // Twig for syntax: for x in y
        if state.peek_text().map(|t| t == "in").unwrap_or(false) {
            state.advance();
            self.parse_expression(state)?; // y
        }
        state.expect(TwigTokenType::PercentRightBrace)?;

        while state.not_at_end() && !(state.at(TwigTokenType::LeftBracePercent) && state.tokens.peek_at(1).map(|t| state.source.get_text_in(t.span)).map(|t| t == "endfor").unwrap_or(false)) {
            self.parse_node(state)?;
        }

        state.expect(TwigTokenType::LeftBracePercent)?;
        state.expect(TwigTokenType::Identifier)?; // endfor
        state.expect(TwigTokenType::PercentRightBrace)?;

        state.finish_at(cp, TwigElementType::ForStatement);
        Ok(())
    }

    fn parse_block_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) -> Result<(), OakError> {
        state.expect(TwigTokenType::Identifier)?; // block
        state.expect(TwigTokenType::Identifier)?; // name
        state.expect(TwigTokenType::PercentRightBrace)?;

        while state.not_at_end() && !(state.at(TwigTokenType::LeftBracePercent) && state.tokens.peek_at(1).map(|t| state.source.get_text_in(t.span)).map(|t| t == "endblock").unwrap_or(false)) {
            self.parse_node(state)?;
        }

        state.expect(TwigTokenType::LeftBracePercent)?;
        state.expect(TwigTokenType::Identifier)?; // endblock
        if state.at(TwigTokenType::Identifier) {
            state.advance();
        }
        state.expect(TwigTokenType::PercentRightBrace)?;

        state.finish_at(cp, TwigElementType::Block);
        Ok(())
    }

    fn parse_macro_definition<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) -> Result<(), OakError> {
        state.expect(TwigTokenType::Identifier)?; // macro
        state.expect(TwigTokenType::Identifier)?; // name

        if state.at(TwigTokenType::LeftParen) {
            state.advance();
            while state.not_at_end() && !state.at(TwigTokenType::RightParen) {
                if state.at(TwigTokenType::Identifier) {
                    state.advance();
                }
                if state.at(TwigTokenType::Comma) {
                    state.advance();
                }
            }
            state.expect(TwigTokenType::RightParen)?;
        }

        state.expect(TwigTokenType::PercentRightBrace)?;

        while state.not_at_end() && !(state.at(TwigTokenType::LeftBracePercent) && state.tokens.peek_at(1).map(|t| state.source.get_text_in(t.span)).map(|t| t == "endmacro").unwrap_or(false)) {
            self.parse_node(state)?;
        }

        state.expect(TwigTokenType::LeftBracePercent)?;
        state.expect(TwigTokenType::Identifier)?; // endmacro
        state.expect(TwigTokenType::PercentRightBrace)?;

        state.finish_at(cp, TwigElementType::MacroDefinition);
        Ok(())
    }

    fn parse_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        self.parse_binary_expression(state, 0)
    }

    fn parse_binary_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, min_precedence: i32) -> Result<(), OakError> {
        let cp = state.checkpoint();
        self.parse_primary_expression(state)?;

        while let Some(kind) = state.peek_kind() {
            let precedence = self.get_precedence(kind);
            if precedence < min_precedence {
                break;
            }

            state.advance();
            self.parse_binary_expression(state, precedence + 1)?;
            state.finish_at(cp, if kind == TwigTokenType::Pipe { TwigElementType::Filter } else { TwigElementType::Expression });
        }

        Ok(())
    }

    fn get_precedence(&self, kind: TwigTokenType) -> i32 {
        match kind {
            TwigTokenType::Pipe => 1,
            TwigTokenType::Plus | TwigTokenType::Minus => 2,
            TwigTokenType::Star | TwigTokenType::Slash => 3,
            _ => -1,
        }
    }

    fn parse_primary_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();

        match state.peek_kind() {
            Some(TwigTokenType::Identifier) => {
                state.advance();
                if state.at(TwigTokenType::LeftParen) {
                    state.advance();
                    while state.not_at_end() && !state.at(TwigTokenType::RightParen) {
                        self.parse_expression(state)?;
                        if state.at(TwigTokenType::Comma) {
                            state.advance();
                        }
                    }
                    state.expect(TwigTokenType::RightParen)?;
                    state.finish_at(cp, TwigElementType::Function);
                }
                else {
                    state.finish_at(cp, TwigElementType::Identifier);
                }
            }
            Some(TwigTokenType::String) | Some(TwigTokenType::Number) | Some(TwigTokenType::Boolean) => {
                state.advance();
                state.finish_at(cp, TwigElementType::Literal);
            }
            Some(TwigTokenType::LeftParen) => {
                state.advance();
                self.parse_expression(state)?;
                state.expect(TwigTokenType::RightParen)?;
            }
            _ => {
                while state.not_at_end() && !state.at(TwigTokenType::PercentRightBrace) && !state.at(TwigTokenType::DoubleRightBrace) && !state.at(TwigTokenType::RightParen) && !state.at(TwigTokenType::Comma) {
                    state.advance();
                }
            }
        }
        Ok(())
    }
}

impl<'config> Parser<TwigLanguage> for TwigParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<TwigLanguage>) -> ParseOutput<'a, TwigLanguage> {
        let lexer = TwigLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();
            while state.not_at_end() {
                self.parse_node(state)?;
            }
            Ok(state.finish_at(checkpoint, TwigElementType::Root))
        })
    }
}
