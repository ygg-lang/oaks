#![doc = include_str!("readme.md")]
pub mod token_type;

pub use token_type::AplTokenType;

use crate::language::AplLanguage;
use oak_core::{
    Lexer, LexerCache, LexerState, OakError,
    lexer::{LexOutput, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<'a, S> = LexerState<'a, S, AplLanguage>;

static APL_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });

#[derive(Clone, Debug)]
pub struct AplLexer<'config> {
    config: &'config AplLanguage,
}

impl<'config> Lexer<AplLanguage> for AplLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl LexerCache<AplLanguage>) -> LexOutput<AplLanguage> {
        let mut state: State<'_, S> = LexerState::new_with_cache(source, 0, cache);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> AplLexer<'config> {
    pub fn new(config: &'config AplLanguage) -> Self {
        Self { config }
    }

    /// 主要词法分析逻辑
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

            if self.lex_symbols(state) {
                continue;
            }

            // 如果没有匹配任何模式，跳过当前字符并生成 Error token
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(AplTokenType::Error, safe_point, state.get_position());
            }
        }

        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        APL_WHITESPACE.scan(state, AplTokenType::Whitespace)
    }

    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if state.peek() == Some('⍝') {
            state.advance('⍝'.len_utf8());
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(AplTokenType::Comment, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if let Some(quote) = state.peek() {
            if quote == '\'' || quote == '"' {
                state.advance(1);
                while let Some(ch) = state.peek() {
                    if ch == quote {
                        state.advance(1);
                        if state.peek() == Some(quote) {
                            state.advance(1);
                            continue;
                        }
                        break;
                    }
                    state.advance(ch.len_utf8());
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                }
                state.add_token(AplTokenType::StringLiteral, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_number_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() || ch == '¯' || (ch == '.' && state.peek_next_n(1).map_or(false, |c| c.is_ascii_digit())) {
                if ch == '¯' {
                    state.advance('¯'.len_utf8());
                }

                let mut has_digits = false;
                while let Some(c) = state.peek() {
                    if c.is_ascii_digit() {
                        state.advance(1);
                        has_digits = true;
                    }
                    else {
                        break;
                    }
                }

                if state.peek() == Some('.') {
                    state.advance(1);
                    while let Some(c) = state.peek() {
                        if c.is_ascii_digit() {
                            state.advance(1);
                            has_digits = true;
                        }
                        else {
                            break;
                        }
                    }
                }

                if !has_digits && state.get_position() == start {
                    return false;
                }

                if let Some(e) = state.peek() {
                    if e == 'e' || e == 'E' {
                        state.advance(1);
                        if let Some(sign) = state.peek() {
                            if sign == '+' || sign == '-' || sign == '¯' {
                                state.advance(sign.len_utf8());
                            }
                        }
                        while let Some(c) = state.peek() {
                            if c.is_ascii_digit() {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                    }
                }

                state.add_token(AplTokenType::NumberLiteral, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_identifier<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '∆' || ch == '⍙' {
                state.advance(ch.len_utf8());
                while let Some(c) = state.peek() {
                    if c.is_alphanumeric() || c == '∆' || c == '⍙' || c == '_' {
                        state.advance(c.len_utf8());
                    }
                    else {
                        break;
                    }
                }
                state.add_token(AplTokenType::Identifier, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_symbols<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.peek() {
            let token = match ch {
                '←' => AplTokenType::LeftArrow,
                '→' => AplTokenType::RightArrow,
                '⋄' => AplTokenType::Diamond,
                '⎕' => AplTokenType::Quad,
                '⍞' => AplTokenType::QuoteQuad,
                '⍴' => AplTokenType::Rho,
                '⍳' => AplTokenType::Iota,
                '∊' => AplTokenType::Epsilon,
                '↑' => AplTokenType::UpArrow,
                '↓' => AplTokenType::DownArrow,
                '∇' => AplTokenType::Del,
                '∆' => AplTokenType::Delta,
                '⍺' => AplTokenType::Alpha,
                '⍵' => AplTokenType::Omega,
                '⍬' => AplTokenType::Zilde,
                '+' => AplTokenType::Plus,
                '-' => AplTokenType::Minus,
                '×' => AplTokenType::Times,
                '÷' => AplTokenType::Divide,
                '*' => AplTokenType::Star,
                '⍟' => AplTokenType::Log,
                '○' => AplTokenType::Circle,
                '∨' => AplTokenType::Or,
                '∧' => AplTokenType::And,
                '∼' => AplTokenType::Not,
                '⍱' => AplTokenType::Nor,
                '⍲' => AplTokenType::Nand,
                '=' => AplTokenType::Equal,
                '≠' => AplTokenType::NotEqual,
                '<' => AplTokenType::LessThan,
                '≤' => AplTokenType::LessEqual,
                '≥' => AplTokenType::GreaterEqual,
                '>' => AplTokenType::GreaterThan,
                '⌈' => AplTokenType::UpStile,
                '⌊' => AplTokenType::DownStile,
                '|' => AplTokenType::Bar,
                '~' => AplTokenType::Tilde,
                '?' => AplTokenType::Question,
                '!' => AplTokenType::Factorial,
                '/' => AplTokenType::Slash,
                '\\' => AplTokenType::Backslash,
                '⌿' => AplTokenType::SlashBar,
                '⍀' => AplTokenType::BackslashBar,
                '.' => AplTokenType::Dot,
                '∘' => AplTokenType::Jot,
                '¨' => AplTokenType::Diaeresis,
                '⍣' => AplTokenType::Power,
                '⍤' => AplTokenType::Rank,
                '≢' => AplTokenType::Tally,
                '(' => AplTokenType::LeftParen,
                ')' => AplTokenType::RightParen,
                '[' => AplTokenType::LeftBracket,
                ']' => AplTokenType::RightBracket,
                '{' => AplTokenType::LeftBrace,
                '}' => AplTokenType::RightBrace,
                ';' => AplTokenType::Semicolon,
                _ => return false,
            };
            state.advance(ch.len_utf8());
            state.add_token(token, start, state.get_position());
            return true;
        }
        false
    }
}
