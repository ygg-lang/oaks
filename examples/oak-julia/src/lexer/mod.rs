#![doc = include_str!("readme.md")]
/// Token type definitions for the Julia language.
pub mod token_type;

use crate::{language::JuliaLanguage, lexer::token_type::JuliaTokenType};
use oak_core::{Lexer, LexerCache, LexerState, lexer::LexOutput, source::Source};

pub(crate) type State<'a, S> = LexerState<'a, S, JuliaLanguage>;

/// Lexer for the Julia language.
#[derive(Clone, Debug)]
pub struct JuliaLexer<'config> {
    config: &'config JuliaLanguage,
}

impl<'config> JuliaLexer<'config> {
    /// Creates a new instance of the Julia lexer.
    pub fn new(config: &'config JuliaLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Lexer<JuliaLanguage> for JuliaLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl LexerCache<JuliaLanguage>) -> LexOutput<JuliaLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}

impl JuliaLexer<'_> {
    /// Skips whitespace characters (spaces and tabs).
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' { state.advance(ch.len_utf8()) } else { break }
        }

        if state.get_position() > start_pos {
            state.add_token(JuliaTokenType::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Lexes a newline character (LF or CRLF).
    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(JuliaTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1)
            }
            state.add_token(JuliaTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Lexes identifiers and keywords.
    ///
    /// Identifiers in Julia start with an alphabetic character or underscore,
    /// followed by alphanumeric characters, underscores, `!`, or `?`.
    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '!' || ch == '?' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let end_pos = state.get_position();
                let identifier_str = state.get_text_in((start_pos..end_pos).into());

                // Check if it's a keyword
                if let Ok(keyword_kind) = identifier_str.as_ref().parse::<JuliaTokenType>() {
                    state.add_token(keyword_kind, start_pos, end_pos)
                }
                else {
                    state.add_token(JuliaTokenType::Identifier, start_pos, end_pos)
                }
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

    /// Lexes a number literal (integer or float).
    ///
    /// Supports decimal integers, floating-point numbers with scientific notation,
    /// and type suffixes.
    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);

                // Handle integer part
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() || ch == '_' { state.advance(1) } else { break }
                }

                let mut is_float = false;

                // Check for decimal point
                if let Some('.') = state.peek() {
                    // Check next char to avoid confusion with range operator
                    if let Some(next_ch) = state.peek_next_n(1) {
                        if next_ch.is_ascii_digit() {
                            is_float = true;
                            state.advance(1); // Skip dot
                            // Handle fractional part
                            while let Some(ch) = state.peek() {
                                if ch.is_ascii_digit() || ch == '_' {
                                    state.advance(1);
                                }
                                else {
                                    break;
                                }
                            }
                        }
                    }
                }

                // Check for scientific notation
                if let Some(ch) = state.peek() {
                    if ch == 'e' || ch == 'E' {
                        is_float = true;
                        state.advance(1);

                        // Optional sign
                        if let Some(sign) = state.peek() {
                            if sign == '+' || sign == '-' {
                                state.advance(1)
                            }
                        }

                        // Exponent part
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                    }
                }

                // Check for type suffix (f32, f64, i32, i64)
                if let Some(ch) = state.peek() {
                    if ch.is_ascii_alphabetic() {
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_alphanumeric() {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                    }
                }

                let token_kind = if is_float { JuliaTokenType::FloatLiteral } else { JuliaTokenType::IntegerLiteral };

                state.add_token(token_kind, start_pos, state.get_position());
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

    /// Lexes a string literal (single or double quoted).
    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '"' || quote == '\'' {
                state.advance(1);
                let mut found_end = false;

                while let Some(ch) = state.peek() {
                    if ch == quote {
                        state.advance(1);
                        found_end = true;
                        break;
                    }
                    else if ch == '\\' {
                        // Handle escape sequences
                        state.advance(1);
                        if let Some(_) = state.peek() {
                            state.advance(1);
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                if found_end {
                    let token_kind = if quote == '\'' { JuliaTokenType::CharLiteral } else { JuliaTokenType::StringLiteral };
                    state.add_token(token_kind, start_pos, state.get_position());
                    true
                }
                else {
                    // No end quote, backtrack
                    state.set_position(start_pos);
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

    /// Lexes triple-quoted string literals.
    fn lex_triple_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        // Check for triple quotes
        if let Some('"') = state.peek() {
            if let Some('"') = state.peek_next_n(1) {
                if let Some('"') = state.peek_next_n(2) {
                    state.advance(3);

                    // Look for closing triple quotes
                    while let Some(ch) = state.peek() {
                        if ch == '"' {
                            if let Some('"') = state.peek_next_n(1) {
                                if let Some('"') = state.peek_next_n(2) {
                                    state.advance(3);
                                    state.add_token(JuliaTokenType::StringLiteral, start_pos, state.get_position());
                                    return true;
                                }
                            }
                        }
                        state.advance(ch.len_utf8());
                    }

                    // No end quote, backtrack
                    state.set_position(start_pos);
                }
            }
        }
        false
    }

    /// Lexes a comment (single-line or nested multi-line).
    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('#') = state.peek() {
            // Check for multi-line comment #= ... =#
            if let Some('=') = state.peek_next_n(1) {
                state.advance(2);
                let mut depth = 1;

                while let Some(ch) = state.peek() {
                    if depth == 0 {
                        break;
                    }
                    if ch == '#' && state.peek_next_n(1) == Some('=') {
                        depth += 1;
                        state.advance(2);
                    }
                    else if ch == '=' && state.peek_next_n(1) == Some('#') {
                        depth -= 1;
                        state.advance(2);
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(JuliaTokenType::Comment, start_pos, state.get_position());
                true
            }
            else {
                // Single-line comment
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }

                state.add_token(JuliaTokenType::Comment, start_pos, state.get_position());
                true
            }
        }
        else {
            false
        }
    }

    /// Lexes operators.
    fn lex_operator<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '+' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        JuliaTokenType::PlusAssign
                    }
                    else {
                        JuliaTokenType::Plus
                    }
                }
                '-' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        JuliaTokenType::MinusAssign
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        JuliaTokenType::Arrow
                    }
                    else {
                        JuliaTokenType::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        JuliaTokenType::StarAssign
                    }
                    else {
                        JuliaTokenType::Star
                    }
                }
                '/' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        JuliaTokenType::SlashAssign
                    }
                    else {
                        JuliaTokenType::Slash
                    }
                }
                '%' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        JuliaTokenType::PercentAssign
                    }
                    else {
                        JuliaTokenType::Percent
                    }
                }
                '^' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        JuliaTokenType::CaretAssign
                    }
                    else {
                        JuliaTokenType::Caret
                    }
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        JuliaTokenType::Equal
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        JuliaTokenType::FatArrow
                    }
                    else {
                        JuliaTokenType::Assign
                    }
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        JuliaTokenType::NotEqual
                    }
                    else {
                        JuliaTokenType::Not
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        JuliaTokenType::LessEqual
                    }
                    else if let Some('<') = state.peek() {
                        state.advance(1);
                        JuliaTokenType::LeftShift
                    }
                    else {
                        JuliaTokenType::LessThan
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        JuliaTokenType::GreaterEqual
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        JuliaTokenType::RightShift
                    }
                    else {
                        JuliaTokenType::GreaterThan
                    }
                }
                '&' => {
                    state.advance(1);
                    if let Some('&') = state.peek() {
                        state.advance(1);
                        JuliaTokenType::And
                    }
                    else {
                        JuliaTokenType::BitAnd
                    }
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.peek() {
                        state.advance(1);
                        JuliaTokenType::Or
                    }
                    else {
                        JuliaTokenType::BitOr
                    }
                }
                '~' => {
                    state.advance(1);
                    JuliaTokenType::BitNot
                }
                ':' => {
                    state.advance(1);
                    JuliaTokenType::Colon
                }
                '.' => {
                    state.advance(1);
                    if let Some('.') = state.peek() {
                        state.advance(1);
                        JuliaTokenType::Range
                    }
                    else {
                        JuliaTokenType::Dot
                    }
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

    /// Lexes delimiters (parentheses, brackets, braces, commas, semicolons).
    fn lex_delimiter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => JuliaTokenType::LeftParen,
                ')' => JuliaTokenType::RightParen,
                '[' => JuliaTokenType::LeftBracket,
                ']' => JuliaTokenType::RightBracket,
                '{' => JuliaTokenType::LeftBrace,
                '}' => JuliaTokenType::RightBrace,
                ',' => JuliaTokenType::Comma,
                ';' => JuliaTokenType::Semicolon,
                _ => return false,
            };

            state.advance(ch.len_utf8());
            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> JuliaLexer<'config> {
    /// Main lexing loop.
    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
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

            if self.lex_triple_string(state) {
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

            if self.lex_operator(state) {
                continue;
            }

            if self.lex_delimiter(state) {
                continue;
            }

            // If no rules match, skip the current character and mark as error
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(JuliaTokenType::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point)
        }

        Ok(())
    }
}
