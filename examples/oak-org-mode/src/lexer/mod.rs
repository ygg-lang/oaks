#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::OrgModeLanguage, lexer::token_type::OrgModeTokenType};
use oak_core::{
    TextEdit,
    errors::OakError,
    lexer::{CommentConfig, LexOutput, Lexer, LexerCache, LexerState, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

pub(crate) type State<'a, S> = LexerState<'a, S, OrgModeLanguage>;

static ORG_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: false });
static ORG_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: "#", block_start: "", block_end: "", nested_blocks: false });
static ORG_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });

/// Org-mode lexer.
#[derive(Clone, Debug)]
pub struct OrgModeLexer<'config> {
    config: &'config OrgModeLanguage,
}

impl<'config> OrgModeLexer<'config> {
    /// Creates a new `OrgModeLexer`.
    pub fn new(config: &'config OrgModeLanguage) -> Self {
        Self { config }
    }

    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        ORG_WHITESPACE.scan(state, OrgModeTokenType::Whitespace)
    }

    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        ORG_COMMENT.scan(state, OrgModeTokenType::Comment, OrgModeTokenType::Comment)
    }

    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        ORG_STRING.scan(state, OrgModeTokenType::Text)
    }

    fn lex_text_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() {
                let start_pos = state.get_position();
                // Read letters and digits
                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }
                let end_pos = state.get_position();
                let text = state.source().get_text_in((start_pos..end_pos).into());
                let kind = if self.config.todo_keywords.iter().any(|k| k == text.as_ref()) {
                    OrgModeTokenType::Todo
                }
                else if self.config.done_keywords.iter().any(|k| k == text.as_ref()) {
                    OrgModeTokenType::Done
                }
                else {
                    OrgModeTokenType::Text
                };
                state.add_token(kind, start_pos, end_pos);
                return true;
            }
        }
        false
    }

    fn lex_priority<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if state.starts_with("[#") {
            let start_pos = state.get_position();
            state.advance(2);

            if let Some(ch) = state.peek() {
                if ch.is_alphabetic() {
                    state.advance(ch.len_utf8());
                    if let Some(']') = state.peek() {
                        state.advance(1);
                        state.add_token(OrgModeTokenType::Priority, start_pos, state.get_position());
                        return true;
                    }
                }
            }

            state.set_position(start_pos);
            false
        }
        else {
            false
        }
    }

    fn lex_number_or_date<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                let start_pos = state.get_position();
                let mut has_dash = false;

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else if ch == '-' {
                        state.advance(1);
                        has_dash = true;
                    }
                    else {
                        break;
                    }
                }

                let kind = if has_dash { OrgModeTokenType::Date } else { OrgModeTokenType::Number };

                state.add_token(kind, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_symbols<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some(ch) = state.peek() {
            let start_pos = state.get_position();
            state.advance(ch.len_utf8());

            let kind = match ch {
                '+' => OrgModeTokenType::Plus,
                '-' => OrgModeTokenType::Minus,
                '*' => OrgModeTokenType::Star,
                '#' => OrgModeTokenType::Hash,
                '|' => OrgModeTokenType::Pipe,
                ':' => OrgModeTokenType::Colon,
                '[' => OrgModeTokenType::LeftBracket,
                ']' => OrgModeTokenType::RightBracket,
                '(' => OrgModeTokenType::LeftParen,
                ')' => OrgModeTokenType::RightParen,
                '{' => OrgModeTokenType::LeftBrace,
                '}' => OrgModeTokenType::RightBrace,
                '<' => OrgModeTokenType::LessThan,
                '>' => OrgModeTokenType::GreaterThan,
                '=' => OrgModeTokenType::Equal,
                '_' => OrgModeTokenType::Underscore,
                '~' => OrgModeTokenType::Tilde,
                '/' => OrgModeTokenType::Slash,
                '\\' => OrgModeTokenType::Backslash,
                '\n' => OrgModeTokenType::Newline,
                _ => {
                    // Unknown character, handle as text
                    state.add_token(OrgModeTokenType::Text, start_pos, state.get_position());
                    return true;
                }
            };

            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            // Prioritize newline characters
            if let Some('\n') = state.peek() {
                let start_pos = state.get_position();
                state.advance(1);
                state.add_token(OrgModeTokenType::Newline, start_pos, state.get_position());
                continue;
            }

            // Skip whitespace characters
            if self.skip_whitespace(state) {
                continue;
            }

            // Handle comments
            if self.skip_comment(state) {
                continue;
            }

            // Handle strings
            if self.lex_string(state) {
                continue;
            }

            // Handle priority
            if self.lex_priority(state) {
                continue;
            }

            // Handle numbers or dates
            if self.lex_number_or_date(state) {
                continue;
            }

            // Handle text or keywords
            if self.lex_text_or_keyword(state) {
                continue;
            }

            // Handle symbols
            if self.lex_symbols(state) {
                continue;
            }

            // If no pattern matched, create error token
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(OrgModeTokenType::Error, start_pos, state.get_position());
            }
            else {
                break;
            }

            state.advance_if_dead_lock(safe_point);
        }
        Ok(())
    }
}

impl<'config> Lexer<OrgModeLanguage> for OrgModeLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<OrgModeLanguage>) -> LexOutput<OrgModeLanguage> {
        let mut state = State::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}
