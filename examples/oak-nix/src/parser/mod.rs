pub mod element_type;

use crate::{
    language::NixLanguage,
    lexer::{NixLexer, token_type::NixTokenType},
};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, NixLanguage, S>;

pub struct NixParser<'a> {
    pub language: &'a NixLanguage,
}

impl<'a> NixParser<'a> {
    pub fn new(language: &'a NixLanguage) -> Self {
        Self { language }
    }

    fn parse_expr<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        match state.peek_kind() {
            Some(NixTokenType::LeftBrace) => self.parse_set(state),
            Some(NixTokenType::LeftBracket) => self.parse_list(state),
            Some(NixTokenType::Let) => self.parse_let_in(state),
            Some(NixTokenType::If) => self.parse_if_then_else(state),
            Some(NixTokenType::Identifier) => {
                state.bump();
                if state.at(NixTokenType::Colon) {
                    // This might be a lambda, but for now just bump
                    state.bump();
                    self.parse_expr(state)
                }
            }
            _ => state.bump(),
        }
    }

    fn parse_set<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let cp = state.checkpoint();
        state.expect(NixTokenType::LeftBrace).ok();
        while state.not_at_end() && !state.at(NixTokenType::RightBrace) {
            self.parse_binding(state)
        }
        state.expect(NixTokenType::RightBrace).ok();
        state.finish_at(cp, crate::parser::element_type::NixElementType::Set);
    }

    fn parse_list<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let cp = state.checkpoint();
        state.expect(NixTokenType::LeftBracket).ok();
        while state.not_at_end() && !state.at(NixTokenType::RightBracket) {
            self.parse_expr(state)
        }
        state.expect(NixTokenType::RightBracket).ok();
        state.finish_at(cp, crate::parser::element_type::NixElementType::List);
    }

    fn parse_let_in<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let cp = state.checkpoint();
        state.expect(NixTokenType::Let).ok();
        while state.not_at_end() && !state.at(NixTokenType::In) {
            self.parse_binding(state)
        }
        if state.at(NixTokenType::In) {
            state.bump();
            self.parse_expr(state)
        }
        state.finish_at(cp, crate::parser::element_type::NixElementType::LetIn);
    }

    fn parse_if_then_else<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let cp = state.checkpoint();
        state.expect(NixTokenType::If).ok();
        self.parse_expr(state);
        if state.at(NixTokenType::Then) {
            state.bump();
            self.parse_expr(state)
        }
        if state.at(NixTokenType::Else) {
            state.bump();
            self.parse_expr(state)
        }
        state.finish_at(cp, crate::parser::element_type::NixElementType::IfThenElse);
    }

    fn parse_binding<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let cp = state.checkpoint();
        state.expect(NixTokenType::Identifier).ok();
        if state.at(NixTokenType::Assign) {
            state.bump();
            self.parse_expr(state);
            state.expect(NixTokenType::Semicolon).ok();
        }
        state.finish_at(cp, crate::parser::element_type::NixElementType::Binding);
    }
}

impl<'config> Parser<NixLanguage> for NixParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<NixLanguage>) -> ParseOutput<'a, NixLanguage> {
        let lexer = NixLexer::new(self.language);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let cp = state.checkpoint();

            while state.not_at_end() {
                self.parse_expr(state);
            }

            Ok(state.finish_at(cp, crate::parser::element_type::NixElementType::Root))
        })
    }
}
