use crate::{kind::ProtobufSyntaxKind, language::ProtobufLanguage, lexer::ProtobufLexer};
use oak_core::{
    errors::OakError,
    parser::{ParseCache, ParseOutput, Parser, ParserState},
    source::{Source, TextEdit},
    tree::GreenNode,
};

pub(crate) type State<'a, S> = ParserState<'a, ProtobufLanguage, S>;

pub struct ProtobufParser<'a> {
    language: &'a ProtobufLanguage,
}

impl<'a> ProtobufParser<'a> {
    pub fn new(language: &'a ProtobufLanguage) -> Self {
        Self { language }
    }

    fn parse_root_internal<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) -> Result<&'b GreenNode<'b, ProtobufLanguage>, OakError> {
        let checkpoint = state.checkpoint();
        self.parse_program(state);
        Ok(state.finish_at(checkpoint, ProtobufSyntaxKind::Root))
    }

    fn parse_program<S: Source + ?Sized>(&self, state: &mut State<'_, S>) {
        while state.not_at_end() {
            if state.at(ProtobufSyntaxKind::Syntax) {
                self.parse_syntax(state);
            }
            else if state.at(ProtobufSyntaxKind::Package) {
                self.parse_package(state);
            }
            else if state.at(ProtobufSyntaxKind::Import) {
                self.parse_import(state);
            }
            else if state.at(ProtobufSyntaxKind::Option) {
                self.parse_option(state);
            }
            else if state.at(ProtobufSyntaxKind::Message) {
                self.parse_message(state);
            }
            else if state.at(ProtobufSyntaxKind::Enum) {
                self.parse_enum(state);
            }
            else if state.at(ProtobufSyntaxKind::Service) {
                self.parse_service(state);
            }
            else {
                state.bump();
            }
        }
    }

    fn parse_syntax<S: Source + ?Sized>(&self, state: &mut State<'_, S>) {
        let checkpoint = state.checkpoint();
        state.expect(ProtobufSyntaxKind::Syntax).ok();
        state.expect(ProtobufSyntaxKind::Assign).ok();
        state.expect(ProtobufSyntaxKind::StringLiteral).ok();
        state.expect(ProtobufSyntaxKind::Semicolon).ok();
        state.finish_at(checkpoint, ProtobufSyntaxKind::SyntaxDef);
    }

    fn parse_package<S: Source + ?Sized>(&self, state: &mut State<'_, S>) {
        let checkpoint = state.checkpoint();
        state.expect(ProtobufSyntaxKind::Package).ok();
        state.expect(ProtobufSyntaxKind::Identifier).ok();
        state.expect(ProtobufSyntaxKind::Semicolon).ok();
        state.finish_at(checkpoint, ProtobufSyntaxKind::PackageDef);
    }

    fn parse_import<S: Source + ?Sized>(&self, state: &mut State<'_, S>) {
        let checkpoint = state.checkpoint();
        state.expect(ProtobufSyntaxKind::Import).ok();
        if state.at(ProtobufSyntaxKind::Public) || state.at(ProtobufSyntaxKind::Weak) {
            state.bump();
        }
        state.expect(ProtobufSyntaxKind::StringLiteral).ok();
        state.expect(ProtobufSyntaxKind::Semicolon).ok();
        state.finish_at(checkpoint, ProtobufSyntaxKind::ImportDef);
    }

    fn parse_option<S: Source + ?Sized>(&self, state: &mut State<'_, S>) {
        let checkpoint = state.checkpoint();
        state.expect(ProtobufSyntaxKind::Option).ok();
        state.expect(ProtobufSyntaxKind::Identifier).ok();
        state.expect(ProtobufSyntaxKind::Assign).ok();
        state.bump(); // value
        state.expect(ProtobufSyntaxKind::Semicolon).ok();
        state.finish_at(checkpoint, ProtobufSyntaxKind::OptionDef);
    }

    fn parse_message<S: Source + ?Sized>(&self, state: &mut State<'_, S>) {
        let checkpoint = state.checkpoint();
        state.expect(ProtobufSyntaxKind::Message).ok();
        state.expect(ProtobufSyntaxKind::Identifier).ok();
        state.expect(ProtobufSyntaxKind::LeftBrace).ok();
        while !state.at(ProtobufSyntaxKind::RightBrace) && state.not_at_end() {
            state.bump();
        }
        state.expect(ProtobufSyntaxKind::RightBrace).ok();
        state.finish_at(checkpoint, ProtobufSyntaxKind::MessageDef);
    }

    fn parse_enum<S: Source + ?Sized>(&self, state: &mut State<'_, S>) {
        let checkpoint = state.checkpoint();
        state.expect(ProtobufSyntaxKind::Enum).ok();
        state.expect(ProtobufSyntaxKind::Identifier).ok();
        state.expect(ProtobufSyntaxKind::LeftBrace).ok();
        while !state.at(ProtobufSyntaxKind::RightBrace) && state.not_at_end() {
            state.bump();
        }
        state.expect(ProtobufSyntaxKind::RightBrace).ok();
        state.finish_at(checkpoint, ProtobufSyntaxKind::EnumDef);
    }

    fn parse_service<S: Source + ?Sized>(&self, state: &mut State<'_, S>) {
        let checkpoint = state.checkpoint();
        state.expect(ProtobufSyntaxKind::Service).ok();
        state.expect(ProtobufSyntaxKind::Identifier).ok();
        state.expect(ProtobufSyntaxKind::LeftBrace).ok();
        while !state.at(ProtobufSyntaxKind::RightBrace) && state.not_at_end() {
            state.bump();
        }
        state.expect(ProtobufSyntaxKind::RightBrace).ok();
        state.finish_at(checkpoint, ProtobufSyntaxKind::ServiceDef);
    }
}

impl<'config> Parser<ProtobufLanguage> for ProtobufParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<ProtobufLanguage>) -> ParseOutput<'a, ProtobufLanguage> {
        let lexer = ProtobufLexer::new(self.language);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
