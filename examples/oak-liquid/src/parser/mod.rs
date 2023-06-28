/// Liquid Parser module
///
/// This module defines the parser for Liquid templates, responsible for parsing the tokens into an AST.
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

/// Element type definitions for Liquid parser.
pub mod element_type;
use crate::{
    language::LiquidLanguage,
    lexer::{LiquidLexer, token_type::LiquidTokenType},
};
use element_type::LiquidElementType;

pub(crate) type State<'a, S> = ParserState<'a, LiquidLanguage, S>;

/// Parser for Liquid templates
#[derive(Debug, Clone)]
pub struct LiquidParser<'config> {
    /// Language configuration
    config: &'config LiquidLanguage,
}

impl<'config> LiquidParser<'config> {
    /// Create a new Liquid parser
    pub fn new(config: &'config LiquidLanguage) -> Self {
        Self { config }
    }

    fn parse_node<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        match state.peek_kind() {
            Some(LiquidTokenType::DoubleLeftBrace) => self.parse_variable(state),
            Some(LiquidTokenType::LeftBracePercent) => self.parse_tag_statement(state),
            Some(LiquidTokenType::Comment) => {
                let cp = state.checkpoint();
                state.bump();
                state.finish_at(cp, LiquidElementType::Comment);
                Ok(())
            }
            _ => {
                let cp = state.checkpoint();
                state.advance();
                state.finish_at(cp, LiquidElementType::Text);
                Ok(())
            }
        }
    }

    fn parse_variable<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        let cp = state.checkpoint();
        state.expect(LiquidTokenType::DoubleLeftBrace)?;
        self.parse_expression(state)?;
        state.expect(LiquidTokenType::DoubleRightBrace)?;
        state.finish_at(cp, LiquidElementType::Variable);
        Ok(())
    }

    fn parse_tag_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        let cp = state.checkpoint();
        state.expect(LiquidTokenType::LeftBracePercent)?;

        let kind = state.peek_kind();
        match kind {
            Some(LiquidTokenType::Identifier) => {
                let text = state.peek_text().unwrap_or_default();
                match text.as_ref() {
                    "if" => self.parse_if_statement(state, cp),
                    "for" => self.parse_for_statement(state, cp),
                    "block" => self.parse_block_statement(state, cp),
                    "macro" => self.parse_macro_definition(state, cp),
                    _ => {
                        while state.not_at_end() && !state.at(LiquidTokenType::PercentRightBrace) {
                            state.advance();
                        }
                        state.expect(LiquidTokenType::PercentRightBrace)?;
                        state.finish_at(cp, LiquidElementType::Tag);
                        Ok(())
                    }
                }
            }
            _ => {
                while state.not_at_end() && !state.at(LiquidTokenType::PercentRightBrace) {
                    state.advance();
                }
                state.expect(LiquidTokenType::PercentRightBrace)?;
                state.finish_at(cp, LiquidElementType::Tag);
                Ok(())
            }
        }
    }

    fn parse_if_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) -> Result<(), oak_core::OakError> {
        state.expect(LiquidTokenType::Identifier)?; // if
        self.parse_expression(state)?;
        state.expect(LiquidTokenType::PercentRightBrace)?;

        while state.not_at_end() {
            if state.at(LiquidTokenType::LeftBracePercent) {
                if let Some(LiquidTokenType::Identifier) = state.peek_kind_at(1) {
                    let text = state.tokens.peek_at(1).map(|t| state.source.get_text_in(t.span)).unwrap_or_default();
                    if text == "endif" || text == "elif" || text == "else" {
                        break;
                    }
                }
            }
            self.parse_node(state)?;
        }

        if state.at(LiquidTokenType::LeftBracePercent) {
            let text = state.tokens.peek_at(1).map(|t| state.source.get_text_in(t.span)).unwrap_or_default();
            if text == "elif" {
                state.expect(LiquidTokenType::LeftBracePercent)?;
                self.parse_if_statement(state, state.checkpoint())?;
            }
            else if text == "else" {
                state.expect(LiquidTokenType::LeftBracePercent)?;
                state.expect(LiquidTokenType::Identifier)?; // else
                state.expect(LiquidTokenType::PercentRightBrace)?;
                while state.not_at_end() && !(state.at(LiquidTokenType::LeftBracePercent) && state.tokens.peek_at(1).map(|t| state.source.get_text_in(t.span)).map(|t| t == "endif").unwrap_or(false)) {
                    self.parse_node(state)?;
                }
            }
        }

        if state.at(LiquidTokenType::LeftBracePercent) && state.tokens.peek_at(1).map(|t| state.source.get_text_in(t.span)).map(|t| t == "endif").unwrap_or(false) {
            state.expect(LiquidTokenType::LeftBracePercent)?;
            state.expect(LiquidTokenType::Identifier)?; // endif
            state.expect(LiquidTokenType::PercentRightBrace)?;
        }

        state.finish_at(cp, LiquidElementType::IfStatement);
        Ok(())
    }

    fn parse_for_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) -> Result<(), oak_core::OakError> {
        state.expect(LiquidTokenType::Identifier)?; // for
        self.parse_expression(state)?; // x
        // Liquid for syntax: for x in y
        if state.peek_text().map(|t| t == "in").unwrap_or(false) {
            state.advance();
            self.parse_expression(state)?; // y
        }
        state.expect(LiquidTokenType::PercentRightBrace)?;

        while state.not_at_end() && !(state.at(LiquidTokenType::LeftBracePercent) && state.tokens.peek_at(1).map(|t| state.source.get_text_in(t.span)).map(|t| t == "endfor").unwrap_or(false)) {
            self.parse_node(state)?;
        }

        state.expect(LiquidTokenType::LeftBracePercent)?;
        state.expect(LiquidTokenType::Identifier)?; // endfor
        state.expect(LiquidTokenType::PercentRightBrace)?;

        state.finish_at(cp, LiquidElementType::ForStatement);
        Ok(())
    }

    fn parse_block_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) -> Result<(), oak_core::OakError> {
        state.expect(LiquidTokenType::Identifier)?; // block
        state.expect(LiquidTokenType::Identifier)?; // name
        state.expect(LiquidTokenType::PercentRightBrace)?;

        while state.not_at_end() && !(state.at(LiquidTokenType::LeftBracePercent) && state.tokens.peek_at(1).map(|t| state.source.get_text_in(t.span)).map(|t| t == "endblock").unwrap_or(false)) {
            self.parse_node(state)?;
        }

        state.expect(LiquidTokenType::LeftBracePercent)?;
        state.expect(LiquidTokenType::Identifier)?; // endblock
        if state.at(LiquidTokenType::Identifier) {
            state.advance();
        }
        state.expect(LiquidTokenType::PercentRightBrace)?;

        state.finish_at(cp, LiquidElementType::Block);
        Ok(())
    }

    fn parse_macro_definition<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) -> Result<(), oak_core::OakError> {
        state.expect(LiquidTokenType::Identifier)?; // macro
        state.expect(LiquidTokenType::Identifier)?; // name

        if state.at(LiquidTokenType::LeftParen) {
            state.advance();
            while state.not_at_end() && !state.at(LiquidTokenType::RightParen) {
                if state.at(LiquidTokenType::Identifier) {
                    state.advance();
                }
                if state.at(LiquidTokenType::Comma) {
                    state.advance();
                }
            }
            state.expect(LiquidTokenType::RightParen)?;
        }

        state.expect(LiquidTokenType::PercentRightBrace)?;

        while state.not_at_end() && !(state.at(LiquidTokenType::LeftBracePercent) && state.tokens.peek_at(1).map(|t| state.source.get_text_in(t.span)).map(|t| t == "endmacro").unwrap_or(false)) {
            self.parse_node(state)?;
        }

        state.expect(LiquidTokenType::LeftBracePercent)?;
        state.expect(LiquidTokenType::Identifier)?; // endmacro
        state.expect(LiquidTokenType::PercentRightBrace)?;

        state.finish_at(cp, LiquidElementType::MacroDefinition);
        Ok(())
    }

    fn parse_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        self.parse_binary_expression(state, 0)
    }

    fn parse_binary_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, min_precedence: i32) -> Result<(), oak_core::OakError> {
        let cp = state.checkpoint();
        self.parse_primary_expression(state)?;

        while let Some(kind) = state.peek_kind() {
            let precedence = self.get_precedence(kind);
            if precedence < min_precedence {
                break;
            }

            state.advance();
            self.parse_binary_expression(state, precedence + 1)?;
            state.finish_at(cp, if kind == LiquidTokenType::Pipe { LiquidElementType::Filter } else { LiquidElementType::Expression });
        }

        Ok(())
    }

    fn get_precedence(&self, kind: LiquidTokenType) -> i32 {
        match kind {
            LiquidTokenType::Pipe => 1,
            LiquidTokenType::Plus | LiquidTokenType::Minus => 2,
            LiquidTokenType::Star | LiquidTokenType::Slash => 3,
            _ => -1,
        }
    }

    fn parse_primary_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        let cp = state.checkpoint();

        match state.peek_kind() {
            Some(LiquidTokenType::Identifier) => {
                state.advance();
                if state.at(LiquidTokenType::LeftParen) {
                    state.advance();
                    while state.not_at_end() && !state.at(LiquidTokenType::RightParen) {
                        self.parse_expression(state)?;
                        if state.at(LiquidTokenType::Comma) {
                            state.advance();
                        }
                    }
                    state.expect(LiquidTokenType::RightParen)?;
                    state.finish_at(cp, LiquidElementType::Function);
                }
                else {
                    state.finish_at(cp, LiquidElementType::Identifier);
                }
            }
            Some(LiquidTokenType::String) | Some(LiquidTokenType::Number) | Some(LiquidTokenType::Boolean) => {
                state.advance();
                state.finish_at(cp, LiquidElementType::Literal);
            }
            Some(LiquidTokenType::LeftParen) => {
                state.advance();
                self.parse_expression(state)?;
                state.expect(LiquidTokenType::RightParen)?;
            }
            _ => {
                while state.not_at_end() && !state.at(LiquidTokenType::PercentRightBrace) && !state.at(LiquidTokenType::DoubleRightBrace) && !state.at(LiquidTokenType::RightParen) && !state.at(LiquidTokenType::Comma) {
                    state.advance();
                }
            }
        }
        Ok(())
    }
}

impl<'config> Parser<LiquidLanguage> for LiquidParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<LiquidLanguage>) -> ParseOutput<'a, LiquidLanguage> {
        let lexer = LiquidLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();
            while state.not_at_end() {
                self.parse_node(state)?;
            }
            Ok(state.finish_at(checkpoint, LiquidElementType::Root))
        })
    }
}
