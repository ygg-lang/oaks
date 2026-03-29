/// EJS Lexer module
///
/// This module defines the lexer for EJS (Embedded JavaScript) templates,
/// responsible for tokenizing the input into meaningful tokens.
use oak_core::{
    OakError,
    lexer::{LexOutput, Lexer, LexerCache, LexerState},
    source::Source,
};

/// Token type definitions for EJS lexer
pub mod token_type;
use crate::language::EjsLanguage;
use token_type::EjsTokenType;

/// Lexer state type alias for EJS
pub(crate) type State<'a, S> = LexerState<'a, S, EjsLanguage>;

/// Lexer for EJS templates
///
/// The EJS lexer handles two distinct modes:
/// - Text mode: Recognizes plain text content until an EJS tag is encountered
/// - Code mode: Uses JavaScript lexical rules inside EJS tags
///
/// The lexer transitions between these modes based on the EJS delimiters.
#[derive(Debug, Clone)]
pub struct EjsLexer<'config> {
    /// Language configuration containing delimiter settings
    config: &'config EjsLanguage,
}

impl<'config> EjsLexer<'config> {
    /// Creates a new EJS lexer with the given configuration
    ///
    /// # Arguments
    ///
    /// * `config` - Reference to the EJS language configuration
    ///
    /// # Returns
    ///
    /// A new `EjsLexer` instance
    pub fn new(config: &'config EjsLanguage) -> Self {
        Self { config }
    }

    /// Main lexing loop that processes the entire source
    ///
    /// This method orchestrates the lexing process by alternating between
    /// text mode and code mode based on the EJS delimiters encountered.
    fn run<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.lex_text(state) {
                continue;
            }

            if self.lex_template_tag(state) {
                continue;
            }

            state.advance_if_dead_lock(safe_point)
        }

        Ok(())
    }

    /// Lexes plain text content outside of EJS tags
    ///
    /// This method recognizes text content until it encounters an EJS opening
    /// delimiter (`<%`) or reaches the end of the source.
    ///
    /// # Returns
    ///
    /// `true` if text was successfully lexed, `false` otherwise
    fn lex_text<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();

        while let Some(ch) = state.peek() {
            let rest = state.rest();

            if rest.starts_with(&self.config.open_delimiter) {
                break;
            }

            state.advance(ch.len_utf8());
        }

        if state.get_position() > start {
            state.add_token(EjsTokenType::Text, start, state.get_position());
            return true;
        }

        false
    }

    /// Lexes EJS template tags and their contents
    ///
    /// This method handles the complete lexing of EJS tags including:
    /// - Opening tags: `<%`, `<%=`, `%-`, `<%#`, `<%%`
    /// - Tag content using JavaScript lexical rules
    /// - Closing tags: `%>`, `-%>`
    ///
    /// # Returns
    ///
    /// `true` if a template tag was successfully lexed, `false` otherwise
    fn lex_template_tag<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let rest = state.rest();

        if !rest.starts_with(&self.config.open_delimiter) {
            return false;
        }

        let open_start = state.get_position();
        let open_delim_len = self.config.open_delimiter.len();
        state.advance(open_delim_len);

        let after_open = state.get_position();

        if let Some(ch) = state.peek() {
            let marker = ch.to_string();

            if marker == self.config.output_escape {
                state.advance(1);
                state.add_token(EjsTokenType::OpenTagOutputEscape, open_start, state.get_position());
                self.lex_code_content(state);
                return true;
            }

            if marker == self.config.output_raw {
                state.advance(1);
                state.add_token(EjsTokenType::OpenTagOutputRaw, open_start, state.get_position());
                self.lex_code_content(state);
                return true;
            }

            if marker == self.config.comment_marker {
                state.advance(1);
                state.add_token(EjsTokenType::OpenTagComment, open_start, state.get_position());
                self.lex_comment_content(state);
                return true;
            }

            if ch == '%' {
                state.advance(1);
                state.add_token(EjsTokenType::EscapedOpenTag, open_start, state.get_position());
                return true;
            }
        }

        state.add_token(EjsTokenType::OpenTag, open_start, after_open);
        self.lex_code_content(state);
        true
    }

    /// Lexes the content inside a code EJS tag
    ///
    /// Uses JavaScript lexical rules to tokenize the code content until
    /// a closing tag is encountered.
    fn lex_code_content<S: Source + ?Sized>(&self, state: &mut State<'_, S>) {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.lex_close_tag(state) {
                return;
            }

            if self.lex_whitespace(state) {
                continue;
            }

            if self.lex_newline(state) {
                continue;
            }

            if self.lex_string(state) {
                continue;
            }

            if self.lex_number(state) {
                continue;
            }

            if self.lex_identifier(state) {
                continue;
            }

            if self.lex_punctuation(state) {
                continue;
            }

            state.advance_if_dead_lock(safe_point)
        }
    }

    /// Lexes the content inside a comment EJS tag
    ///
    /// Consumes all content until a closing tag is found.
    fn lex_comment_content<S: Source + ?Sized>(&self, state: &mut State<'_, S>) {
        let start = state.get_position();

        while state.not_at_end() {
            if self.is_close_tag(state) {
                break;
            }

            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
            }
        }

        if state.get_position() > start {
            state.add_token(EjsTokenType::Comment, start, state.get_position());
        }

        self.lex_close_tag(state);
    }

    /// Checks if the current position is at a closing tag
    ///
    /// Handles both regular closing tag (`%>`) and trim mode closing tag (`-%>`).
    fn is_close_tag<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let rest = state.rest();

        if rest.starts_with(&self.config.close_delimiter) {
            return true;
        }

        let trim_close = format!("-{}", self.config.close_delimiter);
        rest.starts_with(&trim_close)
    }

    /// Lexes a closing EJS tag
    ///
    /// Recognizes both regular closing tag (`%>`) and trim mode closing tag (`-%>`).
    /// The trim mode closing tag removes trailing whitespace from the output.
    ///
    /// # Returns
    ///
    /// `true` if a closing tag was successfully lexed, `false` otherwise
    fn lex_close_tag<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        let trim_close = format!("-{}", self.config.close_delimiter);

        if rest.starts_with(&trim_close) {
            state.advance(trim_close.len());
            state.add_token(EjsTokenType::CloseTagTrim, start, state.get_position());
            return true;
        }

        if rest.starts_with(&self.config.close_delimiter) {
            state.advance(self.config.close_delimiter.len());
            state.add_token(EjsTokenType::CloseTag, start, state.get_position());
            return true;
        }

        false
    }

    /// Lexes whitespace characters (space and tab)
    ///
    /// Consecutive whitespace characters are grouped into a single token.
    ///
    /// # Returns
    ///
    /// `true` if whitespace was successfully lexed, `false` otherwise
    fn lex_whitespace<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();
        let mut found = false;

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' {
                state.advance(ch.len_utf8());
                found = true;
            }
            else {
                break;
            }
        }

        if found {
            state.add_token(EjsTokenType::Whitespace, start, state.get_position());
        }

        found
    }

    /// Lexes newline characters
    ///
    /// Handles both Unix-style (`\n`) and Windows-style (`\r\n`) line endings.
    ///
    /// # Returns
    ///
    /// `true` if a newline was successfully lexed, `false` otherwise
    fn lex_newline<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();

        if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(EjsTokenType::Newline, start, state.get_position());
            return true;
        }

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(EjsTokenType::Newline, start, state.get_position());
            return true;
        }

        false
    }

    /// Lexes string literals
    ///
    /// Handles both single-quoted and double-quoted strings with escape sequences.
    ///
    /// # Returns
    ///
    /// `true` if a string literal was successfully lexed, `false` otherwise
    fn lex_string<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '"' || quote == '\'' {
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch == quote {
                        state.advance(1);
                        break;
                    }

                    if ch == '\\' {
                        state.advance(1);
                        if let Some(escaped) = state.peek() {
                            state.advance(escaped.len_utf8());
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(EjsTokenType::String, start, state.get_position());
                return true;
            }
        }

        false
    }

    /// Lexes numeric literals
    ///
    /// Handles integer and floating-point numbers, including hexadecimal notation.
    ///
    /// # Returns
    ///
    /// `true` if a number was successfully lexed, `false` otherwise
    fn lex_number<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            if ch == '0' {
                if let Some(next) = state.peek_next_n(1) {
                    if next == 'x' || next == 'X' {
                        state.advance(2);

                        while let Some(hex_ch) = state.peek() {
                            if hex_ch.is_ascii_hexdigit() {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }

                        state.add_token(EjsTokenType::Number, start, state.get_position());
                        return true;
                    }
                }
            }

            if ch.is_ascii_digit() {
                while let Some(digit) = state.peek() {
                    if digit.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                if let Some('.') = state.peek() {
                    if let Some(next) = state.peek_next_n(1) {
                        if next.is_ascii_digit() {
                            state.advance(1);

                            while let Some(digit) = state.peek() {
                                if digit.is_ascii_digit() {
                                    state.advance(1);
                                }
                                else {
                                    break;
                                }
                            }
                        }
                    }
                }

                if let Some(exp) = state.peek() {
                    if exp == 'e' || exp == 'E' {
                        state.advance(1);

                        if let Some(sign) = state.peek() {
                            if sign == '+' || sign == '-' {
                                state.advance(1);
                            }
                        }

                        while let Some(digit) = state.peek() {
                            if digit.is_ascii_digit() {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                    }
                }

                state.add_token(EjsTokenType::Number, start, state.get_position());
                return true;
            }
        }

        false
    }

    /// Lexes identifiers and keywords
    ///
    /// Recognizes JavaScript identifiers and the keywords: true, false, null, undefined.
    ///
    /// # Returns
    ///
    /// `true` if an identifier or keyword was successfully lexed, `false` otherwise
    fn lex_identifier<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' || ch == '$' {
                state.advance(ch.len_utf8());

                while let Some(next_ch) = state.peek() {
                    if next_ch.is_ascii_alphanumeric() || next_ch == '_' || next_ch == '$' {
                        state.advance(next_ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let end = state.get_position();
                let text = state.get_text_in((start..end).into());

                let kind = match text.as_ref() {
                    "true" | "false" => EjsTokenType::Boolean,
                    "null" | "undefined" => EjsTokenType::Boolean,
                    _ => EjsTokenType::Identifier,
                };

                state.add_token(kind, start, end);
                return true;
            }
        }

        false
    }

    /// Lexes punctuation and operators
    ///
    /// Recognizes JavaScript operators and punctuation symbols.
    ///
    /// # Returns
    ///
    /// `true` if punctuation was successfully lexed, `false` otherwise
    fn lex_punctuation<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '(' => EjsTokenType::LeftParen,
                ')' => EjsTokenType::RightParen,
                '{' => EjsTokenType::LeftBrace,
                '}' => EjsTokenType::RightBrace,
                '[' => EjsTokenType::LeftBracket,
                ']' => EjsTokenType::RightBracket,
                ',' => EjsTokenType::Comma,
                '.' => EjsTokenType::Dot,
                ':' => EjsTokenType::Colon,
                ';' => EjsTokenType::Semicolon,
                '=' => EjsTokenType::Eq,
                '+' => EjsTokenType::Plus,
                '-' => EjsTokenType::Minus,
                '*' => EjsTokenType::Star,
                '/' => EjsTokenType::Slash,
                '%' => EjsTokenType::Percent,
                '!' => EjsTokenType::Bang,
                '?' => EjsTokenType::Question,
                '<' => EjsTokenType::Lt,
                '>' => EjsTokenType::Gt,
                '&' => EjsTokenType::Amp,
                '|' => EjsTokenType::Pipe,
                '^' => EjsTokenType::Caret,
                '~' => EjsTokenType::Tilde,
                _ => return false,
            };

            state.advance(ch.len_utf8());
            state.add_token(kind, start, state.get_position());
            return true;
        }

        false
    }
}

impl<'config> Lexer<EjsLanguage> for EjsLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl LexerCache<EjsLanguage>) -> LexOutput<EjsLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);

        if result.is_ok() {
            state.add_eof()
        }

        state.finish_with_cache(result, cache)
    }
}
