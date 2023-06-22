use crate::{kind::NixSyntaxKind, language::NixLanguage, lexer::NixLexer};
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
            Some(NixSyntaxKind::LeftBrace) => self.parse_set(state),
            Some(NixSyntaxKind::LeftBracket) => self.parse_list(state),
            Some(NixSyntaxKind::Let) => self.parse_let_in(state),
            Some(NixSyntaxKind::If) => self.parse_if_then_else(state),
            Some(NixSyntaxKind::Identifier) => {
                state.bump();
                if state.at(NixSyntaxKind::Colon) {
                    // This might be a lambda, but for now just bump
                    state.bump();
                    self.parse_expr(state);
                }
            }
            _ => {
                state.bump();
            }
        }
    }

    fn parse_set<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let cp = state.checkpoint();
        state.expect(NixSyntaxKind::LeftBrace).ok();
        while state.not_at_end() && !state.at(NixSyntaxKind::RightBrace) {
            self.parse_binding(state);
        }
        state.expect(NixSyntaxKind::RightBrace).ok();
        state.finish_at(cp, NixSyntaxKind::Set.into());
    }

    fn parse_list<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let cp = state.checkpoint();
        state.expect(NixSyntaxKind::LeftBracket).ok();
        while state.not_at_end() && !state.at(NixSyntaxKind::RightBracket) {
            self.parse_expr(state);
        }
        state.expect(NixSyntaxKind::RightBracket).ok();
        state.finish_at(cp, NixSyntaxKind::List.into());
    }

    fn parse_let_in<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let cp = state.checkpoint();
        state.expect(NixSyntaxKind::Let).ok();
        while state.not_at_end() && !state.at(NixSyntaxKind::In) {
            self.parse_binding(state);
        }
        if state.at(NixSyntaxKind::In) {
            state.bump();
            self.parse_expr(state);
        }
        state.finish_at(cp, NixSyntaxKind::LetIn.into());
    }

    fn parse_if_then_else<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let cp = state.checkpoint();
        state.expect(NixSyntaxKind::If).ok();
        self.parse_expr(state);
        if state.at(NixSyntaxKind::Then) {
            state.bump();
            self.parse_expr(state);
        }
        if state.at(NixSyntaxKind::Else) {
            state.bump();
            self.parse_expr(state);
        }
        state.finish_at(cp, NixSyntaxKind::IfThenElse.into());
    }

    fn parse_binding<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let cp = state.checkpoint();
        state.expect(NixSyntaxKind::Identifier).ok();
        if state.at(NixSyntaxKind::Assign) {
            state.bump();
            self.parse_expr(state);
            state.expect(NixSyntaxKind::Semicolon).ok();
        }
        state.finish_at(cp, NixSyntaxKind::Binding.into());
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

            Ok(state.finish_at(cp, NixSyntaxKind::Root.into()))
        })
    }
}
