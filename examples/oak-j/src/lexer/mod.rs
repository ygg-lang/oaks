#![doc = include_str!("readme.md")]
/// J token type definitions
pub mod token_type;

pub use token_type::JTokenType;

use crate::language::JLanguage;
use oak_core::{
    Lexer, LexerCache, LexerState, OakError,
    lexer::{LexOutput, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

pub(crate) type State<'a, S> = LexerState<'a, S, JLanguage>;

static J_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });

/// J language lexer
#[derive(Clone, Debug)]
pub struct JLexer<'config> {
    _config: &'config JLanguage,
}

impl<'config> Lexer<JLanguage> for JLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl LexerCache<JLanguage>) -> LexOutput<JLanguage> {
        let mut state: State<'_, S> = LexerState::new_with_cache(source, 0, cache);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> JLexer<'config> {
    /// Creates a new J lexer.
    pub fn new(config: &'config JLanguage) -> Self {
        Self { _config: config }
    }

    /// Main lexing logic.
    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.skip_whitespace(state) {
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

            if self.lex_identifier(state) {
                continue;
            }

            if self.lex_operators(state) {
                continue;
            }

            // If no rules matched, skip current character and add error token
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(JTokenType::Error, safe_point, state.get_position());
            }
        }

        Ok(())
    }

    /// Skips whitespace characters.
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        J_WHITESPACE.scan(state, JTokenType::Whitespace)
    }

    /// J language comments start with `NB.`
    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if state.consume_if_starts_with("NB.") {
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(JTokenType::Comment, start, state.get_position());
            return true;
        }
        false
    }

    /// String literal.
    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if state.consume_if_starts_with("'") {
            while let Some(ch) = state.peek() {
                if ch == '\'' {
                    state.advance(ch.len_utf8());
                    // Handle escaped single quote ''
                    if state.consume_if_starts_with("'") {
                        continue;
                    }
                    state.add_token(JTokenType::StringLiteral, start, state.get_position());
                    return true;
                }
                state.advance(ch.len_utf8());
            }
            // Unclosed string.
            state.add_token(JTokenType::Error, start, state.get_position());
            return true;
        }
        false
    }

    /// Number literal.
    fn lex_number_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() || ch == '_' {
                // J uses _ for negative sign.
                state.advance(ch.len_utf8());
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() || ch == '.' || ch == 'e' || ch == 'E' || ch == 'j' || ch == 'r' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }
                state.add_token(JTokenType::NumberLiteral, start, state.get_position());
                return true;
            }
        }
        false
    }

    /// Identifier.
    fn lex_identifier<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() {
                state.advance(ch.len_utf8());
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }
                state.add_token(JTokenType::Identifier, start, state.get_position());
                return true;
            }
        }
        false
    }

    /// Operators and special symbols.
    fn lex_operators<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        // Try to match long operators.
        for (op, token) in [("=:", JTokenType::IsGlobal), ("=.", JTokenType::IsLocal)] {
            if state.consume_if_starts_with(op) {
                state.add_token(token, start, state.get_position());
                return true;
            }
        }

        // Match single character operators.
        if let Some(ch) = state.peek() {
            let token = match ch {
                '=' => Some(JTokenType::Equal),
                '.' => Some(JTokenType::Dot),
                ':' => Some(JTokenType::Colon),
                '+' => Some(JTokenType::Plus),
                '-' => Some(JTokenType::Minus),
                '*' => Some(JTokenType::Star),
                '%' => Some(JTokenType::Percent),
                '$' => Some(JTokenType::Dollar),
                ',' => Some(JTokenType::Comma),
                '#' => Some(JTokenType::Hash),
                '/' => Some(JTokenType::Slash),
                '\\' => Some(JTokenType::Backslash),
                '|' => Some(JTokenType::Pipe),
                '&' => Some(JTokenType::Ampersand),
                '^' => Some(JTokenType::Caret),
                '~' => Some(JTokenType::Tilde),
                '<' => Some(JTokenType::Less),
                '>' => Some(JTokenType::Greater),
                '(' => Some(JTokenType::LeftParen),
                ')' => Some(JTokenType::RightParen),
                '[' => Some(JTokenType::LeftBracket),
                ']' => Some(JTokenType::RightBracket),
                '{' => Some(JTokenType::LeftBrace),
                '}' => Some(JTokenType::RightBrace),
                _ => None,
            };

            if let Some(token) = token {
                state.advance(ch.len_utf8());
                state.add_token(token, start, state.get_position());
                return true;
            }
        }

        false
    }
}
