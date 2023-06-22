use crate::{kind::OrgModeSyntaxKind, language::OrgModeLanguage, lexer::OrgModeLexer};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, OrgModeLanguage, S>;

pub struct OrgModeParser<'a> {
    pub language: &'a OrgModeLanguage,
}

impl<'a> OrgModeParser<'a> {
    pub fn new(language: &'a OrgModeLanguage) -> Self {
        Self { language }
    }

    fn parse_item<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let kind = state.peek_kind();
        match kind {
            Some(OrgModeSyntaxKind::Star) => self.parse_heading(state),
            Some(OrgModeSyntaxKind::Hash) => self.parse_block(state),
            _ => self.parse_paragraph(state),
        }
    }

    fn parse_heading<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let checkpoint = state.checkpoint();
        while state.at(OrgModeSyntaxKind::Star) {
            state.bump();
        }

        while state.not_at_end() && !state.at(OrgModeSyntaxKind::Newline) {
            state.bump();
        }
        state.finish_at(checkpoint, OrgModeSyntaxKind::Heading.into());
    }

    fn parse_block<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let checkpoint = state.checkpoint();
        state.bump(); // #
        while state.not_at_end() && !state.at(OrgModeSyntaxKind::Newline) {
            state.bump();
        }
        state.finish_at(checkpoint, OrgModeSyntaxKind::Block.into());
    }

    fn parse_paragraph<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let checkpoint = state.checkpoint();
        while state.not_at_end() && !state.at(OrgModeSyntaxKind::Newline) {
            state.bump();
        }
        if state.at(OrgModeSyntaxKind::Newline) {
            state.bump();
        }
        state.finish_at(checkpoint, OrgModeSyntaxKind::Paragraph.into());
    }
}

impl<'a> Parser<OrgModeLanguage> for OrgModeParser<'a> {
    fn parse<'b, S: Source + ?Sized>(&self, text: &'b S, edits: &[TextEdit], cache: &'b mut impl ParseCache<OrgModeLanguage>) -> ParseOutput<'b, OrgModeLanguage> {
        let lexer = OrgModeLexer::new(self.language);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();
            while state.not_at_end() {
                self.parse_item(state);
            }

            Ok(state.finish_at(checkpoint, OrgModeSyntaxKind::Document.into()))
        })
    }
}
