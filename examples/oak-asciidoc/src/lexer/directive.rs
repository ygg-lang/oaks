use crate::lexer::{AsciidocLexer, State, token_type::AsciidocTokenType};
use oak_core::source::Source;

impl<'config> AsciidocLexer<'config> {
    /// Handles document attributes.
    pub fn lex_document_attribute<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if state.peek() == Some(':') {
            state.advance(1);

            let mut _attr_name = String::new();
            while state.not_at_end() {
                if let Some(ch) = state.peek() {
                    if ch == ':' {
                        state.advance(1);
                        if let Some(next_ch) = state.peek() {
                            if next_ch == ' ' || next_ch == '\t' {
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
                                if self.config.allow_attributes {
                                    state.add_token(AsciidocTokenType::Attribute, start_pos, state.get_position());
                                }
                                return true;
                            }
                        }
                        break;
                    }
                    else if ch == '\n' || ch == '\r' {
                        break;
                    }
                    _attr_name.push(ch);
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }
            state.set_position(start_pos);
        }
        false
    }

    /// Handles macros.
    pub fn lex_macro<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if state.peek() == Some('{') {
            state.advance(1);

            let mut _macro_name = String::new();
            while state.not_at_end() {
                if let Some(ch) = state.peek() {
                    if ch == ':' {
                        state.advance(1);
                        while state.not_at_end() {
                            if let Some(ch) = state.peek() {
                                if ch == '}' {
                                    state.advance(1);
                                    if self.config.allow_macros {
                                        state.add_token(AsciidocTokenType::Macro, start_pos, state.get_position());
                                    }
                                    return true;
                                }
                                state.advance(ch.len_utf8());
                            }
                            else {
                                break;
                            }
                        }
                        break;
                    }
                    else if ch == '}' {
                        state.advance(1);
                        if self.config.allow_macros {
                            state.add_token(AsciidocTokenType::Macro, start_pos, state.get_position());
                        }
                        return true;
                    }
                    else if ch == '\n' || ch == '\r' {
                        break;
                    }
                    _macro_name.push(ch);
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }
            state.set_position(start_pos);
        }
        false
    }

    /// Handles include directives.
    pub fn lex_include_directive<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if state.peek() == Some('i') {
            if state.source().get_char_at(start_pos + 1) == Some('n')
                && state.source().get_char_at(start_pos + 2) == Some('c')
                && state.source().get_char_at(start_pos + 3) == Some('l')
                && state.source().get_char_at(start_pos + 4) == Some('u')
                && state.source().get_char_at(start_pos + 5) == Some('d')
                && state.source().get_char_at(start_pos + 6) == Some('e')
                && state.source().get_char_at(start_pos + 7) == Some(':')
                && state.source().get_char_at(start_pos + 8) == Some(':')
            {
                state.advance(9);

                while state.not_at_end() {
                    if let Some(ch) = state.peek() {
                        if ch == '[' {
                            state.advance(1);
                            while state.not_at_end() {
                                if let Some(ch) = state.peek() {
                                    if ch == ']' {
                                        state.advance(1);
                                        state.add_token(AsciidocTokenType::Include, start_pos, state.get_position());
                                        return true;
                                    }
                                    state.advance(ch.len_utf8());
                                }
                                else {
                                    break;
                                }
                            }
                            break;
                        }
                        else if ch == '\n' || ch == '\r' {
                            state.add_token(AsciidocTokenType::Include, start_pos, state.get_position());
                            return true;
                        }
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                state.add_token(AsciidocTokenType::Include, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// Handles conditional directives (ifdef, ifndef, endif).
    pub fn lex_conditional_directive<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if state.peek() == Some('i') {
            if state.source().get_char_at(start_pos + 1) == Some('f')
                && state.source().get_char_at(start_pos + 2) == Some('d')
                && state.source().get_char_at(start_pos + 3) == Some('e')
                && state.source().get_char_at(start_pos + 4) == Some('f')
                && (state.source().get_char_at(start_pos + 5) == Some(' ') || state.source().get_char_at(start_pos + 5) == Some('\t') || state.source().get_char_at(start_pos + 5) == Some(':'))
            {
                state.advance(5);

                while state.not_at_end() {
                    if let Some(ch) = state.peek() {
                        if ch == '\n' || ch == '\r' {
                            state.add_token(AsciidocTokenType::Ifdef, start_pos, state.get_position());
                            return true;
                        }
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                state.add_token(AsciidocTokenType::Ifdef, start_pos, state.get_position());
                return true;
            }
            else if state.source().get_char_at(start_pos + 1) == Some('f')
                && state.source().get_char_at(start_pos + 2) == Some('n')
                && state.source().get_char_at(start_pos + 3) == Some('d')
                && state.source().get_char_at(start_pos + 4) == Some('e')
                && state.source().get_char_at(start_pos + 5) == Some('f')
                && (state.source().get_char_at(start_pos + 6) == Some(' ') || state.source().get_char_at(start_pos + 6) == Some('\t') || state.source().get_char_at(start_pos + 6) == Some(':'))
            {
                state.advance(6);

                while state.not_at_end() {
                    if let Some(ch) = state.peek() {
                        if ch == '\n' || ch == '\r' {
                            state.add_token(AsciidocTokenType::Ifndef, start_pos, state.get_position());
                            return true;
                        }
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                state.add_token(AsciidocTokenType::Ifndef, start_pos, state.get_position());
                return true;
            }
        }
        else if state.peek() == Some('e') {
            if state.source().get_char_at(start_pos + 1) == Some('n') && state.source().get_char_at(start_pos + 2) == Some('d') && state.source().get_char_at(start_pos + 3) == Some('i') && state.source().get_char_at(start_pos + 4) == Some('f') {
                state.advance(5);

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

                state.add_token(AsciidocTokenType::Endif, start_pos, state.get_position());
                return true;
            }
        }
        false
    }
}
