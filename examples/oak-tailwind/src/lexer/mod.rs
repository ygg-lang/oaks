#![doc = include_str!("readme.md")]
/// Token types for the Tailwind language.
pub mod token_type;

use crate::{language::TailwindLanguage, lexer::token_type::TailwindTokenType};
use oak_core::{Lexer, LexerCache, LexerState, OakError, lexer::LexOutput, source::Source};

/// Lexer for the Tailwind language.
#[derive(Clone, Debug, Default)]
pub struct TailwindLexer {
    /// Language configuration
    pub config: TailwindLanguage,
}

pub(crate) type State<'a, S> = LexerState<'a, S, TailwindLanguage>;

impl TailwindLexer {
    /// Creates a new `TailwindLexer` with the given configuration.
    pub fn new(config: TailwindLanguage) -> Self {
        Self { config }
    }
}

impl Lexer<TailwindLanguage> for TailwindLexer {
    /// Tokenizes the source text into a sequence of Tailwind tokens.
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[oak_core::TextEdit], cache: &'a mut impl LexerCache<TailwindLanguage>) -> LexOutput<TailwindLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}

impl TailwindLexer {
    /// Runs the lexer logic.
    pub fn run<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_comment(state) {
                continue;
            }

            if self.lex_directive(state) {
                continue;
            }

            if self.lex_tailwind_class_part(state) {
                continue;
            }

            if self.lex_punctuation(state) {
                continue;
            }

            state.advance_if_dead_lock(safe_point)
        }

        Ok(())
    }

    /// Skips whitespace characters.
    pub fn skip_whitespace<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();
        let mut found = false;

        while let Some(ch) = state.peek() {
            if ch.is_whitespace() {
                state.advance(ch.len_utf8());
                found = true
            }
            else {
                break;
            }
        }

        if found {
            state.add_token(TailwindTokenType::Whitespace, start, state.get_position())
        }

        found
    }

    /// Lexes a comment.
    pub fn lex_comment<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();
        if state.consume_if_starts_with("/*") {
            while state.not_at_end() {
                if state.consume_if_starts_with("*/") {
                    break;
                }
                if let Some(ch) = state.peek() {
                    state.advance(ch.len_utf8())
                }
            }
            state.add_token(TailwindTokenType::Comment, start, state.get_position());
            return true;
        }
        if state.consume_if_starts_with("//") {
            while state.not_at_end() {
                if let Some(ch) = state.peek() {
                    if ch == '\n' {
                        break;
                    }
                    state.advance(ch.len_utf8())
                }
            }
            state.add_token(TailwindTokenType::Comment, start, state.get_position());
            return true;
        }
        false
    }

    /// Lexes a directive like @tailwind, @apply.
    pub fn lex_directive<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();
        if state.consume_if_starts_with("@") {
            while let Some(ch) = state.peek() {
                if ch.is_alphabetic() || ch == '-' {
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }
            state.add_token(TailwindTokenType::Directive, start, state.get_position());
            return true;
        }
        false
    }

    /// Lexes a part of a Tailwind class (modifier, utility, or arbitrary value).
    pub fn lex_tailwind_class_part<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();

        if state.consume_if_starts_with("!") {
            state.add_token(TailwindTokenType::Important, start, state.get_position());
            return true;
        }

        if state.peek() == Some('[') {
            return self.lex_arbitrary_value(state);
        }

        // Try lexing a modifier or utility
        let mut has_content = false;
        let _current_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch.is_alphanumeric() || ch == '-' || ch == '/' || ch == '.' || ch == '_' {
                state.advance(ch.len_utf8());
                has_content = true;

                if state.peek() == Some(':') {
                    state.advance(':'.len_utf8());
                    state.add_token(TailwindTokenType::Modifier, start, state.get_position());
                    return true;
                }
            }
            else {
                break;
            }
        }

        if has_content {
            state.add_token(TailwindTokenType::Utility, start, state.get_position());
            return true;
        }

        false
    }

    /// Lexes an arbitrary value like [100px].
    pub fn lex_arbitrary_value<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();
        if state.consume_if_starts_with("[") {
            let mut depth = 1;
            while state.not_at_end() && depth > 0 {
                if let Some(ch) = state.peek() {
                    if ch == '[' {
                        depth += 1;
                    }
                    else if ch == ']' {
                        depth -= 1;
                    }
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }
            state.add_token(TailwindTokenType::ArbitraryValue, start, state.get_position());
            return true;
        }
        false
    }

    /// Lexes punctuation.
    pub fn lex_punctuation<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();

        macro_rules! check {
            ($s:expr, $t:ident) => {
                if state.consume_if_starts_with($s) {
                    state.add_token(TailwindTokenType::$t, start, state.get_position());
                    return true;
                }
            };
        }

        check!("[", LeftBracket);
        check!("]", RightBracket);
        check!("(", LeftParen);
        check!(")", RightParen);
        check!(":", Colon);
        check!(";", Semicolon);
        check!("@", At);
        check!("!", Bang);
        check!("-", Dash);
        check!("/", Slash);
        check!(".", Dot);
        check!("#", Hash);
        check!(",", Comma);

        false
    }
}
