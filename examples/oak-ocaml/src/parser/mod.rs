pub mod element_type;

use crate::{
    language::OCamlLanguage,
    lexer::{OCamlLexer, token_type::OCamlTokenType},
    parser::element_type::OCamlElementType,
};
use oak_core::{
    GreenNode, OakError,
    parser::{ParseCache, ParseOutput, Parser, ParserState},
    source::{Source, TextEdit},
};

pub struct OCamlParser<'config> {
    pub language: &'config OCamlLanguage,
}

impl<'config> OCamlParser<'config> {
    pub fn new(language: &'config OCamlLanguage) -> Self {
        Self { language }
    }

    fn parse_item<'s, S: Source + ?Sized>(&self, state: &mut ParserState<'s, OCamlLanguage, S>) {
        let kind = state.peek_kind();
        match kind {
            Some(OCamlTokenType::Let) => self.parse_let_binding(state),
            Some(OCamlTokenType::Module) => self.parse_module_def(state),
            Some(OCamlTokenType::Type) => self.parse_type_definition(state),
            _ => self.parse_expression(state),
        };
    }

    fn parse_let_binding<'s, S: Source + ?Sized>(&self, state: &mut ParserState<'s, OCamlLanguage, S>) {
        let checkpoint = state.checkpoint();
        state.bump(); // Let
        if state.at(OCamlTokenType::Rec) {
            state.bump(); // Rec
        }
        // Simplified: consume identifier and until = or ;;
        while state.not_at_end() && !state.at(OCamlTokenType::Equal) && !state.at(OCamlTokenType::Semicolon) {
            state.bump();
        }
        if state.at(OCamlTokenType::Equal) {
            state.bump(); // Equal
            self.parse_expression(state);
        }
        state.finish_at(checkpoint, OCamlElementType::LetBinding);
    }

    fn parse_module_def<'s, S: Source + ?Sized>(&self, state: &mut ParserState<'s, OCamlLanguage, S>) {
        let checkpoint = state.checkpoint();
        state.bump(); // Module
        while state.not_at_end() && !state.at(OCamlTokenType::Equal) && !state.at(OCamlTokenType::Semicolon) {
            state.bump();
        }
        if state.at(OCamlTokenType::Equal) {
            state.bump(); // Equal
            while state.not_at_end() && !state.at(OCamlTokenType::Semicolon) {
                state.bump();
            }
        }
        state.finish_at(checkpoint, OCamlElementType::ModuleDef);
    }

    fn parse_type_definition<'s, S: Source + ?Sized>(&self, state: &mut ParserState<'s, OCamlLanguage, S>) {
        let checkpoint = state.checkpoint();
        state.bump(); // Type
        while state.not_at_end() && !state.at(OCamlTokenType::Semicolon) {
            state.bump();
        }
        state.finish_at(checkpoint, OCamlElementType::TypeDefinition);
    }

    fn parse_expression<'s, S: Source + ?Sized>(&self, state: &mut ParserState<'s, OCamlLanguage, S>) {
        let checkpoint = state.checkpoint();
        while state.not_at_end() && !state.at(OCamlTokenType::Semicolon) {
            state.bump();
        }
        if state.at(OCamlTokenType::Semicolon) {
            state.bump(); // Semicolon
            if state.at(OCamlTokenType::Semicolon) {
                state.bump(); // Semicolon
            }
        }
        state.finish_at(checkpoint, OCamlElementType::Expression);
    }

    fn parse_root_internal<'s, S: Source + ?Sized>(&self, state: &mut ParserState<'s, OCamlLanguage, S>) -> Result<&'s GreenNode<'s, OCamlLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            self.parse_item(state);
        }

        Ok(state.finish_at(checkpoint, OCamlElementType::Root))
    }
}

impl<'config> Parser<OCamlLanguage> for OCamlParser<'config> {
    fn parse<'s, S: Source + ?Sized>(&self, text: &'s S, edits: &[TextEdit], cache: &'s mut impl ParseCache<OCamlLanguage>) -> ParseOutput<'s, OCamlLanguage> {
        let lexer = OCamlLexer::new(self.language);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
