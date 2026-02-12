/// Element types and tree structure definitions for Protobuf.
pub mod element_type;

use crate::{
    language::ProtobufLanguage,
    lexer::{ProtobufLexer, token_type::ProtobufTokenType},
};
use oak_core::{
    errors::OakError,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
    tree::GreenNode,
};

pub(crate) type State<'a, S> = ParserState<'a, ProtobufLanguage, S>;

/// Parser implementation for the Protobuf language.
pub struct ProtobufParser<'a> {
    language: &'a ProtobufLanguage,
}

impl<'a> ProtobufParser<'a> {
    /// Creates a new instance of the Protobuf parser with the given configuration.
    pub fn new(language: &'a ProtobufLanguage) -> Self {
        Self { language }
    }

    /// Parses the top-level Protobuf program.
    fn parse_program<S: Source + ?Sized>(&self, state: &mut State<'_, S>) {
        while state.not_at_end() {
            if state.at(ProtobufTokenType::Syntax) {
                self.parse_syntax(state);
            }
            else if state.at(ProtobufTokenType::Package) {
                self.parse_package(state);
            }
            else if state.at(ProtobufTokenType::Import) {
                self.parse_import(state);
            }
            else if state.at(ProtobufTokenType::Option) {
                self.parse_option(state);
            }
            else if state.at(ProtobufTokenType::Message) {
                self.parse_message(state);
            }
            else if state.at(ProtobufTokenType::Enum) {
                self.parse_enum(state);
            }
            else if state.at(ProtobufTokenType::Service) {
                self.parse_service(state);
            }
            else {
                state.bump();
            }
        }
    }

    /// Parses a syntax definition statement.
    fn parse_syntax<S: Source + ?Sized>(&self, state: &mut State<'_, S>) {
        let checkpoint = state.checkpoint();
        state.expect(ProtobufTokenType::Syntax).ok();
        state.expect(ProtobufTokenType::Assign).ok();
        state.expect(ProtobufTokenType::StringLiteral).ok();
        state.expect(ProtobufTokenType::Semicolon).ok();
        state.finish_at(checkpoint, crate::parser::element_type::ProtobufElementType::SyntaxDef);
    }

    /// Parses a package definition statement.
    fn parse_package<S: Source + ?Sized>(&self, state: &mut State<'_, S>) {
        let checkpoint = state.checkpoint();
        state.expect(ProtobufTokenType::Package).ok();
        state.expect(ProtobufTokenType::Identifier).ok();
        state.expect(ProtobufTokenType::Semicolon).ok();
        state.finish_at(checkpoint, crate::parser::element_type::ProtobufElementType::PackageDef);
    }

    /// Parses an import definition statement.
    fn parse_import<S: Source + ?Sized>(&self, state: &mut State<'_, S>) {
        let checkpoint = state.checkpoint();
        state.expect(ProtobufTokenType::Import).ok();
        if state.at(ProtobufTokenType::Public) || state.at(ProtobufTokenType::Weak) {
            state.bump()
        }
        state.expect(ProtobufTokenType::StringLiteral).ok();
        state.expect(ProtobufTokenType::Semicolon).ok();
        state.finish_at(checkpoint, crate::parser::element_type::ProtobufElementType::ImportDef);
    }

    /// Parses an option definition statement.
    fn parse_option<S: Source + ?Sized>(&self, state: &mut State<'_, S>) {
        let checkpoint = state.checkpoint();
        state.expect(ProtobufTokenType::Option).ok();
        state.expect(ProtobufTokenType::Identifier).ok();
        state.expect(ProtobufTokenType::Assign).ok();
        state.bump(); // value
        state.expect(ProtobufTokenType::Semicolon).ok();
        state.finish_at(checkpoint, crate::parser::element_type::ProtobufElementType::OptionDef);
    }

    /// Parses a message definition block.
    fn parse_message<S: Source + ?Sized>(&self, state: &mut State<'_, S>) {
        let checkpoint = state.checkpoint();
        state.expect(ProtobufTokenType::Message).ok();
        state.expect(ProtobufTokenType::Identifier).ok();
        state.expect(ProtobufTokenType::LeftBrace).ok();
        while !state.at(ProtobufTokenType::RightBrace) && state.not_at_end() {
            state.bump()
        }
        state.expect(ProtobufTokenType::RightBrace).ok();
        state.finish_at(checkpoint, crate::parser::element_type::ProtobufElementType::MessageDef);
    }

    /// Parses an enum definition block.
    fn parse_enum<S: Source + ?Sized>(&self, state: &mut State<'_, S>) {
        let checkpoint = state.checkpoint();
        state.expect(ProtobufTokenType::Enum).ok();
        state.expect(ProtobufTokenType::Identifier).ok();
        state.expect(ProtobufTokenType::LeftBrace).ok();
        while !state.at(ProtobufTokenType::RightBrace) && state.not_at_end() {
            state.bump()
        }
        state.expect(ProtobufTokenType::RightBrace).ok();
        state.finish_at(checkpoint, crate::parser::element_type::ProtobufElementType::EnumDef);
    }

    /// Parses a service definition block.
    fn parse_service<S: Source + ?Sized>(&self, state: &mut State<'_, S>) {
        let checkpoint = state.checkpoint();
        state.expect(ProtobufTokenType::Service).ok();
        state.expect(ProtobufTokenType::Identifier).ok();
        state.expect(ProtobufTokenType::LeftBrace).ok();
        while !state.at(ProtobufTokenType::RightBrace) && state.not_at_end() {
            state.bump()
        }
        state.expect(ProtobufTokenType::RightBrace).ok();
        state.finish_at(checkpoint, crate::parser::element_type::ProtobufElementType::ServiceDef);
    }
}

impl<'a> Parser<ProtobufLanguage> for ProtobufParser<'a> {
    fn parse<'b, S: Source + ?Sized>(&self, text: &'b S, edits: &[TextEdit], cache: &'b mut impl ParseCache<ProtobufLanguage>) -> ParseOutput<'b, ProtobufLanguage> {
        let lexer = ProtobufLexer::new(self.language);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();
            self.parse_program(state);
            Ok(state.finish_at(checkpoint, element_type::ProtobufElementType::Root))
        })
    }
}
