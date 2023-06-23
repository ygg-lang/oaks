pub mod element_type;

use crate::{
    language::NotedownLanguage,
    lexer::{NotedownLexer, token_type::NoteTokenType},
    parser::element_type::NoteElementType,
};
use oak_core::{
    TextEdit,
    errors::OakError,
    parser::{ParseCache, ParseOutput, Parser, ParserState},
    source::Source,
    tree::GreenNode,
};

type State<'a, S> = ParserState<'a, NotedownLanguage, S>;

pub struct NoteParser<'a> {
    pub language: &'a NotedownLanguage,
}

impl<'a> NoteParser<'a> {
    pub fn new(language: &'a NotedownLanguage) -> Self {
        Self { language }
    }
}

impl<'p> Parser<NotedownLanguage> for NoteParser<'p> {
    fn parse<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<NotedownLanguage>) -> ParseOutput<'a, NotedownLanguage> {
        let lexer = NotedownLexer::new(self.language);
        oak_core::parser::parse_with_lexer(&lexer, source, edits, cache, |state| self.parse_root_internal(state))
    }
}

impl<'p> NoteParser<'p> {
    fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, NotedownLanguage>, OakError> {
        let checkpoint = state.checkpoint();
        while state.not_at_end() {
            self.parse_block(state);
        }

        Ok(state.finish_at(checkpoint, NoteElementType::Root))
    }

    fn parse_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let kind = state.peek_kind();
        match kind {
            Some(NoteTokenType::Hash) => self.parse_heading(state),
            Some(NoteTokenType::Asterisk) | Some(NoteTokenType::Dash) | Some(NoteTokenType::Plus) => self.parse_list_item(state),
            Some(NoteTokenType::Pipe) => self.parse_table(state),
            Some(NoteTokenType::Backtick) => self.parse_code_block(state),
            _ => self.parse_paragraph(state),
        }
    }

    fn parse_heading<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let checkpoint = state.checkpoint();
        let mut level = 0;
        while state.at(NoteTokenType::Hash) {
            state.bump();
            level += 1;
        }

        while state.not_at_end() && !state.at(NoteTokenType::Newline) {
            state.bump();
        }

        let kind = match level {
            1 => NoteElementType::Heading,
            2 => NoteElementType::Heading,
            3 => NoteElementType::Heading,
            4 => NoteElementType::Heading,
            5 => NoteElementType::Heading,
            6 => NoteElementType::Heading,
            _ => NoteElementType::Paragraph,
        };
        state.finish_at(checkpoint, kind);
    }

    fn parse_list_item<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let checkpoint = state.checkpoint();
        state.bump(); // marker
        while state.not_at_end() && !state.at(NoteTokenType::Newline) {
            state.bump();
        }
        state.finish_at(checkpoint, NoteElementType::ListItem);
    }

    fn parse_table<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let checkpoint = state.checkpoint();
        while state.not_at_end() && state.at(NoteTokenType::Pipe) {
            self.parse_table_row(state);
        }
        state.finish_at(checkpoint, NoteElementType::Table);
    }

    fn parse_table_row<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let checkpoint = state.checkpoint();
        while state.not_at_end() && !state.at(NoteTokenType::Newline) {
            if state.at(NoteTokenType::Pipe) {
                let cell_checkpoint = state.checkpoint();
                state.bump(); // |
                while state.not_at_end() && !state.at(NoteTokenType::Pipe) && !state.at(NoteTokenType::Newline) {
                    state.bump();
                }
                state.finish_at(cell_checkpoint, NoteElementType::Token(NoteTokenType::TableCell));
            }
            else {
                state.bump();
            }
        }
        if state.at(NoteTokenType::Newline) {
            state.bump();
        }
        state.finish_at(checkpoint, crate::parser::element_type::NoteElementType::TableRow);
    }

    fn parse_code_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let checkpoint = state.checkpoint();
        // Simplified code block parsing
        state.bump(); // ```
        while state.not_at_end() {
            if state.at(NoteTokenType::Backtick) {
                state.bump();
                break;
            }
            state.bump();
        }
        state.finish_at(checkpoint, crate::parser::element_type::NoteElementType::CodeBlock);
    }

    fn parse_paragraph<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let checkpoint = state.checkpoint();
        while state.not_at_end() && !state.at(NoteTokenType::Newline) {
            state.bump();
        }
        if state.at(NoteTokenType::Newline) {
            state.bump();
        }
        state.finish_at(checkpoint, crate::parser::element_type::NoteElementType::Paragraph);
    }
}
