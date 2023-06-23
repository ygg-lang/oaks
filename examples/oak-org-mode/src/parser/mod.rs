pub mod element_type;

use crate::{
    language::OrgModeLanguage,
    lexer::{OrgModeLexer, token_type::OrgModeTokenType},
    parser::element_type::OrgModeElementType,
};
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
            Some(OrgModeTokenType::Star) => {
                if self.is_at_start_of_line(state) {
                    // 只有 * 后面跟着空格或者是行尾，才认为是标题
                    let mut is_heading = false;
                    let next_kind = state.peek_kind_at(1);
                    if next_kind == Some(OrgModeTokenType::Whitespace) || next_kind == Some(OrgModeTokenType::Newline) || next_kind.is_none() {
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
            Some(OrgModeTokenType::Hash) => {
                if self.is_at_start_of_line(state) {
                    self.parse_block(state);
                }
                else {
                    self.parse_paragraph(state);
                }
            }
            Some(OrgModeTokenType::Minus) | Some(OrgModeTokenType::Plus) => {
                if self.is_at_start_of_line(state) {
                    self.parse_list(state);
                }
                else {
                    self.parse_paragraph(state);
                }
            }
            Some(OrgModeTokenType::Newline) | Some(OrgModeTokenType::Whitespace) => {
                state.bump();
            }
            _ => {
                self.parse_paragraph(state);
            }
        }
    }

    fn parse_list<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let checkpoint = state.checkpoint();
        while state.at(OrgModeTokenType::Minus) || state.at(OrgModeTokenType::Plus) {
            self.parse_list_item(state);
            // Handle optional newline/whitespace between items
            while state.at(OrgModeTokenType::Newline) || state.at(OrgModeTokenType::Whitespace) {
                state.bump();
            }
        }
        state.finish_at(checkpoint, OrgModeElementType::List);
    }

    fn parse_inline_content<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        while state.not_at_end() && !state.at(OrgModeTokenType::Newline) {
            let next_kind = state.peek_kind();
            match next_kind {
                Some(OrgModeTokenType::Star) => {
                    self.parse_bold(state);
                }
                Some(OrgModeTokenType::Slash) => {
                    self.parse_italic(state);
                }
                Some(OrgModeTokenType::Underscore) => {
                    self.parse_underline(state);
                }
                Some(OrgModeTokenType::LeftBracket) => {
                    self.parse_link(state);
                }
                Some(OrgModeTokenType::Tilde) => {
                    self.parse_inline_code(state);
                }
                Some(OrgModeTokenType::Equal) => {
                    self.parse_verbatim(state);
                }
                Some(OrgModeTokenType::Plus) => {
                    self.parse_strikethrough(state);
                }
                _ => {
                    state.bump();
                }
            }
        }
    }

    fn parse_list_item<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let checkpoint = state.checkpoint();
        state.bump(); // - or +
        self.parse_inline_content(state);
        state.finish_at(checkpoint, OrgModeElementType::ListItem);
    }

    fn parse_heading<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let checkpoint = state.checkpoint();
        while state.at(OrgModeTokenType::Star) {
            state.bump();
        }

        self.parse_inline_content(state);
        state.finish_at(checkpoint, OrgModeElementType::Heading);
    }

    fn parse_block<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let checkpoint = state.checkpoint();
        state.bump(); // #
        while state.not_at_end() && !state.at(OrgModeTokenType::Newline) {
            state.bump();
        }
        state.finish_at(checkpoint, OrgModeElementType::Block);
    }

    fn parse_paragraph<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let checkpoint = state.checkpoint();
        self.parse_inline_content(state);
        if state.at(OrgModeTokenType::Newline) {
            state.bump();
        }
        state.finish_at(checkpoint, OrgModeElementType::Paragraph);
    }

    fn parse_bold<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let checkpoint = state.checkpoint();
        state.bump(); // *
        while state.not_at_end() && !state.at(OrgModeTokenType::Star) && !state.at(OrgModeTokenType::Newline) {
            state.bump();
        }
        if state.at(OrgModeTokenType::Star) {
            state.bump();
        }
        state.finish_at(checkpoint, OrgModeElementType::Bold);
    }

    fn parse_italic<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let checkpoint = state.checkpoint();
        state.bump(); // /
        while state.not_at_end() && !state.at(OrgModeTokenType::Slash) && !state.at(OrgModeTokenType::Newline) {
            state.bump();
        }
        if state.at(OrgModeTokenType::Slash) {
            state.bump();
        }
        state.finish_at(checkpoint, OrgModeElementType::Italic);
    }

    fn parse_underline<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let checkpoint = state.checkpoint();
        state.bump(); // _
        while state.not_at_end() && !state.at(OrgModeTokenType::Underscore) && !state.at(OrgModeTokenType::Newline) {
            state.bump();
        }
        if state.at(OrgModeTokenType::Underscore) {
            state.bump();
        }
        state.finish_at(checkpoint, OrgModeElementType::Underline);
    }

    fn parse_link<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let checkpoint = state.checkpoint();
        state.bump(); // [
        if state.at(OrgModeTokenType::LeftBracket) {
            state.bump(); // [[
            while state.not_at_end() && !state.at(OrgModeTokenType::RightBracket) && !state.at(OrgModeTokenType::Newline) {
                state.bump();
            }
            if state.at(OrgModeTokenType::RightBracket) {
                state.bump(); // ]
                if state.at(OrgModeTokenType::RightBracket) {
                    state.bump(); // ]]
                }
                else if state.at(OrgModeTokenType::LeftBracket) {
                    state.bump(); // ][
                    while state.not_at_end() && !state.at(OrgModeTokenType::RightBracket) && !state.at(OrgModeTokenType::Newline) {
                        state.bump();
                    }
                    if state.at(OrgModeTokenType::RightBracket) {
                        state.bump(); // ]
                        if state.at(OrgModeTokenType::RightBracket) {
                            state.bump(); // ]]
                        }
                    }
                }
            }
        }
        state.finish_at(checkpoint, crate::parser::element_type::OrgModeElementType::Link);
    }

    fn parse_inline_code<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let checkpoint = state.checkpoint();
        state.bump(); // ~
        while state.not_at_end() && !state.at(OrgModeTokenType::Tilde) && !state.at(OrgModeTokenType::Newline) {
            state.bump();
        }
        if state.at(OrgModeTokenType::Tilde) {
            state.bump();
        }
        state.finish_at(checkpoint, crate::parser::element_type::OrgModeElementType::InlineCode);
    }

    fn parse_verbatim<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let checkpoint = state.checkpoint();
        state.bump(); // =
        while state.not_at_end() && !state.at(OrgModeTokenType::Equal) && !state.at(OrgModeTokenType::Newline) {
            state.bump();
        }
        if state.at(OrgModeTokenType::Equal) {
            state.bump();
        }
        state.finish_at(checkpoint, crate::parser::element_type::OrgModeElementType::Verbatim);
    }

    fn parse_strikethrough<'b, S: Source + ?Sized>(&self, state: &mut State<'b, S>) {
        let checkpoint = state.checkpoint();
        state.bump(); // +
        while state.not_at_end() && !state.at(OrgModeTokenType::Plus) && !state.at(OrgModeTokenType::Newline) {
            state.bump();
        }
        if state.at(OrgModeTokenType::Plus) {
            state.bump();
        }
        state.finish_at(checkpoint, crate::parser::element_type::OrgModeElementType::Strikethrough);
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

            Ok(state.finish_at(checkpoint, crate::parser::element_type::OrgModeElementType::Document))
        })
    }
}
