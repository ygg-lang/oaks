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

    fn is_at_start_of_line<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) -> bool {
        let pos = state.current_offset();
        if pos == 0 {
            return true;
        }
        let prev_text = state.source.get_text_in((pos - 1..pos).into());
        prev_text.as_ref() == "\n"
    }

    fn parse_item<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let kind = state.peek_kind();
        match kind {
            Some(OrgModeSyntaxKind::Star) => {
                if self.is_at_start_of_line(state) {
                    // 只有 * 后面跟着空格或者是行尾，才认为是标题
                    let mut is_heading = false;
                    let next_kind = state.peek_kind_at(1);
                    if next_kind == Some(OrgModeSyntaxKind::Whitespace) || next_kind == Some(OrgModeSyntaxKind::Newline) || next_kind.is_none() {
                        is_heading = true;
                    }

                    if is_heading {
                        self.parse_heading(state);
                    }
                    else {
                        self.parse_paragraph(state);
                    }
                }
                else {
                    self.parse_paragraph(state);
                }
            }
            Some(OrgModeSyntaxKind::Hash) => {
                if self.is_at_start_of_line(state) {
                    self.parse_block(state);
                }
                else {
                    self.parse_paragraph(state);
                }
            }
            Some(OrgModeSyntaxKind::Minus) | Some(OrgModeSyntaxKind::Plus) => {
                if self.is_at_start_of_line(state) {
                    self.parse_list(state);
                }
                else {
                    self.parse_paragraph(state);
                }
            }
            Some(OrgModeSyntaxKind::Newline) | Some(OrgModeSyntaxKind::Whitespace) => {
                state.bump();
            }
            _ => self.parse_paragraph(state),
        }
    }

    fn parse_list<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let checkpoint = state.checkpoint();
        while state.at(OrgModeSyntaxKind::Minus) || state.at(OrgModeSyntaxKind::Plus) {
            self.parse_list_item(state);
            // Handle optional newline/whitespace between items
            while state.at(OrgModeSyntaxKind::Newline) || state.at(OrgModeSyntaxKind::Whitespace) {
                state.bump();
            }
        }
        state.finish_at(checkpoint, OrgModeSyntaxKind::List.into());
    }

    fn parse_list_item<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let checkpoint = state.checkpoint();
        state.bump(); // - or +
        while state.not_at_end() && !state.at(OrgModeSyntaxKind::Newline) {
            state.bump();
        }
        state.finish_at(checkpoint, OrgModeSyntaxKind::ListItem.into());
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
            let next_kind = state.peek_kind();
            match next_kind {
                Some(OrgModeSyntaxKind::Star) => self.parse_bold(state),
                Some(OrgModeSyntaxKind::Slash) => self.parse_italic(state),
                Some(OrgModeSyntaxKind::Underscore) => self.parse_underline(state),
                Some(OrgModeSyntaxKind::LeftBracket) => self.parse_link(state),
                Some(OrgModeSyntaxKind::Tilde) => self.parse_inline_code(state),
                Some(OrgModeSyntaxKind::Equal) => self.parse_verbatim(state),
                Some(OrgModeSyntaxKind::Plus) => self.parse_strikethrough(state),
                _ => {
                    state.bump();
                }
            }
        }
        if state.at(OrgModeSyntaxKind::Newline) {
            state.bump();
        }
        state.finish_at(checkpoint, OrgModeSyntaxKind::Paragraph.into());
    }

    fn parse_bold<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let checkpoint = state.checkpoint();
        state.bump(); // *
        while state.not_at_end() && !state.at(OrgModeSyntaxKind::Star) && !state.at(OrgModeSyntaxKind::Newline) {
            state.bump();
        }
        if state.at(OrgModeSyntaxKind::Star) {
            state.bump();
        }
        state.finish_at(checkpoint, OrgModeSyntaxKind::Bold.into());
    }

    fn parse_italic<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let checkpoint = state.checkpoint();
        state.bump(); // /
        while state.not_at_end() && !state.at(OrgModeSyntaxKind::Slash) && !state.at(OrgModeSyntaxKind::Newline) {
            state.bump();
        }
        if state.at(OrgModeSyntaxKind::Slash) {
            state.bump();
        }
        state.finish_at(checkpoint, OrgModeSyntaxKind::Italic.into());
    }

    fn parse_underline<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let checkpoint = state.checkpoint();
        state.bump(); // _
        while state.not_at_end() && !state.at(OrgModeSyntaxKind::Underscore) && !state.at(OrgModeSyntaxKind::Newline) {
            state.bump();
        }
        if state.at(OrgModeSyntaxKind::Underscore) {
            state.bump();
        }
        state.finish_at(checkpoint, OrgModeSyntaxKind::Underline.into());
    }

    fn parse_link<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let checkpoint = state.checkpoint();
        state.bump(); // [
        if state.at(OrgModeSyntaxKind::LeftBracket) {
            state.bump(); // [[
            while state.not_at_end() && !state.at(OrgModeSyntaxKind::RightBracket) && !state.at(OrgModeSyntaxKind::Newline) {
                state.bump();
            }
            if state.at(OrgModeSyntaxKind::RightBracket) {
                state.bump(); // ]
                if state.at(OrgModeSyntaxKind::RightBracket) {
                    state.bump(); // ]]
                }
                else if state.at(OrgModeSyntaxKind::LeftBracket) {
                    state.bump(); // ][
                    while state.not_at_end() && !state.at(OrgModeSyntaxKind::RightBracket) && !state.at(OrgModeSyntaxKind::Newline) {
                        state.bump();
                    }
                    if state.at(OrgModeSyntaxKind::RightBracket) {
                        state.bump(); // ]
                        if state.at(OrgModeSyntaxKind::RightBracket) {
                            state.bump(); // ]]
                        }
                    }
                }
            }
        }
        state.finish_at(checkpoint, OrgModeSyntaxKind::Link.into());
    }

    fn parse_inline_code<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let checkpoint = state.checkpoint();
        state.bump(); // ~
        while state.not_at_end() && !state.at(OrgModeSyntaxKind::Tilde) && !state.at(OrgModeSyntaxKind::Newline) {
            state.bump();
        }
        if state.at(OrgModeSyntaxKind::Tilde) {
            state.bump();
        }
        state.finish_at(checkpoint, OrgModeSyntaxKind::InlineCode.into());
    }

    fn parse_verbatim<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let checkpoint = state.checkpoint();
        state.bump(); // =
        while state.not_at_end() && !state.at(OrgModeSyntaxKind::Equal) && !state.at(OrgModeSyntaxKind::Newline) {
            state.bump();
        }
        if state.at(OrgModeSyntaxKind::Equal) {
            state.bump();
        }
        state.finish_at(checkpoint, OrgModeSyntaxKind::Verbatim.into());
    }

    fn parse_strikethrough<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let checkpoint = state.checkpoint();
        state.bump(); // +
        while state.not_at_end() && !state.at(OrgModeSyntaxKind::Plus) && !state.at(OrgModeSyntaxKind::Newline) {
            state.bump();
        }
        if state.at(OrgModeSyntaxKind::Plus) {
            state.bump();
        }
        state.finish_at(checkpoint, OrgModeSyntaxKind::Strikethrough.into());
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
