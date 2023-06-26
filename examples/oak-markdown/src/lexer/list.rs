use crate::lexer::{MarkdownLexer, State, token_type::MarkdownTokenType};
use oak_core::Source;

impl<'config> MarkdownLexer<'config> {
    /// Handles list markers.
    pub fn lex_list_marker<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        let mut check_pos = start_pos;
        while check_pos > 0 {
            check_pos -= 1;
            if let Some(ch) = state.source().get_char_at(check_pos) {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                else if ch != ' ' && ch != '\t' {
                    return false;
                }
            }
        }

        if let Some(ch) = state.peek() {
            match ch {
                '-' | '*' | '+' => {
                    state.advance(1);
                    if let Some(next_ch) = state.peek() {
                        if next_ch == ' ' || next_ch == '\t' {
                            state.add_token(MarkdownTokenType::ListMarker, start_pos, state.get_position());
                            return true;
                        }
                    }
                    state.set_position(start_pos);
                    false
                }
                '0'..='9' => {
                    while let Some(digit) = state.peek() {
                        if digit.is_ascii_digit() { state.advance(1) } else { break }
                    }

                    if let Some('.') = state.peek() {
                        state.advance(1);
                        if let Some(next_ch) = state.peek() {
                            if next_ch == ' ' || next_ch == '\t' {
                                state.add_token(MarkdownTokenType::ListMarker, start_pos, state.get_position());
                                return true;
                            }
                        }
                    }

                    state.set_position(start_pos);
                    false
                }
                _ => false,
            }
        }
        else {
            false
        }
    }

    /// Handles task markers.
    pub fn lex_task_marker<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('[') = state.peek() {
            state.advance(1);
            if let Some(ch) = state.peek() {
                if ch == ' ' || ch == 'x' || ch == 'X' {
                    state.advance(1);
                    if let Some(']') = state.peek() {
                        state.advance(1);
                        state.add_token(MarkdownTokenType::TaskMarker, start_pos, state.get_position());
                        return true;
                    }
                }
            }
            state.set_position(start_pos);
        }
        false
    }

    /// Lexes definition list descriptions.
    pub fn lex_definition_description<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        let mut check_pos = start_pos;
        while check_pos > 0 {
            check_pos -= 1;
            if let Some(ch) = state.source().get_char_at(check_pos) {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                else if ch != ' ' && ch != '\t' {
                    return false;
                }
            }
        }

        if let Some(':') = state.peek() {
            state.advance(1);
            if let Some(next_ch) = state.peek() {
                if next_ch == ' ' || next_ch == '\t' {
                    state.add_token(MarkdownTokenType::DefinitionDescription, start_pos, state.get_position());
                    return true;
                }
            }
            state.set_position(start_pos);
            false
        }
        else {
            false
        }
    }

    /// Lexes abbreviation definitions.
    pub fn lex_abbreviation<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        let mut check_pos = start_pos;
        while check_pos > 0 {
            check_pos -= 1;
            if let Some(ch) = state.source().get_char_at(check_pos) {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                else if ch != ' ' && ch != '\t' {
                    return false;
                }
            }
        }

        if let Some('*') = state.peek() {
            state.advance(1);
            if let Some('[') = state.peek() {
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch == ']' {
                        state.advance(1);
                        if let Some(':') = state.peek() {
                            state.advance(1);
                            state.add_token(MarkdownTokenType::Abbreviation, start_pos, state.get_position());
                            return true;
                        }
                        break;
                    }
                    state.advance(ch.len_utf8());
                }
            }
            state.set_position(start_pos);
        }
        false
    }
}
