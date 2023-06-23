#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::SassLanguage, lexer::token_type::SassTokenType};
use oak_core::{
    Lexer, LexerState, OakError, TextEdit,
    lexer::{CommentConfig, LexOutput, LexerCache, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<'s, S> = LexerState<'s, S, SassLanguage>;

static SASS_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static SASS_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: "//", block_start: "/*", block_end: "*/", nested_blocks: false });
static SASS_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });
static SASS_CHAR: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['\''], escape: Some('\\') });

#[derive(Clone, Debug)]
pub struct SassLexer<'config> {
    _config: &'config SassLanguage,
}

impl<'config> Lexer<SassLanguage> for SassLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<SassLanguage>) -> LexOutput<SassLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> SassLexer<'config> {
    pub fn new(config: &'config SassLanguage) -> Self {
        Self { _config: config }
    }

    /// Main lexer loop that tokenizes the source text.
    fn run<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> Result<(), OakError> {
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

            if self.lex_variable(state) {
                continue;
            }

            if self.lex_color_literal(state) {
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

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }

    /// Skips whitespace characters.
    fn skip_whitespace<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        SASS_WHITESPACE.scan(state, SassTokenType::Whitespace)
    }

    fn skip_comment<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        SASS_COMMENT.scan(state, SassTokenType::LineComment, SassTokenType::BlockComment)
    }

    fn lex_string_literal<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        if SASS_STRING.scan(state, SassTokenType::StringLiteral) {
            return true;
        }
        if SASS_CHAR.scan(state, SassTokenType::StringLiteral) {
            return true;
        }
        false
    }

    fn lex_number_literal<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        let first = match state.current() {
            Some(c) => c,
            None => return false,
        };
        if !first.is_ascii_digit() {
            return false;
        }

        let mut is_float = false;
        state.advance(first.len_utf8());

        // Read integer part
        while let Some(c) = state.current() {
            if c.is_ascii_digit() || c == '_' {
                state.advance(c.len_utf8());
            }
            else {
                break;
            }
        }

        // fractional part
        if state.current() == Some('.') {
            let n1 = state.source().get_char_at(state.get_position() + 1);
            if n1.map(|c| c.is_ascii_digit()).unwrap_or(false) {
                is_float = true;
                state.advance(1); // consume '.'
                while let Some(c) = state.current() {
                    if c.is_ascii_digit() || c == '_' {
                        state.advance(c.len_utf8());
                    }
                    else {
                        break;
                    }
                }
            }
        }

        // Unit suffix (px, em, rem, %, etc.)
        while let Some(c) = state.current() {
            if c.is_ascii_alphabetic() || c == '%' {
                state.advance(c.len_utf8());
            }
            else {
                break;
            }
        }

        let end = state.get_position();
        state.add_token(if is_float { SassTokenType::FloatLiteral } else { SassTokenType::NumberLiteral }, start, end);
        true
    }

    fn lex_identifier_or_keyword<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        let ch = match state.current() {
            Some(c) => c,
            None => return false,
        };
        if !(ch.is_ascii_alphabetic() || ch == '_' || ch == '-' || ch == '@' || ch == '!') {
            return false;
        }
        state.advance(ch.len_utf8());
        while let Some(c) = state.current() {
            if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                state.advance(c.len_utf8());
            }
            else {
                break;
            }
        }
        let end = state.get_position();
        let text = state.source().get_text_in(core::range::Range { start, end });
        let kind = match text.as_ref() {
            "@import" => SassTokenType::Import,
            "@include" => SassTokenType::Include,
            "@extend" => SassTokenType::Extend,
            "@mixin" => SassTokenType::Mixin,
            "@function" => SassTokenType::Function,
            "@return" => SassTokenType::Return,
            "@if" => SassTokenType::If,
            "@else" => SassTokenType::Else,
            "@elseif" => SassTokenType::ElseIf,
            "@for" => SassTokenType::For,
            "@each" => SassTokenType::Each,
            "@while" => SassTokenType::While,
            "!default" => SassTokenType::Default,
            "!important" => SassTokenType::Important,
            "!optional" => SassTokenType::Optional,
            "!global" => SassTokenType::Global,
            "and" => SassTokenType::And,
            "or" => SassTokenType::Or,
            "not" => SassTokenType::Not,
            _ => SassTokenType::Identifier,
        };
        state.add_token(kind, start, end);
        true
    }

    fn lex_variable<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        if state.current() != Some('$') {
            return false;
        }
        state.advance(1);

        // Variable name must start with a letter or underscore
        if let Some(ch) = state.current() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());
                while let Some(c) = state.current() {
                    if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                        state.advance(c.len_utf8());
                    }
                    else {
                        break;
                    }
                }
                state.add_token(SassTokenType::Variable, start, state.get_position());
                return true;
            }
        }
        state.set_position(start);
        false
    }

    fn lex_color_literal<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        if state.current() != Some('#') {
            return false;
        }
        state.advance(1);

        let mut hex_digits = 0;
        while let Some(c) = state.current() {
            if c.is_ascii_hexdigit() {
                state.advance(c.len_utf8());
                hex_digits += 1;
            }
            else {
                break;
            }
        }

        // Valid hex color length: 3, 4, 6, 8
        if hex_digits == 3 || hex_digits == 4 || hex_digits == 6 || hex_digits == 8 {
            state.add_token(SassTokenType::ColorLiteral, start, state.get_position());
            return true;
        }

        state.set_position(start);
        false
    }

    fn lex_operators<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();

        // Multi-character operators
        let patterns: &[(&str, SassTokenType)] = &[("==", SassTokenType::EqEq), ("!=", SassTokenType::Ne), ("<=", SassTokenType::Le), (">=", SassTokenType::Ge)];

        for (pat, kind) in patterns {
            if state.source().get_text_from(start).as_ref().starts_with(pat) {
                state.advance(pat.len());
                state.add_token(*kind, start, state.get_position());
                return true;
            }
        }

        // Single-character operators
        if let Some(ch) = state.current() {
            let kind = match ch {
                '+' => Some(SassTokenType::Plus),
                '-' => Some(SassTokenType::Minus),
                '*' => Some(SassTokenType::Star),
                '/' => Some(SassTokenType::Slash),
                '%' => Some(SassTokenType::Percent),
                '=' => Some(SassTokenType::Eq),
                '<' => Some(SassTokenType::Lt),
                '>' => Some(SassTokenType::Gt),
                _ => None,
            };
            if let Some(k) = kind {
                state.advance(ch.len_utf8());
                state.add_token(k, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_single_char_tokens<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.current() {
            let kind = match ch {
                '(' => SassTokenType::LeftParen,
                ')' => SassTokenType::RightParen,
                '{' => SassTokenType::LeftBrace,
                '}' => SassTokenType::RightBrace,
                '[' => SassTokenType::LeftBracket,
                ']' => SassTokenType::RightBracket,
                ';' => SassTokenType::Semicolon,
                ':' => SassTokenType::Colon,
                ',' => SassTokenType::Comma,
                '.' => SassTokenType::Dot,
                '#' => SassTokenType::Hash,
                '$' => SassTokenType::Dollar,
                '@' => SassTokenType::At,
                '&' => SassTokenType::Ampersand,
                '!' => SassTokenType::Exclamation,
                '?' => SassTokenType::Question,
                '~' => SassTokenType::Tilde,
                _ => return false,
            };
            state.advance(ch.len_utf8());
            state.add_token(kind, start, state.get_position());
            return true;
        }
        false
    }
}
