#![doc = include_str!("readme.md")]
/// Less token types and role definitions.
pub mod token_type;
use crate::language::LessLanguage;
use oak_core::{Lexer, LexerState, OakError, lexer::LexOutput, source::Source};
pub use token_type::LessTokenType;

type State<'s, S> = LexerState<'s, S, LessLanguage>;

/// Lexer for the Less language.
pub struct LessLexer<'config> {
    /// Language configuration.
    config: &'config LessLanguage,
}

impl<'config> LessLexer<'config> {
    /// Creates a new `LessLexer` with the given language configuration.
    pub fn new(config: &'config LessLanguage) -> Self {
        Self { config }
    }

    /// Skips whitespace characters.
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' { state.advance(ch.len_utf8()) } else { break }
        }

        if state.get_position() > start_pos {
            state.add_token(LessTokenType::Whitespace, start_pos, state.get_position());
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
            state.add_token(LessTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1)
            }
            state.add_token(LessTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles CSS comments (`/* ... */`).
    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            if let Some('*') = state.peek_next_n(1) {
                state.advance(2); // Skip /*

                while let Some(ch) = state.peek() {
                    if ch == '*' && state.peek_next_n(1) == Some('/') {
                        state.advance(2); // Skip */
                        break;
                    }
                    state.advance(ch.len_utf8())
                }

                state.add_token(LessTokenType::Comment, start_pos, state.get_position());
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

    /// Handles string literals (both single and double quoted).
    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '"' || quote == '\'' {
                state.advance(1); // Skip opening quote

                while let Some(ch) = state.peek() {
                    if ch == quote {
                        state.advance(1); // Skip closing quote
                        break;
                    }
                    else if ch == '\\' {
                        state.advance(1); // Skip escape character
                        if state.peek().is_some() {
                            state.advance(1)
                        }
                    }
                    else {
                        state.advance(ch.len_utf8())
                    }
                }

                state.add_token(LessTokenType::StringLiteral, start_pos, state.get_position());
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

    /// Handles CSS URLs (`url(...)`).
    fn lex_url<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('u') = state.peek() {
            if state.peek_next_n(1) == Some('r') && state.peek_next_n(2) == Some('l') && state.peek_next_n(3) == Some('(') {
                state.advance(4); // Skip url(

                // Skip whitespace
                while let Some(ch) = state.peek() {
                    if ch.is_whitespace() { state.advance(ch.len_utf8()) } else { break }
                }

                // Check for quoted or unquoted URL
                if let Some(quote) = state.peek() {
                    if quote == '"' || quote == '\'' {
                        state.advance(1);
                        while let Some(ch) = state.peek() {
                            if ch == quote {
                                state.advance(1);
                                break;
                            }
                            else if ch == '\\' {
                                state.advance(1);
                                if state.peek().is_some() {
                                    state.advance(1)
                                }
                            }
                            else {
                                state.advance(ch.len_utf8())
                            }
                        }
                    }
                    else {
                        while let Some(ch) = state.peek() {
                            if ch == ')' || ch.is_whitespace() {
                                break;
                            }
                            state.advance(ch.len_utf8())
                        }
                    }
                }

                // Skip whitespace
                while let Some(ch) = state.peek() {
                    if ch.is_whitespace() { state.advance(ch.len_utf8()) } else { break }
                }

                // Skip closing )
                if let Some(')') = state.peek() {
                    state.advance(1)
                }

                state.add_token(LessTokenType::UrlLiteral, start_pos, state.get_position());
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

    /// Handles color literals (e.g., `#fff`, `#ffffff`).
    fn lex_color<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('#') = state.peek() {
            state.advance(1); // Skip #

            let mut count = 0;
            while let Some(ch) = state.peek() {
                if ch.is_ascii_hexdigit() {
                    state.advance(1);
                    count += 1
                }
                else {
                    break;
                }
            }

            if count == 3 || count == 4 || count == 6 || count == 8 {
                state.add_token(LessTokenType::ColorLiteral, start_pos, state.get_position());
                true
            }
            else {
                // Not a valid color, but we'll treat it as a hash + something else
                // This is a simplification for the lexer
                state.add_token(LessTokenType::Hash, start_pos, start_pos + 1);
                state.set_position(start_pos + 1);
                true
            }
        }
        else {
            false
        }
    }

    /// Handles number literals and units (e.g., `10px`, `1.5em`, `100%`).
    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        let mut has_digits = false;
        if let Some(ch) = state.peek() {
            if ch == '+' || ch == '-' {
                state.advance(1)
            }
        }

        while let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);
                has_digits = true
            }
            else {
                break;
            }
        }

        if let Some('.') = state.peek() {
            if let Some(next_ch) = state.peek_next_n(1) {
                if next_ch.is_ascii_digit() {
                    state.advance(1); // Skip .
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(1);
                            has_digits = true
                        }
                        else {
                            break;
                        }
                    }
                }
            }
        }

        if has_digits {
            // Check for units
            let unit_start = state.get_position();
            while let Some(ch) = state.peek() {
                if ch.is_alphabetic() || ch == '%' { state.advance(ch.len_utf8()) } else { break }
            }

            if state.get_position() > unit_start {
                // We have a number with a unit
                state.add_token(LessTokenType::NumberLiteral, start_pos, state.get_position())
            }
            else {
                state.add_token(LessTokenType::NumberLiteral, start_pos, state.get_position())
            }
            true
        }
        else {
            state.set_position(start_pos);
            false
        }
    }

    /// Handles identifiers (e.g., property names, selectors).
    fn lex_identifier<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' || ch == '-' {
                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' || ch == '-' { state.advance(ch.len_utf8()) } else { break }
                }

                state.add_token(LessTokenType::Identifier, start_pos, state.get_position());
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

    /// Handles CSS at-rules (e.g., `@import`, `@media`).
    fn lex_at_rule<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('@') = state.peek() {
            state.advance(1); // Skip @

            let rule_start = state.get_position();
            while let Some(ch) = state.peek() {
                if ch.is_alphabetic() || ch == '-' { state.advance(ch.len_utf8()) } else { break }
            }

            let rule_name = state.get_text_in((rule_start..state.get_position()).into());
            let token_kind = match rule_name.as_ref() {
                "import" => LessTokenType::AtImport,
                "media" => LessTokenType::AtMedia,
                "keyframes" => LessTokenType::AtKeyframes,
                "font-face" => LessTokenType::AtFontFace,
                "charset" => LessTokenType::AtCharset,
                "namespace" => LessTokenType::AtNamespace,
                "supports" => LessTokenType::AtSupports,
                "page" => LessTokenType::AtPage,
                "document" => LessTokenType::AtDocument,
                _ => LessTokenType::AtRule,
            };

            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles delimiters (e.g., `(`, `)`, `{`, `}`, `,`, `;`).
    fn lex_delimiter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => LessTokenType::LeftParen,
                ')' => LessTokenType::RightParen,
                '{' => LessTokenType::LeftBrace,
                '}' => LessTokenType::RightBrace,
                '[' => LessTokenType::LeftBracket,
                ']' => LessTokenType::RightBracket,
                ',' => LessTokenType::Comma,
                ';' => LessTokenType::Semicolon,
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

    /// Handles operators (e.g., `:`, `.`, `>`, `+`, `~`, `*`, `/`).
    fn lex_operator<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                ':' => LessTokenType::Colon,
                '.' => LessTokenType::Dot,
                '#' => LessTokenType::Hash,
                '+' => LessTokenType::Plus,
                '-' => LessTokenType::Minus,
                '*' => LessTokenType::Star,
                '/' => LessTokenType::Slash,
                '=' => LessTokenType::Equals,
                '~' => LessTokenType::Tilde,
                '|' => LessTokenType::Pipe,
                '^' => LessTokenType::Caret,
                '$' => LessTokenType::Dollar,
                '>' => LessTokenType::GreaterThan,
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

    /// Main entry point for the lexer.
    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            // Try various lexing rules
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

            if self.lex_url(state) {
                continue;
            }

            if self.lex_color(state) {
                continue;
            }

            if self.lex_number(state) {
                continue;
            }

            if self.lex_at_rule(state) {
                continue;
            }

            if self.lex_identifier(state) {
                continue;
            }

            if self.lex_delimiter(state) {
                continue;
            }

            if self.lex_operator(state) {
                continue;
            }

            // If no rules match, skip the current character and mark as error
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(LessTokenType::Error, start_pos, state.get_position())
            }
            else {
                break;
            }

            state.advance_if_dead_lock(safe_point)
        }
        Ok(())
    }
}

impl<'config> Lexer<LessLanguage> for LessLexer<'config> {
    /// Tokenizes the source code into a stream of Less tokens.
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[oak_core::source::TextEdit], mut cache: &'a mut impl oak_core::lexer::LexerCache<LessLanguage>) -> LexOutput<LessLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, &mut cache)
    }
}
