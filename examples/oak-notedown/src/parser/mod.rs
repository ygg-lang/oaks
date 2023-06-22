use crate::{kind::NoteSyntaxKind, language::NotedownLanguage, lexer::NotedownLexer};
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

        Ok(state.finish_at(checkpoint, NoteSyntaxKind::Root))
    }

    fn parse_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let kind = state.peek_kind();
        match kind {
            Some(NoteSyntaxKind::Hash) => self.parse_heading(state),
            Some(NoteSyntaxKind::Asterisk) | Some(NoteSyntaxKind::Dash) | Some(NoteSyntaxKind::Plus) => self.parse_list_item(state),
            Some(NoteSyntaxKind::Pipe) => self.parse_table(state),
            Some(NoteSyntaxKind::Backtick) => self.parse_code_block(state),
            _ => self.parse_paragraph(state),
        }
    }

    fn parse_heading<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let checkpoint = state.checkpoint();
        let mut level = 0;
        while state.at(NoteSyntaxKind::Hash) {
            state.bump();
            level += 1;
        }

        while state.not_at_end() && !state.at(NoteSyntaxKind::Newline) {
            state.bump();
        }

        let kind = match level {
            1 => NoteSyntaxKind::Heading1,
            2 => NoteSyntaxKind::Heading2,
            3 => NoteSyntaxKind::Heading3,
            4 => NoteSyntaxKind::Heading4,
            5 => NoteSyntaxKind::Heading5,
            6 => NoteSyntaxKind::Heading6,
            _ => NoteSyntaxKind::Paragraph,
        };
        state.finish_at(checkpoint, kind);
    }

    fn parse_list_item<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let checkpoint = state.checkpoint();
        state.bump(); // marker
        while state.not_at_end() && !state.at(NoteSyntaxKind::Newline) {
            state.bump();
        }
        state.finish_at(checkpoint, NoteSyntaxKind::ListItem);
    }

    fn parse_table<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let checkpoint = state.checkpoint();
        while state.not_at_end() && state.at(NoteSyntaxKind::Pipe) {
            self.parse_table_row(state);
        }
        state.finish_at(checkpoint, NoteSyntaxKind::Table);
    }

    fn parse_table_row<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let checkpoint = state.checkpoint();
        while state.not_at_end() && !state.at(NoteSyntaxKind::Newline) {
            if state.at(NoteSyntaxKind::Pipe) {
                let cell_checkpoint = state.checkpoint();
                state.bump(); // |
                while state.not_at_end() && !state.at(NoteSyntaxKind::Pipe) && !state.at(NoteSyntaxKind::Newline) {
                    state.bump();
                }
                state.finish_at(cell_checkpoint, NoteSyntaxKind::TableCell);
            }
            else {
                state.bump();
            }
        }
        if state.at(NoteSyntaxKind::Newline) {
            state.bump();
        }
        state.finish_at(checkpoint, NoteSyntaxKind::TableRow);
    }

    fn parse_code_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let checkpoint = state.checkpoint();
        // Simplified code block parsing
        state.bump(); // ```
        while state.not_at_end() {
            if state.at(NoteSyntaxKind::Backtick) {
                state.bump();
                break;
            }
            state.bump();
        }
        state.finish_at(checkpoint, NoteSyntaxKind::CodeBlock);
    }

    fn parse_paragraph<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let checkpoint = state.checkpoint();
        while state.not_at_end() && !state.at(NoteSyntaxKind::Newline) {
            state.bump();
        }
        if state.at(NoteSyntaxKind::Newline) {
            state.bump();
        }
        state.finish_at(checkpoint, NoteSyntaxKind::Paragraph);
    }
}
