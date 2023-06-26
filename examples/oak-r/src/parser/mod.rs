/// Element type definitions for the R parser.
pub mod element_type;

use crate::{
    language::RLanguage,
    lexer::{RLexer, token_type::RTokenType},
    parser::element_type::RElementType,
};
use oak_core::{
    GreenNode, OakError, TextEdit,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::Source,
};

pub(crate) type State<'a, S> = ParserState<'a, RLanguage, S>;

/// Parser for the R programming language.
pub struct RParser<'config> {
    pub(crate) config: &'config RLanguage,
}

impl<'config> RParser<'config> {
    /// Creates a new RParser with the given language configuration.
    pub fn new(config: &'config RLanguage) -> Self {
        Self { config }
    }

    fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        if state.at(RTokenType::Identifier) && state.peek_at(1).map(|t| t.kind) == Some(RTokenType::LeftArrow) {
            let checkpoint = state.checkpoint();
            state.bump(); // ident
            state.bump(); // <-
            self.parse_expression(state);
            state.finish_at(checkpoint, RElementType::Assignment);
        }
        else if state.at(RTokenType::Function) {
            let checkpoint = state.checkpoint();
            state.bump(); // function
            if state.at(RTokenType::LeftParen) {
                state.bump();
                while state.not_at(RTokenType::RightParen) && state.not_at_end() {
                    if state.at(RTokenType::Identifier) {
                        state.bump();
                    }
                    if state.at(RTokenType::Comma) {
                        state.bump();
                    }
                }
                if state.at(RTokenType::RightParen) {
                    state.bump();
                }
            }
            if state.at(RTokenType::LeftBrace) {
                let body_checkpoint = state.checkpoint();
                state.bump();
                while state.not_at(RTokenType::RightBrace) && state.not_at_end() {
                    self.parse_statement(state);
                    while state.at(RTokenType::Newline) || state.at(RTokenType::Semicolon) {
                        state.bump();
                    }
                }
                if state.at(RTokenType::RightBrace) {
                    state.bump();
                }
                state.finish_at(body_checkpoint, RElementType::BlockExpression);
            }
            state.finish_at(checkpoint, RElementType::Function);
        }
        else {
            self.parse_expression(state);
        }
    }

    fn parse_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let checkpoint = state.checkpoint();
        if state.at(RTokenType::Identifier) {
            state.bump();
            if state.at(RTokenType::LeftParen) {
                state.bump();
                while state.not_at(RTokenType::RightParen) && state.not_at_end() {
                    self.parse_expression(state);
                    if state.at(RTokenType::Comma) {
                        state.bump();
                    }
                }
                if state.at(RTokenType::RightParen) {
                    state.bump();
                }
                state.finish_at(checkpoint, RElementType::CallExpression);
            }
            else {
                state.finish_at(checkpoint, RElementType::IdentifierExpression);
            }
        }
        else if state.at(RTokenType::IntegerLiteral) || state.at(RTokenType::FloatLiteral) || state.at(RTokenType::StringLiteral) || state.at(RTokenType::BooleanLiteral) {
            state.bump();
            state.finish_at(checkpoint, RElementType::LiteralExpression);
        }
        else {
            state.bump();
        }
    }
}

impl<'config> Parser<RLanguage> for RParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<RLanguage>) -> ParseOutput<'a, RLanguage> {
        let lexer = RLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();
            while state.not_at_end() {
                self.parse_statement(state);
                while state.at(RTokenType::Newline) || state.at(RTokenType::Semicolon) {
                    state.bump()
                }
            }
            Ok(state.finish_at(checkpoint, RElementType::Root))
        })
    }
}
