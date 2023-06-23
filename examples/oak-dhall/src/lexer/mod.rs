#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::DHallLanguage, lexer::token_type::DHallTokenType};
use oak_core::{
    LexOutput, Lexer, LexerCache, LexerState, OakError,
    lexer::{CommentConfig, StringConfig, WhitespaceConfig},
    source::{Source, TextEdit},
};
use std::sync::LazyLock;

static DHALL_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static DHALL_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: "--", block_start: "{-", block_end: "-}", nested_blocks: true });
static DHALL_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });

#[derive(Clone)]
pub struct DHallLexer<'config> {
    _config: &'config DHallLanguage,
}

impl<'config> Lexer<DHallLanguage> for DHallLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<DHallLanguage>) -> LexOutput<DHallLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        state.finish_with_cache(result, cache)
    }
}

impl<'config> DHallLexer<'config> {
    pub fn new(config: &'config DHallLanguage) -> Self {
        Self { _config: config }
    }

    fn run<'a, S: Source + ?Sized>(&self, state: &mut LexerState<'a, S, DHallLanguage>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();
            if self.skip_whitespace(state) {
                continue;
            };

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

            state.advance_if_dead_lock(safe_point)
        }

        Ok(())
    }

    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut LexerState<'a, S, DHallLanguage>) -> bool {
        DHALL_WHITESPACE.scan(state, DHallTokenType::Whitespace)
    }

    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut LexerState<'a, S, DHallLanguage>) -> bool {
        DHALL_COMMENT.scan(state, DHallTokenType::Comment, DHallTokenType::Comment)
    }

    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut LexerState<'a, S, DHallLanguage>) -> bool {
        DHALL_STRING.scan(state, DHallTokenType::String)
    }

    fn lex_number_literal<'a, S: Source + ?Sized>(&self, state: &mut LexerState<'a, S, DHallLanguage>) -> bool {
        let start = state.get_position();
        let first = match state.peek() {
            Some(c) => c,
            None => return false,
        };

        if !first.is_ascii_digit() {
            return false;
        }

        state.advance(1);
        while let Some(c) = state.peek() {
            if c.is_ascii_digit() { state.advance(1) } else { break }
        }

        state.add_token(DHallTokenType::Number, start, state.get_position());
        true
    }

    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut LexerState<'a, S, DHallLanguage>) -> bool {
        let start = state.get_position();
        let first = match state.peek() {
            Some(c) => c,
            None => return false,
        };

        if !first.is_alphabetic() && first != '_' && first != 'λ' {
            return false;
        }

        state.advance(1);
        while let Some(c) = state.peek() {
            if c.is_alphanumeric() || c == '_' || c == '-' || c == '/' { state.advance(1) } else { break }
        }

        let end = state.get_position();
        let text = state.get_text_in((start..end).into());

        let kind = match text.as_ref() {
            "if" => DHallTokenType::If,
            "then" => DHallTokenType::Then,
            "else" => DHallTokenType::Else,
            "let" => DHallTokenType::Let,
            "in" => DHallTokenType::In,
            "using" => DHallTokenType::Using,
            "as" => DHallTokenType::As,
            "merge" => DHallTokenType::Merge,
            "Some" => DHallTokenType::Some,
            "None" => DHallTokenType::None,
            "with" => DHallTokenType::With,
            "forall" => DHallTokenType::Forall,
            "assert" => DHallTokenType::Assert,
            "Bool" => DHallTokenType::Bool,
            "Natural" => DHallTokenType::Natural,
            "Integer" => DHallTokenType::Integer,
            "Double" => DHallTokenType::Double,
            "Text" => DHallTokenType::Text,
            "List" => DHallTokenType::List,
            "Optional" => DHallTokenType::Optional,
            "True" => DHallTokenType::True,
            "False" => DHallTokenType::False,
            "λ" => DHallTokenType::Lambda,
            _ => DHallTokenType::Identifier,
        };

        state.add_token(kind, start, end);
        true
    }

    fn lex_operators<'a, S: Source + ?Sized>(&self, state: &mut LexerState<'a, S, DHallLanguage>) -> bool {
        let start = state.get_position();
        let text = state.rest();

        let ops = [
            ("->", DHallTokenType::Arrow),
            ("→", DHallTokenType::Arrow),
            ("=>", DHallTokenType::FatArrow),
            ("==", DHallTokenType::EqualEqual),
            ("≡", DHallTokenType::EqualEqual),
            ("!=", DHallTokenType::NotEqual),
            ("&&", DHallTokenType::And),
            ("∧", DHallTokenType::And),
            ("||", DHallTokenType::Or),
            ("∨", DHallTokenType::Or),
            ("++", DHallTokenType::Append),
            ("//", DHallTokenType::Combine),
            ("⫽", DHallTokenType::Combine),
            ("/\\", DHallTokenType::CombineTypes),
            ("⩓", DHallTokenType::CombineTypes),
            ("//\\", DHallTokenType::Prefer),
            ("∀", DHallTokenType::Forall),
            ("λ", DHallTokenType::Lambda),
        ];

        for (op, kind) in ops {
            if text.starts_with(op) {
                state.advance(op.len());
                state.add_token(kind, start, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_single_char_tokens<'a, S: Source + ?Sized>(&self, state: &mut LexerState<'a, S, DHallLanguage>) -> bool {
        let start = state.get_position();
        let c = match state.peek() {
            Some(c) => c,
            None => return false,
        };

        let kind = match c {
            '(' => DHallTokenType::LeftParen,
            ')' => DHallTokenType::RightParen,
            '[' => DHallTokenType::LeftBracket,
            ']' => DHallTokenType::RightBracket,
            '{' => DHallTokenType::LeftBrace,
            '}' => DHallTokenType::RightBrace,
            '<' => DHallTokenType::Less,
            '>' => DHallTokenType::Greater,
            ',' => DHallTokenType::Comma,
            '.' => DHallTokenType::Dot,
            ':' => DHallTokenType::Colon,
            ';' => DHallTokenType::Semicolon,
            '=' => DHallTokenType::Equal,
            '@' => DHallTokenType::At,
            '#' => DHallTokenType::Hash,
            '?' => DHallTokenType::Question,
            '+' => DHallTokenType::Plus,
            '*' => DHallTokenType::Star,
            '/' => DHallTokenType::Slash,
            '|' => DHallTokenType::Pipe,
            '\\' => DHallTokenType::Lambda,
            _ => return false,
        };

        state.advance(1);
        state.add_token(kind, start, state.get_position());
        true
    }
}
