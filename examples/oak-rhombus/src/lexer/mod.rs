#![doc = include_str!("readme.md")]
use crate::{language::RhombusLanguage, lexer::token_type::RhombusTokenType};
pub mod token_type;
use oak_core::{
    Lexer, LexerCache, LexerState, OakError, Source, TextEdit,
    lexer::{CommentConfig, LexOutput, StringConfig, WhitespaceConfig},
};
use std::sync::LazyLock;

pub(crate) type State<'a, S> = LexerState<'a, S, RhombusLanguage>;

static RHOMBUS_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static RHOMBUS_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: "//", block_start: "/*", block_end: "*/", nested_blocks: true });
static RHOMBUS_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });

#[derive(Clone, Debug)]
pub struct RhombusLexer<'config> {
    config: &'config RhombusLanguage,
}

impl<'config> Lexer<RhombusLanguage> for RhombusLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<RhombusLanguage>) -> LexOutput<RhombusLanguage> {
        let mut state: State<'_, S> = LexerState::new_with_cache(source, 0, cache);
        let result = self.run(&mut state);
        state.finish_with_cache(result, cache)
    }
}

impl<'config> RhombusLexer<'config> {
    pub fn new(config: &'config RhombusLanguage) -> Self {
        Self { config }
    }

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

            if self.lex_single_char_tokens(state) {
                continue;
            }

            // Error handling: if no rules match, skip the current character and mark as error
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(RhombusTokenType::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point)
        }

        // Add EOF token
        state.add_eof();
        Ok(())
    }

    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        RHOMBUS_WHITESPACE.scan(state, RhombusTokenType::Whitespace)
    }

    /// Handles newlines
    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(RhombusTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(RhombusTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        RHOMBUS_COMMENT.scan(state, RhombusTokenType::LineComment, RhombusTokenType::Comment)
    }

    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        RHOMBUS_STRING.scan(state, RhombusTokenType::StringLiteral)
    }

    fn lex_number_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let mut len = 0;
        let mut has_digits = false;

        {
            let rest = state.rest();
            if rest.is_empty() {
                return false;
            }

            let first_char = rest.chars().next().unwrap();
            if !first_char.is_ascii_digit() {
                return false;
            }

            let mut chars = rest.chars();
            while let Some(ch) = chars.next() {
                if ch.is_ascii_digit() || ch == '.' || ch == '_' {
                    len += ch.len_utf8();
                    if ch.is_ascii_digit() {
                        has_digits = true;
                    }
                }
                else {
                    break;
                }
            }
        }

        if has_digits {
            state.advance(len);
            let end = state.get_position();
            state.add_token(RhombusTokenType::NumberLiteral, start, end);
            true
        }
        else {
            false
        }
    }

    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let mut len;

        {
            let rest = state.rest();
            if rest.is_empty() {
                return false;
            }

            let first_char = rest.chars().next().unwrap();
            if !self.is_identifier_start(first_char) {
                return false;
            }

            len = first_char.len_utf8();
            let mut chars = rest.chars().skip(1);

            while let Some(ch) = chars.next() {
                if self.is_identifier_continue(ch) {
                    len += ch.len_utf8();
                }
                else {
                    break;
                }
            }
        }

        let text = state.get_text_in(oak_core::Range { start, end: start + len }).to_string();
        state.advance(len);
        let end = state.get_position();

        let kind = match text.as_str() {
            "fun" => RhombusTokenType::Fun,
            "val" => RhombusTokenType::Val,
            "var" => RhombusTokenType::Var,
            "let" => RhombusTokenType::Let,
            "if" => RhombusTokenType::If,
            "else" => RhombusTokenType::Else,
            "match" => RhombusTokenType::Match,
            "case" => RhombusTokenType::Case,
            "block" => RhombusTokenType::Block,
            "module" => RhombusTokenType::Module,
            "import" => RhombusTokenType::Import,
            "export" => RhombusTokenType::Export,
            "true" | "false" => RhombusTokenType::BooleanLiteral,
            _ => RhombusTokenType::Identifier,
        };

        state.add_token(kind, start, end);
        true
    }

    fn is_identifier_start(&self, ch: char) -> bool {
        ch.is_alphabetic() || ch == '_'
    }

    fn is_identifier_continue(&self, ch: char) -> bool {
        self.is_identifier_start(ch) || ch.is_ascii_digit()
    }

    fn lex_single_char_tokens<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let ch = match state.peek() {
            Some(ch) => ch,
            None => return false,
        };

        let kind = match ch {
            '(' => Some(RhombusTokenType::LeftParen),
            ')' => Some(RhombusTokenType::RightParen),
            '[' => Some(RhombusTokenType::LeftBracket),
            ']' => Some(RhombusTokenType::RightBracket),
            '{' => Some(RhombusTokenType::LeftBrace),
            '}' => Some(RhombusTokenType::RightBrace),
            '.' => Some(RhombusTokenType::Dot),
            ',' => Some(RhombusTokenType::Comma),
            ':' => Some(RhombusTokenType::Colon),
            ';' => Some(RhombusTokenType::Semicolon),
            _ => None,
        };

        if let Some(kind) = kind {
            state.advance(ch.len_utf8());
            state.add_token(kind, start, state.get_position());
            true
        }
        else {
            false
        }
    }
}
