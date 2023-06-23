pub mod element_type;

use crate::{
    language::NginxLanguage,
    lexer::{NginxLexer, token_type::NginxTokenType},
    parser::element_type::NginxElementType,
};
use oak_core::{
    GreenNode, OakError,
    parser::{ParseCache, ParseOutput, Parser, ParserState},
    source::{Source, TextEdit},
};

pub struct NginxParser<'a> {
    pub language: &'a NginxLanguage,
}

impl<'a> NginxParser<'a> {
    pub fn new(language: &'a NginxLanguage) -> Self {
        Self { language }
    }

    fn parse_directive<'b, S: Source + ?Sized>(&self, state: &mut ParserState<'b, NginxLanguage, S>) {
        if state.at(NginxTokenType::CommentToken) {
            let checkpoint = state.checkpoint();
            state.bump();
            state.finish_at(checkpoint, crate::parser::element_type::NginxElementType::Comment);
            return;
        }

        let is_block_directive = matches!(state.peek_kind(), Some(NginxTokenType::HttpKeyword | NginxTokenType::ServerKeyword | NginxTokenType::LocationKeyword | NginxTokenType::EventsKeyword | NginxTokenType::UpstreamKeyword));

        if is_block_directive {
            self.parse_block(state);
        }
        else {
            let checkpoint = state.checkpoint();
            state.bump(); // directive name
            while state.not_at_end() && !state.at(NginxTokenType::Semicolon) && !state.at(NginxTokenType::LeftBrace) {
                let p_checkpoint = state.checkpoint();
                state.bump();
                state.finish_at(p_checkpoint, crate::parser::element_type::NginxElementType::Parameter);
            }
            if state.at(NginxTokenType::Semicolon) {
                state.bump();
            }
            state.finish_at(checkpoint, crate::parser::element_type::NginxElementType::Directive);
        }
    }

    fn parse_block<'b, S: Source + ?Sized>(&self, state: &mut ParserState<'b, NginxLanguage, S>) {
        let checkpoint = state.checkpoint();
        state.bump(); // block keyword (http, server, etc.)

        // Optional parameters for location or upstream
        while state.not_at_end() && !state.at(NginxTokenType::LeftBrace) {
            let p_checkpoint = state.checkpoint();
            state.bump();
            state.finish_at(p_checkpoint, crate::parser::element_type::NginxElementType::Parameter);
        }

        if state.at(NginxTokenType::LeftBrace) {
            state.bump();
            while state.not_at_end() && !state.at(NginxTokenType::RightBrace) {
                self.parse_directive(state);
            }
            if state.at(NginxTokenType::RightBrace) {
                state.bump();
            }
        }
        state.finish_at(checkpoint, crate::parser::element_type::NginxElementType::Block);
    }

    fn parse_root_internal<'b, S: Source + ?Sized>(&self, state: &mut ParserState<'b, NginxLanguage, S>) -> Result<&'b GreenNode<'b, NginxLanguage>, OakError> {
        let checkpoint = state.checkpoint();
        while state.not_at_end() {
            self.parse_directive(state);
        }

        Ok(state.finish_at(checkpoint, crate::parser::element_type::NginxElementType::Root))
    }
}

impl<'a> Parser<NginxLanguage> for NginxParser<'a> {
    fn parse<'b, S: Source + ?Sized>(&self, text: &'b S, edits: &[TextEdit], cache: &'b mut impl ParseCache<NginxLanguage>) -> ParseOutput<'b, NginxLanguage> {
        let lexer = NginxLexer::new(self.language);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
