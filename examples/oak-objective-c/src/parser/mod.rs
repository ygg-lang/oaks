pub mod element_type;

use crate::{
    language::ObjectiveCLanguage,
    lexer::{ObjectiveCLexer, token_type::ObjectiveCTokenType},
    parser::element_type::ObjectiveCElementType,
};
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
        if state.at(ObjectiveCTokenType::At) {
            state.bump();
            if state.at(ObjectiveCTokenType::InterfaceKeyword) {
                self.parse_interface(state);
            }
            else if state.at(ObjectiveCTokenType::ImplementationKeyword) {
                self.parse_implementation(state);
            }
            else if state.at(ObjectiveCTokenType::ProtocolKeyword) {
                self.parse_protocol(state);
            }
            else {
                let checkpoint = state.checkpoint();
                state.bump();
                state.finish_at(checkpoint, crate::parser::element_type::ObjectiveCElementType::Error);
            }
        }
        else if state.at(ObjectiveCTokenType::ImportKeyword) || state.at(ObjectiveCTokenType::IncludeKeyword) {
            self.parse_import(state);
        }
        else {
            // Simplified: just consume tokens
            state.bump();
        }
    }

    fn parse_interface<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let checkpoint = state.checkpoint();
        state.expect(ObjectiveCTokenType::InterfaceKeyword).ok();
        // Simplified: consume until @end
        while state.not_at_end() && !state.at(ObjectiveCTokenType::EndKeyword) {
            state.bump();
        }
        if state.at(ObjectiveCTokenType::EndKeyword) {
            state.expect(ObjectiveCTokenType::EndKeyword).ok();
        }
        state.finish_at(checkpoint, crate::parser::element_type::ObjectiveCElementType::InterfaceDeclaration);
    }

    fn parse_implementation<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let checkpoint = state.checkpoint();
        state.expect(ObjectiveCTokenType::ImplementationKeyword).ok();
        // Simplified: consume until @end
        while state.not_at_end() && !state.at(ObjectiveCTokenType::EndKeyword) {
            state.bump();
        }
        if state.at(ObjectiveCTokenType::EndKeyword) {
            state.expect(ObjectiveCTokenType::EndKeyword).ok();
        }
        state.finish_at(checkpoint, crate::parser::element_type::ObjectiveCElementType::ImplementationDeclaration);
    }

    fn parse_protocol<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let checkpoint = state.checkpoint();
        state.expect(ObjectiveCTokenType::ProtocolKeyword).ok();
        // Simplified: consume until @end
        while state.not_at_end() && !state.at(ObjectiveCTokenType::EndKeyword) {
            state.bump();
        }
        if state.at(ObjectiveCTokenType::EndKeyword) {
            state.expect(ObjectiveCTokenType::EndKeyword).ok();
        }
        state.finish_at(checkpoint, crate::parser::element_type::ObjectiveCElementType::ProtocolDeclaration);
    }

    fn parse_import<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        // Simplified: consume until semicolon or newline
        state.bump(); // #import or #include
        while state.not_at_end() && !state.at(ObjectiveCTokenType::Semicolon) && !state.at(ObjectiveCTokenType::Newline) {
            state.bump();
        }
        if state.at(ObjectiveCTokenType::Semicolon) {
            state.expect(ObjectiveCTokenType::Semicolon).ok();
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

            Ok(state.finish_at(checkpoint, crate::parser::element_type::ObjectiveCElementType::Root))
        })
    }
}
