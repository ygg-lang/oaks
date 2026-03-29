/// Jinja Parser module
///
/// This module defines the parser for Jinja templates, responsible for parsing the tokens into an AST.
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

/// Element type definitions for Jinja parser.
pub mod element_type;
use crate::{
    language::JinjaLanguage,
    lexer::{JinjaLexer, token_type::JinjaTokenType},
};
use element_type::JinjaElementType;

pub(crate) type State<'a, S> = ParserState<'a, JinjaLanguage, S>;

/// Parser for Jinja templates
#[derive(Debug, Clone)]
pub struct JinjaParser<'config> {
    /// Language configuration
    config: &'config JinjaLanguage,
}

impl<'config> JinjaParser<'config> {
    /// Create a new Jinja parser
    pub fn new(config: &'config JinjaLanguage) -> Self {
        Self { config }
    }

    fn parse_node<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        match state.peek_kind() {
            Some(JinjaTokenType::DoubleLeftBrace) => self.parse_variable(state),
            Some(JinjaTokenType::LeftBracePercent) => self.parse_tag_statement(state),
            Some(JinjaTokenType::Comment) => {
                let cp = state.checkpoint();
                state.bump();
                state.finish_at(cp, JinjaElementType::Comment);
                Ok(())
            }
            _ => {
                let cp = state.checkpoint();
                state.advance();
                state.finish_at(cp, JinjaElementType::Text);
                Ok(())
            }
        }
    }

    fn parse_variable<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        let cp = state.checkpoint();
        state.expect(JinjaTokenType::DoubleLeftBrace)?;
        self.parse_expression(state)?;
        state.expect(JinjaTokenType::DoubleRightBrace)?;
        state.finish_at(cp, JinjaElementType::Variable);
        Ok(())
    }

    fn parse_tag_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        let cp = state.checkpoint();
        state.expect(JinjaTokenType::LeftBracePercent)?;

        let kind = state.peek_kind();
        match kind {
            Some(JinjaTokenType::Identifier) => {
                let text = state.peek_text().unwrap_or_default();
                match text.as_ref() {
                    "if" => self.parse_if_statement(state, cp),
                    "for" => self.parse_for_statement(state, cp),
                    "block" => self.parse_block_statement(state, cp),
                    "macro" => self.parse_macro_definition(state, cp),
                    "extends" => self.parse_extends_statement(state, cp),
                    "include" => self.parse_include_statement(state, cp),
                    "set" => self.parse_set_statement(state, cp),
                    "from" => self.parse_from_import_statement(state, cp),
                    "import" => self.parse_import_statement(state, cp),
                    _ => {
                        while state.not_at_end() && !state.at(JinjaTokenType::PercentRightBrace) {
                            state.advance();
                        }
                        state.expect(JinjaTokenType::PercentRightBrace)?;
                        state.finish_at(cp, JinjaElementType::Tag);
                        Ok(())
                    }
                }
            }
            _ => {
                while state.not_at_end() && !state.at(JinjaTokenType::PercentRightBrace) {
                    state.advance();
                }
                state.expect(JinjaTokenType::PercentRightBrace)?;
                state.finish_at(cp, JinjaElementType::Tag);
                Ok(())
            }
        }
    }

    fn parse_if_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) -> Result<(), oak_core::OakError> {
        state.expect(JinjaTokenType::Identifier)?; // if
        self.parse_expression(state)?;
        state.expect(JinjaTokenType::PercentRightBrace)?;

        while state.not_at_end() {
            if state.at(JinjaTokenType::LeftBracePercent) {
                if let Some(JinjaTokenType::Identifier) = state.peek_non_trivia_kind_at(1) {
                    let text = state.peek_non_trivia_at(1).map(|t| state.source.get_text_in(t.span)).unwrap_or_default();
                    if text == "endif" || text == "elif" || text == "else" {
                        break;
                    }
                }
            }
            self.parse_node(state)?;
        }

        if state.at(JinjaTokenType::LeftBracePercent) {
            let text = state.peek_non_trivia_at(1).map(|t| state.source.get_text_in(t.span)).unwrap_or_default();
            if text == "elif" {
                state.expect(JinjaTokenType::LeftBracePercent)?;
                self.parse_if_statement(state, state.checkpoint())?;
            }
            else if text == "else" {
                state.expect(JinjaTokenType::LeftBracePercent)?;
                state.expect(JinjaTokenType::Identifier)?; // else
                state.expect(JinjaTokenType::PercentRightBrace)?;
                while state.not_at_end() && !(state.at(JinjaTokenType::LeftBracePercent) && state.peek_non_trivia_at(1).map(|t| state.source.get_text_in(t.span)).map(|t| t == "endif").unwrap_or(false)) {
                    self.parse_node(state)?;
                }
            }
        }

        if state.at(JinjaTokenType::LeftBracePercent) && state.peek_non_trivia_at(1).map(|t| state.source.get_text_in(t.span)).map(|t| t == "endif").unwrap_or(false) {
            state.expect(JinjaTokenType::LeftBracePercent)?;
            state.expect(JinjaTokenType::Identifier)?; // endif
            state.expect(JinjaTokenType::PercentRightBrace)?;
        }

        state.finish_at(cp, JinjaElementType::IfStatement);
        Ok(())
    }

    fn parse_for_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) -> Result<(), oak_core::OakError> {
        state.expect(JinjaTokenType::Identifier)?; // for
        self.parse_expression(state)?; // x
        // Jinja for syntax: for x in y
        if state.peek_text().map(|t| t == "in").unwrap_or(false) {
            state.advance();
            self.parse_expression(state)?; // y
        }
        state.expect(JinjaTokenType::PercentRightBrace)?;

        while state.not_at_end() {
            if state.at(JinjaTokenType::LeftBracePercent) {
                if let Some(JinjaTokenType::Identifier) = state.peek_non_trivia_kind_at(1) {
                    let text = state.peek_non_trivia_at(1).map(|t| state.source.get_text_in(t.span)).unwrap_or_default();
                    if text == "endfor" || text == "else" {
                        break;
                    }
                }
            }
            self.parse_node(state)?;
        }

        if state.at(JinjaTokenType::LeftBracePercent) {
            let text = state.peek_non_trivia_at(1).map(|t| state.source.get_text_in(t.span)).unwrap_or_default();
            if text == "else" {
                state.expect(JinjaTokenType::LeftBracePercent)?;
                state.expect(JinjaTokenType::Identifier)?; // else
                state.expect(JinjaTokenType::PercentRightBrace)?;
                while state.not_at_end() {
                    if state.at(JinjaTokenType::LeftBracePercent) {
                        if let Some(JinjaTokenType::Identifier) = state.peek_non_trivia_kind_at(1) {
                            let text = state.peek_non_trivia_at(1).map(|t| state.source.get_text_in(t.span)).unwrap_or_default();
                            if text == "endfor" {
                                break;
                            }
                        }
                    }
                    self.parse_node(state)?;
                }
            }
        }

        state.expect(JinjaTokenType::LeftBracePercent)?;
        state.expect(JinjaTokenType::Identifier)?; // endfor
        state.expect(JinjaTokenType::PercentRightBrace)?;

        state.finish_at(cp, JinjaElementType::ForStatement);
        Ok(())
    }

    fn parse_block_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) -> Result<(), oak_core::OakError> {
        state.expect(JinjaTokenType::Identifier)?; // block
        state.expect(JinjaTokenType::Identifier)?; // name
        state.expect(JinjaTokenType::PercentRightBrace)?;

        while state.not_at_end() && !(state.at(JinjaTokenType::LeftBracePercent) && state.peek_non_trivia_at(1).map(|t| state.source.get_text_in(t.span)).map(|t| t == "endblock").unwrap_or(false)) {
            self.parse_node(state)?;
        }

        state.expect(JinjaTokenType::LeftBracePercent)?;
        state.expect(JinjaTokenType::Identifier)?; // endblock
        if state.at(JinjaTokenType::Identifier) {
            state.advance();
        }
        state.expect(JinjaTokenType::PercentRightBrace)?;

        state.finish_at(cp, JinjaElementType::Block);
        Ok(())
    }

    fn parse_macro_definition<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) -> Result<(), oak_core::OakError> {
        state.expect(JinjaTokenType::Identifier)?; // macro
        state.expect(JinjaTokenType::Identifier)?; // name

        if state.at(JinjaTokenType::LeftParen) {
            state.advance();
            while state.not_at_end() && !state.at(JinjaTokenType::RightParen) {
                if state.at(JinjaTokenType::Identifier) {
                    state.advance();
                }
                if state.at(JinjaTokenType::Comma) {
                    state.advance();
                }
            }
            state.expect(JinjaTokenType::RightParen)?;
        }

        state.expect(JinjaTokenType::PercentRightBrace)?;

        while state.not_at_end() && !(state.at(JinjaTokenType::LeftBracePercent) && state.peek_non_trivia_at(1).map(|t| state.source.get_text_in(t.span)).map(|t| t == "endmacro").unwrap_or(false)) {
            self.parse_node(state)?;
        }

        state.expect(JinjaTokenType::LeftBracePercent)?;
        state.expect(JinjaTokenType::Identifier)?; // endmacro
        state.expect(JinjaTokenType::PercentRightBrace)?;

        state.finish_at(cp, JinjaElementType::MacroDefinition);
        Ok(())
    }

    fn parse_extends_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) -> Result<(), oak_core::OakError> {
        state.expect(JinjaTokenType::Identifier)?;
        self.parse_expression(state)?;
        state.expect(JinjaTokenType::PercentRightBrace)?;
        state.finish_at(cp, JinjaElementType::Extends);
        Ok(())
    }

    fn parse_include_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) -> Result<(), oak_core::OakError> {
        state.expect(JinjaTokenType::Identifier)?;
        self.parse_expression(state)?;
        state.expect(JinjaTokenType::PercentRightBrace)?;
        state.finish_at(cp, JinjaElementType::Include);
        Ok(())
    }

    fn parse_set_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) -> Result<(), oak_core::OakError> {
        state.expect(JinjaTokenType::Identifier)?;
        state.expect(JinjaTokenType::Identifier)?;
        state.expect(JinjaTokenType::Eq)?;
        self.parse_expression(state)?;
        state.expect(JinjaTokenType::PercentRightBrace)?;
        state.finish_at(cp, JinjaElementType::Set);
        Ok(())
    }

    fn parse_from_import_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) -> Result<(), oak_core::OakError> {
        state.expect(JinjaTokenType::Identifier)?;
        self.parse_expression(state)?;
        state.expect(JinjaTokenType::Identifier)?;
        while state.not_at_end() && !state.at(JinjaTokenType::PercentRightBrace) {
            state.advance();
        }
        state.expect(JinjaTokenType::PercentRightBrace)?;
        state.finish_at(cp, JinjaElementType::FromImport);
        Ok(())
    }

    fn parse_import_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) -> Result<(), oak_core::OakError> {
        state.expect(JinjaTokenType::Identifier)?;
        self.parse_expression(state)?;
        while state.not_at_end() && !state.at(JinjaTokenType::PercentRightBrace) {
            state.advance();
        }
        state.expect(JinjaTokenType::PercentRightBrace)?;
        state.finish_at(cp, JinjaElementType::Import);
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
            state.finish_at(cp, if kind == JinjaTokenType::Pipe { JinjaElementType::Filter } else { JinjaElementType::Expression });
        }

        Ok(())
    }

    fn get_precedence(&self, kind: JinjaTokenType) -> i32 {
        match kind {
            JinjaTokenType::Or => 0,
            JinjaTokenType::And => 1,
            JinjaTokenType::EqEq | JinjaTokenType::Neq | JinjaTokenType::Lt | JinjaTokenType::Gt | JinjaTokenType::LtEq | JinjaTokenType::GtEq => 3,
            JinjaTokenType::Pipe => 4,
            JinjaTokenType::Plus | JinjaTokenType::Minus => 5,
            JinjaTokenType::Star | JinjaTokenType::Slash => 6,
            _ => -1,
        }
    }

    fn parse_primary_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        let cp = state.checkpoint();

        match state.peek_kind() {
            Some(JinjaTokenType::Identifier) => {
                state.advance();
                loop {
                    if state.at(JinjaTokenType::Dot) {
                        state.advance();
                        if state.at(JinjaTokenType::Identifier) {
                            state.advance();
                        }
                    }
                    else if state.at(JinjaTokenType::LeftBracket) {
                        state.advance();
                        self.parse_expression(state)?;
                        state.expect(JinjaTokenType::RightBracket)?;
                    }
                    else {
                        break;
                    }
                }
                if state.at(JinjaTokenType::LeftParen) {
                    state.advance();
                    while state.not_at_end() && !state.at(JinjaTokenType::RightParen) {
                        self.parse_expression(state)?;
                        if state.at(JinjaTokenType::Comma) {
                            state.advance();
                        }
                    }
                    state.expect(JinjaTokenType::RightParen)?;
                    state.finish_at(cp, JinjaElementType::Function);
                }
                else {
                    state.finish_at(cp, JinjaElementType::Identifier);
                }
            }
            Some(JinjaTokenType::String) | Some(JinjaTokenType::Number) | Some(JinjaTokenType::Boolean) => {
                state.advance();
                state.finish_at(cp, JinjaElementType::Literal);
            }
            Some(JinjaTokenType::LeftParen) => {
                state.advance();
                self.parse_expression(state)?;
                state.expect(JinjaTokenType::RightParen)?;
            }
            Some(JinjaTokenType::Not) => {
                state.advance();
                self.parse_primary_expression(state)?;
            }
            _ => {
                while state.not_at_end() && !state.at(JinjaTokenType::PercentRightBrace) && !state.at(JinjaTokenType::DoubleRightBrace) && !state.at(JinjaTokenType::RightParen) && !state.at(JinjaTokenType::Comma) {
                    state.advance();
                }
            }
        }
        Ok(())
    }
}

impl<'config> Parser<JinjaLanguage> for JinjaParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<JinjaLanguage>) -> ParseOutput<'a, JinjaLanguage> {
        let lexer = JinjaLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();
            while state.not_at_end() {
                self.parse_node(state)?;
            }
            Ok(state.finish_at(checkpoint, JinjaElementType::Root))
        })
    }
}
