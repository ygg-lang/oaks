#![doc = include_str!("readme.md")]
/// Token types for Lua.
pub mod token_type;

/// Lua lexer implementation.
///
/// Implements lexical analysis for the Lua language, converting source code into a sequence of tokens.
use crate::language::LuaLanguage;
pub use crate::lexer::token_type::LuaTokenType;
use oak_core::{Lexer, LexerCache, LexerState, OakError, lexer::LexOutput, source::Source};

pub(crate) type State<'a, S> = LexerState<'a, S, LuaLanguage>;

/// Lua lexer.
#[derive(Clone)]
pub struct LuaLexer<'config> {
    config: &'config LuaLanguage,
}

impl<'config> LuaLexer<'config> {
    /// Creates a new Lua lexer.
    pub fn new(config: &'config LuaLanguage) -> Self {
        Self { config }
    }

    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            // Try various lexical rules
            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_newline(state) {
                continue;
            }

            if self.lex_comment(state) {
                continue;
            }

            if self.lex_string(state) {
                continue;
            }

            if self.lex_number(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_operator_or_delimiter(state) {
                continue;
            }

            // If all rules do not match, skip the current character and mark as error
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(LuaTokenType::Error, start_pos, state.get_position())
            }

            state.advance_if_dead_lock(safe_point)
        }

        Ok(())
    }

    /// Skips whitespace characters.
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' { state.advance(ch.len_utf8()) } else { break }
        }

        if state.get_position() > start_pos {
            state.add_token(LuaTokenType::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles newline characters.
    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(LuaTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1)
            }
            state.add_token(LuaTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles comments.
    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('-') = state.current() {
            if let Some('-') = state.peek() {
                state.advance(1); // First '-'
                state.advance(1); // Second '-'

                // Check if it's a long comment --[[
                if let Some('[') = state.current() {
                    if let Some('[') = state.peek() {
                        state.advance(1); // '['
                        state.advance(1); // '['

                        // Find ]]
                        while let Some(ch) = state.current() {
                            if ch == ']' {
                                if let Some(']') = state.peek() {
                                    state.advance(1); // ']'
                                    state.advance(1); // ']'
                                    break;
                                }
                            }
                            state.advance(ch.len_utf8())
                        }
                    }
                    else {
                        // Single-line comment, read until the end of the line
                        while let Some(ch) = state.current() {
                            if ch == '\n' || ch == '\r' {
                                break;
                            }
                            state.advance(ch.len_utf8())
                        }
                    }
                }
                else {
                    // Single-line comment, read until the end of the line
                    while let Some(ch) = state.current() {
                        if ch == '\n' || ch == '\r' {
                            break;
                        }
                        state.advance(ch.len_utf8())
                    }
                }

                state.add_token(LuaTokenType::Comment, start_pos, state.get_position());
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

    /// Handles string literals.
    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(quote_char) = state.current() {
            if quote_char == '"' || quote_char == '\'' {
                state.advance(1); // Skip start quote

                let mut escaped = false;
                while let Some(ch) = state.current() {
                    if escaped {
                        escaped = false;
                        state.advance(ch.len_utf8())
                    }
                    else if ch == '\\' {
                        escaped = true;
                        state.advance(1)
                    }
                    else if ch == quote_char {
                        state.advance(1); // Skip end quote
                        break;
                    }
                    else if ch == '\n' || ch == '\r' {
                        // Strings cannot span lines unless escaped
                        break;
                    }
                    else {
                        state.advance(ch.len_utf8())
                    }
                }

                state.add_token(LuaTokenType::String, start_pos, state.get_position());
                true
            }
            else if quote_char == '[' {
                // Long string [[...]]
                if let Some('[') = state.peek() {
                    state.advance(1); // '['
                    state.advance(1); // '['

                    // Find ]]
                    while let Some(ch) = state.current() {
                        if ch == ']' {
                            if let Some(']') = state.peek() {
                                state.advance(1); // ']'
                                state.advance(1); // ']'
                                break;
                            }
                        }
                        state.advance(ch.len_utf8())
                    }

                    state.add_token(LuaTokenType::String, start_pos, state.get_position());
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
        else {
            false
        }
    }

    /// Handles numbers.
    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.current() {
            if ch.is_ascii_digit() {
                // Check if it's hexadecimal
                if ch == '0' {
                    if let Some(next_ch) = state.peek() {
                        if next_ch == 'x' || next_ch == 'X' {
                            state.advance(1); // '0'
                            state.advance(1); // 'x' 'X'

                            // Read hexadecimal digits
                            while let Some(hex_ch) = state.current() {
                                if hex_ch.is_ascii_hexdigit() { state.advance(1) } else { break }
                            }

                            state.add_token(LuaTokenType::Number, start_pos, state.get_position());
                            return true;
                        }
                    }
                }

                // Normal number
                let mut has_dot = false;
                let mut has_exp = false;

                while let Some(num_ch) = state.current() {
                    if num_ch.is_ascii_digit() {
                        state.advance(1)
                    }
                    else if num_ch == '.' && !has_dot && !has_exp {
                        has_dot = true;
                        state.advance(1)
                    }
                    else if (num_ch == 'e' || num_ch == 'E') && !has_exp {
                        has_exp = true;
                        state.advance(1);

                        // Optional sign
                        if let Some(sign_ch) = state.current() {
                            if sign_ch == '+' || sign_ch == '-' {
                                state.advance(1)
                            }
                        }
                    }
                    else {
                        break;
                    }
                }

                state.add_token(LuaTokenType::Number, start_pos, state.get_position());
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

    /// Handles identifiers or keywords.
    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some(ch) = state.current() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                let range = state.take_while(|c| c.is_ascii_alphanumeric() || c == '_');
                // Use the get_text_in method of the Source trait
                let text = state.get_text_in(range.clone().into());
                let token_kind = self.keyword_or_identifier(&text);
                state.add_token(token_kind, range.start, range.end);
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

    /// Recognizes keywords.
    fn keyword_or_identifier(&self, text: &str) -> LuaTokenType {
        match text {
            "and" => LuaTokenType::And,
            "break" => LuaTokenType::Break,
            "do" => LuaTokenType::Do,
            "else" => LuaTokenType::Else,
            "elseif" => LuaTokenType::Elseif,
            "end" => LuaTokenType::End,
            "false" => LuaTokenType::False,
            "for" => LuaTokenType::For,
            "function" => LuaTokenType::Function,
            "goto" => LuaTokenType::Goto,
            "if" => LuaTokenType::If,
            "in" => LuaTokenType::In,
            "local" => LuaTokenType::Local,
            "nil" => LuaTokenType::Nil,
            "not" => LuaTokenType::Not,
            "or" => LuaTokenType::Or,
            "repeat" => LuaTokenType::Repeat,
            "return" => LuaTokenType::Return,
            "then" => LuaTokenType::Then,
            "true" => LuaTokenType::True,
            "until" => LuaTokenType::Until,
            "while" => LuaTokenType::While,
            _ => LuaTokenType::Identifier,
        }
    }

    /// Handles operators and delimiters.
    fn lex_operator_or_delimiter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        LuaTokenType::EqEq
                    }
                    else {
                        LuaTokenType::Eq
                    }
                }
                '~' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        LuaTokenType::TildeEq
                    }
                    else {
                        LuaTokenType::Tilde
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        LuaTokenType::LtEq
                    }
                    else if let Some('<') = state.peek() {
                        state.advance(1);
                        LuaTokenType::LtLt
                    }
                    else {
                        LuaTokenType::Lt
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        LuaTokenType::GtEq
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        LuaTokenType::GtGt
                    }
                    else {
                        LuaTokenType::Gt
                    }
                }
                '.' => {
                    state.advance(1);
                    if let Some('.') = state.peek() {
                        state.advance(1);
                        if let Some('.') = state.peek() {
                            state.advance(1);
                            LuaTokenType::DotDotDot
                        }
                        else {
                            LuaTokenType::DotDot
                        }
                    }
                    else {
                        LuaTokenType::Dot
                    }
                }
                ':' => {
                    state.advance(1);
                    if let Some(':') = state.peek() {
                        state.advance(1);
                        LuaTokenType::ColonColon
                    }
                    else {
                        LuaTokenType::Colon
                    }
                }
                '/' => {
                    state.advance(1);
                    if let Some('/') = state.peek() {
                        state.advance(1);
                        LuaTokenType::SlashSlash
                    }
                    else {
                        LuaTokenType::Slash
                    }
                }
                '+' => {
                    state.advance(1);
                    LuaTokenType::Plus
                }
                '-' => {
                    state.advance(1);
                    LuaTokenType::Minus
                }
                '*' => {
                    state.advance(1);
                    LuaTokenType::Star
                }
                '%' => {
                    state.advance(1);
                    LuaTokenType::Percent
                }
                '^' => {
                    state.advance(1);
                    LuaTokenType::Caret
                }
                '#' => {
                    state.advance(1);
                    LuaTokenType::Hash
                }
                '&' => {
                    state.advance(1);
                    LuaTokenType::Ampersand
                }
                '|' => {
                    state.advance(1);
                    LuaTokenType::Pipe
                }
                '(' => {
                    state.advance(1);
                    LuaTokenType::LeftParen
                }
                ')' => {
                    state.advance(1);
                    LuaTokenType::RightParen
                }
                '{' => {
                    state.advance(1);
                    LuaTokenType::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    LuaTokenType::RightBrace
                }
                '[' => {
                    state.advance(1);
                    LuaTokenType::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    LuaTokenType::RightBracket
                }
                ';' => {
                    state.advance(1);
                    LuaTokenType::Semicolon
                }
                ',' => {
                    state.advance(1);
                    LuaTokenType::Comma
                }
                _ => return false,
            };

            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<LuaLanguage> for LuaLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[oak_core::TextEdit], cache: &'a mut impl LexerCache<LuaLanguage>) -> LexOutput<LuaLanguage> {
        let mut state = State::new_with_cache(source, 0, cache);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}
