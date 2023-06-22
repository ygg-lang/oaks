use crate::{kind::ObjectiveCSyntaxKind, language::ObjectiveCLanguage, lexer::ObjectiveCLexer};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, ObjectiveCLanguage, S>;

pub struct ObjectiveCParser<'config> {
    pub language: &'config ObjectiveCLanguage,
}

impl<'config> ObjectiveCParser<'config> {
    pub fn new(language: &'config ObjectiveCLanguage) -> Self {
        Self { language }
    }

    fn parse_item<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        if state.at(ObjectiveCSyntaxKind::At) {
            state.bump();
            if state.at(ObjectiveCSyntaxKind::InterfaceKeyword) {
                self.parse_interface(state);
            }
            else if state.at(ObjectiveCSyntaxKind::ImplementationKeyword) {
                self.parse_implementation(state);
            }
            else if state.at(ObjectiveCSyntaxKind::ProtocolKeyword) {
                self.parse_protocol(state);
            }
            else {
                let checkpoint = state.checkpoint();
                state.bump();
                state.finish_at(checkpoint, ObjectiveCSyntaxKind::Error.into());
            }
        }
        else if state.at(ObjectiveCSyntaxKind::ImportKeyword) || state.at(ObjectiveCSyntaxKind::IncludeKeyword) {
            self.parse_import(state);
        }
        else {
            // Simplified: just consume tokens
            state.bump();
        }
    }

    fn parse_interface<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let checkpoint = state.checkpoint();
        state.expect(ObjectiveCSyntaxKind::InterfaceKeyword).ok();
        // Simplified: consume until @end
        while state.not_at_end() && !state.at(ObjectiveCSyntaxKind::EndKeyword) {
            state.bump();
        }
        if state.at(ObjectiveCSyntaxKind::EndKeyword) {
            state.expect(ObjectiveCSyntaxKind::EndKeyword).ok();
        }
        state.finish_at(checkpoint, ObjectiveCSyntaxKind::InterfaceDeclaration.into());
    }

    fn parse_implementation<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let checkpoint = state.checkpoint();
        state.expect(ObjectiveCSyntaxKind::ImplementationKeyword).ok();
        // Simplified: consume until @end
        while state.not_at_end() && !state.at(ObjectiveCSyntaxKind::EndKeyword) {
            state.bump();
        }
        if state.at(ObjectiveCSyntaxKind::EndKeyword) {
            state.expect(ObjectiveCSyntaxKind::EndKeyword).ok();
        }
        state.finish_at(checkpoint, ObjectiveCSyntaxKind::ImplementationDeclaration.into());
    }

    fn parse_protocol<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let checkpoint = state.checkpoint();
        state.expect(ObjectiveCSyntaxKind::ProtocolKeyword).ok();
        // Simplified: consume until @end
        while state.not_at_end() && !state.at(ObjectiveCSyntaxKind::EndKeyword) {
            state.bump();
        }
        if state.at(ObjectiveCSyntaxKind::EndKeyword) {
            state.expect(ObjectiveCSyntaxKind::EndKeyword).ok();
        }
        state.finish_at(checkpoint, ObjectiveCSyntaxKind::ProtocolDeclaration.into());
    }

    fn parse_import<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        // Simplified: consume until semicolon or newline
        state.bump(); // #import or #include
        while state.not_at_end() && !state.at(ObjectiveCSyntaxKind::Semicolon) && !state.at(ObjectiveCSyntaxKind::Newline) {
            state.bump();
        }
        if state.at(ObjectiveCSyntaxKind::Semicolon) {
            state.expect(ObjectiveCSyntaxKind::Semicolon).ok();
        }
    }
}

impl<'config> Parser<ObjectiveCLanguage> for ObjectiveCParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<ObjectiveCLanguage>) -> ParseOutput<'a, ObjectiveCLanguage> {
        let lexer = ObjectiveCLexer::new(self.language);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                self.parse_item(state);
            }

            Ok(state.finish_at(checkpoint, ObjectiveCSyntaxKind::Root.into()))
        })
    }
}
