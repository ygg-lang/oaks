use crate::{kind::DHallSyntaxKind, language::DHallLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

pub struct DHallLexer<'config> {
    config: &'config DHallLanguage,
}

impl<'config> DHallLexer<'config> {
    pub fn new(config: &'config DHallLanguage) -> Self {
        Self { config }
    }

    /// Skip whitespace characters
    fn skip_whitespace(&self, state: &mut LexerState<DHallLanguage>) -> bool {
        let start_pos = state.get_position();
        let mut found_whitespace = false;

        while let Some(ch) = state.peek() {
            if ch.is_whitespace() && ch != '\n' && ch != '\r' {
                state.advance(ch.len_utf8());
                found_whitespace = true;
            }
            else {
                break;
            }
        }

        if found_whitespace {
            state.add_token(DHallSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handle newlines
    fn lex_newline(&self, state: &mut LexerState<DHallLanguage>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(DHallSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(DHallSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handle comments
    fn lex_comment(&self, state: &mut LexerState<DHallLanguage>) -> bool {
        let start_pos = state.get_position();

        if let Some('%') = state.peek() {
            state.advance(1);

            // Read to end of line
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(DHallSyntaxKind::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handle identifiers, atoms or keywords
    fn lex_identifier_atom_or_keyword(&self, state: &mut LexerState<DHallLanguage>, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(first_char) = state.peek() {
            if first_char.is_ascii_lowercase() || first_char == '_' {
                // Atom or keyword
                state.advance(first_char.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '@' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = source.get_text_in((start_pos..state.get_position()).into()).unwrap_or("");
                let kind = match text {
                    "after" => DHallSyntaxKind::After,
                    "and" => DHallSyntaxKind::And,
                    "andalso" => DHallSyntaxKind::Andalso,
                    "band" => DHallSyntaxKind::Band,
                    "begin" => DHallSyntaxKind::Begin,
                    "bnot" => DHallSyntaxKind::Bnot,
                    "bor" => DHallSyntaxKind::Bor,
                    "bsl" => DHallSyntaxKind::Bsl,
                    "bsr" => DHallSyntaxKind::Bsr,
                    "bxor" => DHallSyntaxKind::Bxor,
                    "case" => DHallSyntaxKind::Case,
                    "catch" => DHallSyntaxKind::Catch,
                    "cond" => DHallSyntaxKind::Cond,
                    "div" => DHallSyntaxKind::Div,
                    "end" => DHallSyntaxKind::End,
                    "fun" => DHallSyntaxKind::Fun,
                    "if" => DHallSyntaxKind::If,
                    "let" => DHallSyntaxKind::Let,
                    "not" => DHallSyntaxKind::Not,
                    "of" => DHallSyntaxKind::Of,
                    "or" => DHallSyntaxKind::Or,
                    "orelse" => DHallSyntaxKind::Orelse,
                    "query" => DHallSyntaxKind::Query,
                    "receive" => DHallSyntaxKind::Receive,
                    "rem" => DHallSyntaxKind::Rem,
                    "try" => DHallSyntaxKind::Try,
                    "when" => DHallSyntaxKind::When,
                    "xor" => DHallSyntaxKind::Xor,
                    _ => DHallSyntaxKind::Atom,
                };

                state.add_token(kind, start_pos, state.get_position());
                true
            }
            else if first_char.is_ascii_uppercase() || first_char == '_' {
                // Variable
                state.advance(first_char.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                state.add_token(DHallSyntaxKind::Variable, start_pos, state.get_position());
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    /// Handle numbers
    fn lex_number(&self, state: &mut LexerState<DHallLanguage>) -> bool {
        let start_pos = state.get_position();

        if let Some(first_char) = state.peek() {
            if first_char.is_ascii_digit() {
                state.advance(1);

                // Read more digits
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else if ch == '.' {
                        // Float
                        state.advance(1);
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                        break;
                    }
                    else if ch == 'e' || ch == 'E' {
                        // Scientific notation
                        state.advance(1);
                        if let Some(sign) = state.peek() {
                            if sign == '+' || sign == '-' {
                                state.advance(1);
                            }
                        }
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                        break;
                    }
                    else {
                        break;
                    }
                }

                state.add_token(DHallSyntaxKind::Number, start_pos, state.get_position());
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    /// Handle strings
    fn lex_string(&self, state: &mut LexerState<DHallLanguage>) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1);
                    state.add_token(DHallSyntaxKind::String, start_pos, state.get_position());
                    return true;
                }
                else if ch == '\\' {
                    state.advance(1);
                    if let Some(_) = state.peek() {
                        state.advance(1);
                    }
                }
                else if ch == '\n' || ch == '\r' {
                    // Unclosed string
                    state.add_token(DHallSyntaxKind::Error, start_pos, state.get_position());
                    return true;
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            // EOF but string not closed
            state.add_token(DHallSyntaxKind::Error, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handle characters
    fn lex_character(&self, state: &mut LexerState<DHallLanguage>) -> bool {
        let start_pos = state.get_position();

        if let Some('$') = state.peek() {
            state.advance(1);
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(DHallSyntaxKind::Character, start_pos, state.get_position());
                true
            }
            else {
                state.add_token(DHallSyntaxKind::Error, start_pos, state.get_position());
                true
            }
        }
        else {
            false
        }
    }

    /// Handle operators
    fn lex_operator(&self, state: &mut LexerState<DHallLanguage>) -> bool {
        let start_pos = state.get_position();

        if let Some(first_char) = state.peek() {
            match first_char {
                '+' => {
                    state.advance(1);
                    if let Some('+') = state.peek() {
                        state.advance(1);
                        state.add_token(DHallSyntaxKind::PlusPlus, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(DHallSyntaxKind::Plus, start_pos, state.get_position());
                    }
                    true
                }
                '-' => {
                    state.advance(1);
                    if let Some('-') = state.peek() {
                        state.advance(1);
                        state.add_token(DHallSyntaxKind::MinusMinus, start_pos, state.get_position());
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        state.add_token(DHallSyntaxKind::Arrow, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(DHallSyntaxKind::Minus, start_pos, state.get_position());
                    }
                    true
                }
                '*' => {
                    state.advance(1);
                    state.add_token(DHallSyntaxKind::Star, start_pos, state.get_position());
                    true
                }
                '/' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(DHallSyntaxKind::SlashEqual, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(DHallSyntaxKind::Slash, start_pos, state.get_position());
                    }
                    true
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(DHallSyntaxKind::EqualEqual, start_pos, state.get_position());
                    }
                    else if let Some(':') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            state.add_token(DHallSyntaxKind::EqualColonEqual, start_pos, state.get_position());
                        }
                        else {
                            state.add_token(DHallSyntaxKind::Equal, start_pos, state.get_position());
                        }
                    }
                    else if let Some('<') = state.peek() {
                        state.advance(1);
                        state.add_token(DHallSyntaxKind::LessEqual, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(DHallSyntaxKind::Equal, start_pos, state.get_position());
                    }
                    true
                }
                '<' => {
                    state.advance(1);
                    if let Some('-') = state.peek() {
                        state.advance(1);
                        state.add_token(DHallSyntaxKind::Arrow, start_pos, state.get_position());
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(DHallSyntaxKind::LessEqual, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(DHallSyntaxKind::Less, start_pos, state.get_position());
                    }
                    true
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        state.add_token(DHallSyntaxKind::GreaterEqual, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(DHallSyntaxKind::Greater, start_pos, state.get_position());
                    }
                    true
                }
                '!' => {
                    state.advance(1);
                    state.add_token(DHallSyntaxKind::Exclamation, start_pos, state.get_position());
                    true
                }
                '?' => {
                    state.advance(1);
                    state.add_token(DHallSyntaxKind::Question, start_pos, state.get_position());
                    true
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.peek() {
                        state.advance(1);
                        state.add_token(DHallSyntaxKind::PipePipe, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(DHallSyntaxKind::Pipe, start_pos, state.get_position());
                    }
                    true
                }
                '#' => {
                    state.advance(1);
                    state.add_token(DHallSyntaxKind::Hash, start_pos, state.get_position());
                    true
                }
                _ => false,
            }
        }
        else {
            false
        }
    }

    /// Handle delimiters
    fn lex_delimiter(&self, state: &mut LexerState<DHallLanguage>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '(' => DHallSyntaxKind::LeftParen,
                ')' => DHallSyntaxKind::RightParen,
                '{' => DHallSyntaxKind::LeftBrace,
                '}' => DHallSyntaxKind::RightBrace,
                '[' => DHallSyntaxKind::LeftBracket,
                ']' => DHallSyntaxKind::RightBracket,
                ',' => DHallSyntaxKind::Comma,
                ';' => DHallSyntaxKind::Semicolon,
                '.' => DHallSyntaxKind::Dot,
                ':' => DHallSyntaxKind::Colon,
                _ => return false,
            };

            state.advance(ch.len_utf8());
            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<DHallLanguage> for DHallLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<DHallSyntaxKind> {
        let mut state = LexerState::new(source);

        while state.not_at_end() {
            // Try various lexical rules
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_comment(&mut state) {
                continue;
            }

            if self.lex_identifier_atom_or_keyword(&mut state, source) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_string(&mut state) {
                continue;
            }

            if self.lex_character(&mut state) {
                continue;
            }

            if self.lex_operator(&mut state) {
                continue;
            }

            if self.lex_delimiter(&mut state) {
                continue;
            }

            // If no rules match, skip current character and mark as error
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(DHallSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // Add EOF token
        let eof_pos = state.get_position();
        state.add_token(DHallSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
