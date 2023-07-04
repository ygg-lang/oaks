use oak_core::{
    Lexer, LexerState, OakError, Source, TextEdit,
    lexer::{LexOutput, LexerCache},
};

/// Token types for the VOC language.
pub mod token_type;

use crate::language::VocLanguage;
pub use token_type::VocTokenType;

pub(crate) type State<'a, S> = LexerState<'a, S, VocLanguage>;

/// A lexer for the VOC language.
#[derive(Clone, Debug)]
pub struct VocLexer<'config> {
    /// Language configuration reference.
    #[allow(dead_code)]
    config: &'config VocLanguage,
}

impl<'config> VocLexer<'config> {
    /// Creates a new `VocLexer` with the given configuration.
    pub fn new(config: &'config VocLanguage) -> Self {
        Self { config }
    }

    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch.is_whitespace() {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        if state.get_position() > start_pos {
            state.add_token(VocTokenType::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn is_section_tag<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let mut offset = 1;
        if state.peek_next_n(offset) == Some('/') {
            offset += 1;
        }

        for name in &["template", "script", "style"] {
            let mut matches = true;
            for (i, c) in name.chars().enumerate() {
                if state.peek_next_n(offset + i) != Some(c) {
                    matches = false;
                    break;
                }
            }
            if matches && state.peek_next_n(offset + name.len()) == Some('>') {
                return true;
            }
        }
        false
    }

    fn scan_section<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        state.advance(1);

        let is_close = state.peek() == Some('/');
        if is_close {
            state.advance(1);
        }

        while let Some(c) = state.peek() {
            if c == '>' {
                state.advance(1);
                let kind = if is_close { VocTokenType::SectionClose } else { VocTokenType::SectionOpen };
                state.add_token(kind, start, state.get_position());
                return true;
            }
            state.advance(c.len_utf8());
        }

        state.add_error(OakError::unexpected_eof(state.get_position(), None));
        let kind = if is_close { VocTokenType::SectionClose } else { VocTokenType::SectionOpen };
        state.add_token(kind, start, state.get_position());
        true
    }

    fn scan_tag<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        state.advance(1);

        let is_close = state.peek() == Some('/');
        if is_close {
            state.advance(1);
        }

        while let Some(c) = state.peek() {
            if c == '>' {
                state.advance(1);
                let kind = if is_close { VocTokenType::TagClose } else { VocTokenType::TagOpen };
                state.add_token(kind, start, state.get_position());
                return true;
            }
            if !is_close && c == '/' && state.peek_next() == Some('>') {
                state.advance(1);
                state.advance(1);
                state.add_token(VocTokenType::SelfCloseTag, start, state.get_position());
                return true;
            }
            state.advance(c.len_utf8());
        }

        state.add_error(OakError::unexpected_eof(state.get_position(), None));
        let kind = if is_close { VocTokenType::TagClose } else { VocTokenType::TagOpen };
        state.add_token(kind, start, state.get_position());
        true
    }

    #[allow(dead_code)]
    fn scan_attribute<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        while let Some(c) = state.peek() {
            if c == '=' {
                state.advance(1);
                break;
            }
            state.advance(c.len_utf8());
        }

        if state.peek() == Some('"') {
            state.advance(1);
            while let Some(c) = state.peek() {
                if c == '"' {
                    state.advance(1);
                    break;
                }
                state.advance(c.len_utf8());
            }
        }

        state.add_token(VocTokenType::Attribute, start, state.get_position());
        true
    }

    fn scan_text<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        while let Some(c) = state.peek() {
            if c == '<' {
                break;
            }
            state.advance(c.len_utf8());
        }

        if state.get_position() > start {
            state.add_token(VocTokenType::Text, start, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn scan_style_selector<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        while let Some(c) = state.peek() {
            if c == '{' {
                break;
            }
            state.advance(c.len_utf8());
        }

        state.add_token(VocTokenType::Selector, start, state.get_position());
        true
    }

    fn scan_style_property<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        while let Some(c) = state.peek() {
            if c == ':' {
                state.advance(1);
                break;
            }
            state.advance(c.len_utf8());
        }

        while let Some(c) = state.peek() {
            if c == ';' || c == '}' {
                break;
            }
            state.advance(c.len_utf8());
        }

        state.add_token(VocTokenType::Property, start, state.get_position());
        true
    }

    fn scan_variable<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        state.advance(1);

        while let Some(c) = state.peek() {
            if c == ':' {
                state.advance(1);
                break;
            }
            state.advance(c.len_utf8());
        }

        while let Some(c) = state.peek() {
            if c == ';' || c == '}' {
                break;
            }
            state.advance(c.len_utf8());
        }

        state.add_token(VocTokenType::Variable, start, state.get_position());
        true
    }

    fn is_style_property<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let rest = state.rest();
        for c in rest.chars() {
            if c == ':' {
                return true;
            }
            if !c.is_alphanumeric() && c != '-' {
                return false;
            }
        }
        false
    }
}

impl<'config> Lexer<VocLanguage> for VocLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[TextEdit], _cache: &'a mut impl LexerCache<VocLanguage>) -> LexOutput<VocLanguage> {
        let mut state = State::new(source);

        while state.not_at_end() {
            if self.skip_whitespace(&mut state) {
                continue;
            }

            match state.peek() {
                Some('<') => {
                    if self.is_section_tag(&mut state) {
                        self.scan_section(&mut state);
                    }
                    else {
                        self.scan_tag(&mut state);
                    }
                }
                Some('{') => {
                    let start = state.get_position();
                    state.advance(1);
                    state.add_token(VocTokenType::BlockOpen, start, state.get_position());
                }
                Some('}') => {
                    let start = state.get_position();
                    state.advance(1);
                    state.add_token(VocTokenType::BlockClose, start, state.get_position());
                }
                Some('.') | Some('#') => {
                    self.scan_style_selector(&mut state);
                }
                Some('$') => {
                    self.scan_variable(&mut state);
                }
                Some(c) if c.is_alphabetic() => {
                    if self.is_style_property(&mut state) {
                        self.scan_style_property(&mut state);
                    }
                    else {
                        self.scan_text(&mut state);
                    }
                }
                Some(_) => {
                    self.scan_text(&mut state);
                }
                None => break,
            }
        }

        state.add_eof();
        state.finish(Ok(()))
    }
}
