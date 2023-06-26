use crate::lexer::{AsciidocLexer, State, token_type::AsciidocTokenType};
use oak_core::source::Source;

impl<'config> AsciidocLexer<'config> {
    /// Handles emphasis.
    pub fn lex_emphasis<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if state.peek() == Some('*') {
            state.advance(1);
            if state.peek() == Some('*') {
                state.advance(1);
                state.add_token(AsciidocTokenType::Strong, start_pos, state.get_position());
                return true;
            }
            state.add_token(AsciidocTokenType::Emphasis, start_pos, state.get_position());
            return true;
        }

        false
    }

    /// Handles monospace text.
    pub fn lex_monospace<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if state.peek() == Some('`') {
            state.advance(1);
            let mut found_end = false;

            while state.not_at_end() {
                if let Some(ch) = state.peek() {
                    if ch == '`' {
                        state.advance(1);
                        found_end = true;
                        break;
                    }
                    else if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }

            if found_end {
                state.add_token(AsciidocTokenType::Monospace, start_pos, state.get_position());
                return true;
            }
            state.set_position(start_pos);
        }
        false
    }

    /// Handles footnotes.
    pub fn lex_footnote<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if state.peek() == Some('~') {
            if state.source().get_char_at(start_pos + 1) == Some('[') {
                state.advance(2);
                while state.not_at_end() {
                    if let Some(ch) = state.peek() {
                        if ch == ']' {
                            state.advance(1);
                            if self.config.allow_footnotes {
                                state.add_token(AsciidocTokenType::FootnoteReference, start_pos, state.get_position());
                                return true;
                            }
                            break;
                        }
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }
                state.set_position(start_pos);
            }
        }
        else if state.peek() == Some('[') {
            if let Some('^') = state.source().get_char_at(start_pos + 1) {
                state.advance(2);
                while state.not_at_end() {
                    if let Some(ch) = state.peek() {
                        if ch == ']' {
                            state.advance(1);
                            if let Some(':') = state.peek() {
                                state.advance(1);
                                while state.not_at_end() {
                                    if let Some(ch) = state.peek() {
                                        if ch == '\n' || ch == '\r' {
                                            break;
                                        }
                                        state.advance(ch.len_utf8());
                                    }
                                    else {
                                        break;
                                    }
                                }
                                if self.config.allow_footnotes {
                                    state.add_token(AsciidocTokenType::FootnoteDefinition, start_pos, state.get_position());
                                    return true;
                                }
                                break;
                            }
                            break;
                        }
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }
                state.set_position(start_pos);
            }
        }
        false
    }

    /// Handles cross-references.
    pub fn lex_cross_reference<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if state.peek() == Some('^') {
            if state.source().get_char_at(start_pos + 1) == Some('[') {
                state.advance(2);
                while state.not_at_end() {
                    if let Some(ch) = state.peek() {
                        if ch == ']' {
                            state.advance(1);
                            if self.config.allow_cross_references {
                                state.add_token(AsciidocTokenType::CrossReference, start_pos, state.get_position());
                                return true;
                            }
                            break;
                        }
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }
            }
            state.set_position(start_pos);
        }
        false
    }
}
