use crate::lexer::{AsciidocLexer, State, token_type::AsciidocTokenType};
use oak_core::source::Source;

impl<'config> AsciidocLexer<'config> {
    /// Handles block delimiters.
    pub fn lex_block_delimiter<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        let mut count = 0;
        while state.peek() == Some('+') {
            state.advance(1);
            count += 1;
        }

        if count >= 2 {
            state.add_token(AsciidocTokenType::BlockDelimiter, start_pos, state.get_position());
            return true;
        }

        state.set_position(start_pos);
        false
    }

    /// Handles code blocks.
    pub fn lex_code_block<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if state.peek() == Some('`') {
            if state.source().get_char_at(start_pos + 1) == Some('`') && state.source().get_char_at(start_pos + 2) == Some('`') {
                state.advance(3);

                let lang_start = state.get_position();
                while state.not_at_end() {
                    if let Some(ch) = state.peek() {
                        if ch == '\n' || ch == '\r' {
                            break;
                        }
                        state.advance(ch.len_utf8());
                    }
                }

                if state.get_position() > lang_start {
                    state.add_token(AsciidocTokenType::CodeBlockLanguage, lang_start, state.get_position());
                }

                state.add_token(AsciidocTokenType::CodeBlock, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// Handles tables.
    pub fn lex_table<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if state.peek() == Some('|') {
            state.advance(1);
            state.add_token(AsciidocTokenType::Table, start_pos, state.get_position());

            let mut cell_start = state.get_position();
            while state.not_at_end() {
                if let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        if state.get_position() > cell_start {
                            state.add_token(AsciidocTokenType::TableCell, cell_start, state.get_position());
                        }
                        break;
                    }
                    else if ch == '|' {
                        if state.get_position() > cell_start {
                            state.add_token(AsciidocTokenType::TableCell, cell_start, state.get_position());
                        }
                        state.advance(1);
                        cell_start = state.get_position();
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }
                else {
                    break;
                }
            }
            return true;
        }

        false
    }

    /// Handles table separators.
    pub fn lex_table_separator<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if state.peek() == Some('|') {
            if state.source().get_char_at(start_pos + 1) == Some('-') {
                state.advance(1);
                let mut has_separator = false;

                while state.not_at_end() {
                    if let Some(ch) = state.peek() {
                        if ch == '-' {
                            state.advance(1);
                            has_separator = true;
                        }
                        else if ch == '|' {
                            state.advance(1);
                            if has_separator {
                                state.add_token(AsciidocTokenType::TableSeparator, start_pos, state.get_position());
                                return true;
                            }
                            break;
                        }
                        else if ch == '\n' || ch == '\r' {
                            break;
                        }
                        else {
                            break;
                        }
                    }
                    else {
                        break;
                    }
                }
            }
        }
        false
    }
}
