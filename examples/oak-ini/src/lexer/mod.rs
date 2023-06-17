use crate::{language::IniLanguage, syntax::IniSyntaxKind};
use alloc::string::String;
use oak_core::{
    Lexer, SourceText,
    lexer::{LexOutput, LexerState},
};

type State<'input> = LexerState<'input, IniLanguage>;

/// INI lexer implementation
pub struct IniLexer<'config> {
    config: &'config IniLanguage,
}

impl<'config> IniLexer<'config> {
    /// Create a new INI lexer
    pub fn new(config: &'config IniLanguage) -> Self {
        Self { config }
    }

    /// 跳过空白字符（不包括换行符）
    fn skip_whitespace(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' || ch == '\r' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        if state.get_position() > start_pos {
            state.add_token(IniSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(IniSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Lex comment
    fn lex_comment(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch == ';' || ch == '#' {
                // Skip comment character
                state.advance(1);

                // Read until end of line
                while let Some(ch) = state.peek() {
                    if ch != '\n' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                state.add_token(IniSyntaxKind::Comment, start_pos, state.get_position());
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

    /// Lex string literal
    fn lex_string(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(quote_char) = state.peek() {
            if quote_char == '"' || quote_char == '\'' {
                // Skip opening quote
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch != quote_char {
                        if ch == '\\' {
                            state.advance(1); // escape character
                            if state.peek().is_some() {
                                state.advance(1); // escaped character
                            }
                        }
                        else {
                            state.advance(ch.len_utf8());
                        }
                    }
                    else {
                        break;
                    }
                }

                if let Some(ch) = state.peek() {
                    if ch == quote_char {
                        state.advance(1); // closing quote
                    }
                }

                state.add_token(IniSyntaxKind::String, start_pos, state.get_position());
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

    /// Lex number (integer or float)
    fn lex_number(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();
        let mut is_float = false;

        // Handle optional sign
        if let Some(ch) = state.peek() {
            if ch == '+' || ch == '-' {
                state.advance(1);
            }
        }

        // Read digits
        let mut has_digits = false;
        while let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);
                has_digits = true;
            }
            else {
                break;
            }
        }

        if !has_digits {
            return false;
        }

        // Check for decimal point
        if let Some('.') = state.peek() {
            // Look ahead to see if there's a digit after the decimal point
            let current_pos = state.get_position();
            state.advance(1);
            if let Some(ch) = state.peek() {
                if ch.is_ascii_digit() {
                    is_float = true;

                    // Read fractional part
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }
                else {
                    // Not a decimal number, backtrack
                    state.set_position(current_pos);
                }
            }
            else {
                // Not a decimal number, backtrack
                state.set_position(current_pos);
            }
        }

        // Check for exponent
        if let Some(ch) = state.peek() {
            if ch == 'e' || ch == 'E' {
                let current_pos = state.get_position();
                state.advance(1);

                // Optional sign in exponent
                if let Some(ch) = state.peek() {
                    if ch == '+' || ch == '-' {
                        state.advance(1);
                    }
                }

                // Read exponent digits
                let mut has_exp_digits = false;
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                        has_exp_digits = true;
                    }
                    else {
                        break;
                    }
                }

                if has_exp_digits {
                    is_float = true;
                }
                else {
                    // Invalid exponent, backtrack
                    state.set_position(current_pos);
                }
            }
        }

        let kind = if is_float { IniSyntaxKind::Float } else { IniSyntaxKind::Integer };

        state.add_token(kind, start_pos, state.get_position());
        true
    }

    /// Lex identifier or keyword
    fn lex_identifier(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                let mut text = String::new();

                // Read identifier characters
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                        text.push(ch);
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let kind = match text.as_str() {
                    "true" | "false" => IniSyntaxKind::Boolean,
                    _ if self.is_datetime_like(&text) => IniSyntaxKind::DateTime,
                    _ => IniSyntaxKind::Identifier,
                };

                state.add_token(kind, start_pos, state.get_position());
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

    /// 处理标点符号
    fn lex_punctuation(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '[' => IniSyntaxKind::LeftBracket,
                ']' => IniSyntaxKind::RightBracket,
                ',' => IniSyntaxKind::Comma,
                '.' => IniSyntaxKind::Dot,
                '=' => IniSyntaxKind::Equal,
                _ => return false,
            };

            state.advance(1);
            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Check if text looks like a datetime
    fn is_datetime_like(&self, text: &str) -> bool {
        // Simple heuristic for datetime detection
        text.len() >= 10
            && text.chars().nth(4) == Some('-')
            && text.chars().nth(7) == Some('-')
            && text.chars().take(4).all(|c| c.is_ascii_digit())
            && text.chars().skip(5).take(2).all(|c| c.is_ascii_digit())
            && text.chars().skip(8).take(2).all(|c| c.is_ascii_digit())
    }
}

impl<'config> Lexer<IniLanguage> for IniLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<IniSyntaxKind> {
        let mut state = State::new(source);

        loop {
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_comment(&mut state) {
                continue;
            }

            if self.lex_string(&mut state) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_identifier(&mut state) {
                continue;
            }

            if self.lex_punctuation(&mut state) {
                continue;
            }

            // 如果没有匹配任何规则，标记为错误并前进一个字符
            if let Some(ch) = state.peek() {
                let start_pos = state.get_position();
                state.advance(ch.len_utf8());
                state.add_token(IniSyntaxKind::Error, start_pos, state.get_position());
            }
            else {
                break;
            }
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(IniSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
