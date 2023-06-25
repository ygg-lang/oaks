/// WGSL element types.
pub mod element_type;

use crate::{language::WgslLanguage, lexer::token_type::WgslTokenType};
use oak_core::{
    TokenType,
    parser::{Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

/// A parser state for the WGSL language.
pub(crate) type State<'a, S> = ParserState<'a, WgslLanguage, S>;

/// A parser for the WGSL language.
pub struct WgslParser<'config> {
    pub(crate) config: &'config WgslLanguage,
}

impl<'config> WgslParser<'config> {
    /// Creates a new WGSL parser.
    pub fn new(config: &'config WgslLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<WgslLanguage> for WgslParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[oak_core::TextEdit], cache: &'a mut impl oak_core::ParseCache<WgslLanguage>) -> oak_core::ParseOutput<'a, WgslLanguage> {
        let lexer = crate::lexer::WgslLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let cp = state.checkpoint();
            while state.not_at_end() {
                if state.at(WgslTokenType::FnKw) {
                    self.parse_function(state);
                }
                else if state.at(WgslTokenType::StructKw) {
                    self.parse_struct(state);
                }
                else if state.at(WgslTokenType::VarKw) || state.at(WgslTokenType::LetKw) {
                    self.parse_variable(state);
                }
                else {
                    state.bump();
                }
            }
            Ok(state.finish_at(cp, element_type::WgslElementType::Root))
        })
    }
}

impl<'config> WgslParser<'config> {
    /// Parses a function definition.
    fn parse_function<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(WgslTokenType::FnKw).ok();
        state.expect(WgslTokenType::Identifier).ok();
        state.expect(WgslTokenType::LeftParen).ok();

        while state.not_at_end() && !state.at(WgslTokenType::RightParen) {
            self.parse_param(state);
            if state.at(WgslTokenType::Comma) {
                state.bump();
            }
        }
        state.expect(WgslTokenType::RightParen).ok();

        if state.eat(WgslTokenType::Arrow) {
            self.parse_type(state);
        }

        self.parse_block(state);
        state.finish_at(cp, element_type::WgslElementType::Function);
    }

    /// Parses a parameter.
    fn parse_param<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(WgslTokenType::Identifier).ok();
        state.expect(WgslTokenType::Colon).ok();
        self.parse_type(state);
        state.finish_at(cp, element_type::WgslElementType::Param);
    }

    /// Parses a type.
    fn parse_type<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        state.expect(WgslTokenType::Identifier).ok();
    }

    /// Parses a struct definition.
    fn parse_struct<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(WgslTokenType::StructKw).ok();
        state.expect(WgslTokenType::Identifier).ok();
        state.expect(WgslTokenType::LeftBrace).ok();
        while state.not_at_end() && !state.at(WgslTokenType::RightBrace) {
            self.parse_struct_member(state);
            if state.at(WgslTokenType::Comma) {
                state.bump();
            }
        }
        state.expect(WgslTokenType::RightBrace).ok();
        state.finish_at(cp, element_type::WgslElementType::Struct);
    }

    /// Parses a struct member.
    fn parse_struct_member<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(WgslTokenType::Identifier).ok();
        state.expect(WgslTokenType::Colon).ok();
        self.parse_type(state);
        if state.at(WgslTokenType::Comma) {
            state.bump();
        }
        state.finish_at(cp, element_type::WgslElementType::StructMember);
    }

    /// Parses a variable definition.
    fn parse_variable<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        if state.at(WgslTokenType::VarKw) {
            state.expect(WgslTokenType::VarKw).ok();
        }
        else {
            state.expect(WgslTokenType::LetKw).ok();
        }
        state.expect(WgslTokenType::Identifier).ok();
        if state.eat(WgslTokenType::Colon) {
            self.parse_type(state);
        }
        if state.eat(WgslTokenType::Assign) {
            // Simplified expression parsing
            while state.not_at_end() && !state.at(WgslTokenType::Semicolon) {
                state.bump()
            }
        }
        state.expect(WgslTokenType::Semicolon).ok();
        state.finish_at(cp, element_type::WgslElementType::Variable);
    }

    /// Parses a block statement.
    fn parse_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(WgslTokenType::LeftBrace).ok();
        while state.not_at_end() && !state.at(WgslTokenType::RightBrace) {
            state.bump()
        }
        state.expect(WgslTokenType::RightBrace).ok();
        state.finish_at(cp, element_type::WgslElementType::Block);
    }
}
