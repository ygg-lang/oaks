#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::ScssLanguage, lexer::token_type::ScssTokenType};
use oak_core::{
    Lexer, LexerState, OakError, Source, TextEdit,
    lexer::{CommentConfig, LexOutput, LexerCache, StringConfig, WhitespaceConfig},
};
use std::sync::LazyLock;

type State<'a, S> = LexerState<'a, S, ScssLanguage>;

static SCSS_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static SCSS_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: "//", block_start: "/*", block_end: "*/", nested_blocks: true });
static SCSS_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });

/// Lexer for the SCSS language.
#[derive(Debug, Clone)]
pub struct ScssLexer<'config> {
    _config: &'config ScssLanguage,
}

impl<'config> Lexer<ScssLanguage> for ScssLexer<'config> {
    /// Tokenizes the input source text.
    fn lex<'a, S: Source + ?Sized>(&self, text: &S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<ScssLanguage>) -> LexOutput<ScssLanguage> {
        let mut state = LexerState::new(text);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> ScssLexer<'config> {
    /// Creates a new `ScssLexer` with the given configuration.
    pub fn new(config: &'config ScssLanguage) -> Self {
        Self { _config: config }
    }

    /// Main lexer loop that tokenizes the source text.
    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_newline(state) {
                continue;
            }

            if self.skip_comment(state) {
                continue;
            }

            if self.lex_string_literal(state) {
                continue;
            }

            if self.lex_number_literal(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_operators(state) {
                continue;
            }

            if self.lex_single_char_tokens(state) {
                continue;
            }

            // Error handling: if no rule matches, skip current character and mark as error
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(ScssTokenType::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point)
        }

        Ok(())
    }

    /// Skips whitespace characters.
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        SCSS_WHITESPACE.scan(state, ScssTokenType::Whitespace)
    }

    /// Handles newlines.
    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(ScssTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1)
            }
            state.add_token(ScssTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Skips comments.
    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        SCSS_COMMENT.scan(state, ScssTokenType::Comment, ScssTokenType::Comment)
    }

    /// Lexes string literals.
    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        SCSS_STRING.scan(state, ScssTokenType::StringLiteral)
    }

    /// Lexes number literals.
    fn lex_number_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(first_char) = state.peek() {
            if first_char.is_ascii_digit() {
                state.advance(first_char.len_utf8());

                // Continue with digits
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() { state.advance(ch.len_utf8()) } else { break }
                }

                // Handle decimal point
                if state.peek() == Some('.') && state.peek_next_n(1).map_or(false, |c| c.is_ascii_digit()) {
                    state.advance(1); // consume '.'
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() { state.advance(ch.len_utf8()) } else { break }
                    }
                }

                state.add_token(ScssTokenType::IntegerLiteral, start, state.get_position());
                return true;
            }
        }
        false
    }

    /// Lexes identifiers or keywords.
    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let text = state.source().get_text_from(start);

        if let Some(first_char) = text.chars().next() {
            if first_char.is_alphabetic() || first_char == '_' || first_char == '$' {
                let mut len = first_char.len_utf8();

                let mut chars = text.chars().skip(1);
                while let Some(ch) = chars.next() {
                    if ch.is_alphanumeric() || ch == '_' || ch == '-' {
                        len += ch.len_utf8();
                    }
                    else {
                        break;
                    }
                }

                let word = &text[..len];
                let kind = self.keyword_kind(word).unwrap_or(ScssTokenType::Identifier);
                state.advance(len);
                state.add_token(kind, start, state.get_position());
                return true;
            }
        }
        false
    }

    /// Lexes operators.
    fn lex_operators<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let text = state.source().get_text_from(start);

        // Two-character operators
        if text.len() >= 2 {
            let two_char = &text[..2];
            if let Some(kind) = self.operator_kind(two_char) {
                state.advance(2);
                state.add_token(kind, start, state.get_position());
                return true;
            }
        }

        // Single-character operators
        if let Some(first_char) = text.chars().next() {
            let single_char = &text[..first_char.len_utf8()];
            if let Some(kind) = self.operator_kind(single_char) {
                state.advance(first_char.len_utf8());
                state.add_token(kind, start, state.get_position());
                return true;
            }
        }

        false
    }

    /// Lexes single character tokens.
    fn lex_single_char_tokens<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let text = state.source().get_text_from(start);

        if let Some(first_char) = text.chars().next() {
            let single_char = &text[..first_char.len_utf8()];
            if let Some(kind) = self.single_char_kind(single_char) {
                state.advance(first_char.len_utf8());
                state.add_token(kind, start, state.get_position());
                return true;
            }
        }

        false
    }

    /// Returns the syntax kind for a given keyword.
    fn keyword_kind(&self, text: &str) -> Option<ScssTokenType> {
        match text {
            "import" => Some(ScssTokenType::Import),
            "include" => Some(ScssTokenType::Include),
            "mixin" => Some(ScssTokenType::Mixin),
            "function" => Some(ScssTokenType::Function),
            "return" => Some(ScssTokenType::Return),
            "if" => Some(ScssTokenType::If),
            "else" => Some(ScssTokenType::Else),
            "for" => Some(ScssTokenType::For),
            "while" => Some(ScssTokenType::While),
            "each" => Some(ScssTokenType::Each),
            "in" => Some(ScssTokenType::In),
            "true" => Some(ScssTokenType::True),
            "false" => Some(ScssTokenType::False),
            "null" => Some(ScssTokenType::Null),
            _ => None,
        }
    }

    fn operator_kind(&self, text: &str) -> Option<ScssTokenType> {
        match text {
            "==" => Some(ScssTokenType::EqEq),
            "!=" => Some(ScssTokenType::Ne),
            "<=" => Some(ScssTokenType::Le),
            ">=" => Some(ScssTokenType::Ge),
            "&&" => Some(ScssTokenType::AndAnd),
            "||" => Some(ScssTokenType::OrOr),
            "=" => Some(ScssTokenType::Eq),
            "<" => Some(ScssTokenType::Lt),
            ">" => Some(ScssTokenType::Gt),
            "&" => Some(ScssTokenType::And),
            "|" => Some(ScssTokenType::Or),
            "^" => Some(ScssTokenType::Xor),
            "+" => Some(ScssTokenType::Plus),
            "-" => Some(ScssTokenType::Minus),
            "*" => Some(ScssTokenType::Star),
            "/" => Some(ScssTokenType::Slash),
            "%" => Some(ScssTokenType::Percent),
            "!" => Some(ScssTokenType::Bang),
            _ => None,
        }
    }

    fn single_char_kind(&self, text: &str) -> Option<ScssTokenType> {
        match text {
            "(" => Some(ScssTokenType::LeftParen),
            ")" => Some(ScssTokenType::RightParen),
            "{" => Some(ScssTokenType::LeftBrace),
            "}" => Some(ScssTokenType::RightBrace),
            "[" => Some(ScssTokenType::LeftBracket),
            "]" => Some(ScssTokenType::RightBracket),
            ";" => Some(ScssTokenType::Semicolon),
            ":" => Some(ScssTokenType::Colon),
            "," => Some(ScssTokenType::Comma),
            "." => Some(ScssTokenType::Dot),
            "#" => Some(ScssTokenType::Hash),
            "@" => Some(ScssTokenType::At),
            "$" => Some(ScssTokenType::Dollar),
            _ => None,
        }
    }
}
