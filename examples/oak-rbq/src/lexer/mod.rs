#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::RbqLanguage, lexer::token_type::RbqTokenType};
use oak_core::{
    errors::OakError,
    lexer::{CommentConfig, LexOutput, Lexer, LexerCache, LexerState},
    source::{Source, TextEdit},
};
use std::sync::LazyLock;

type State<'a, S> = LexerState<'a, S, RbqLanguage>;

static RBQ_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: "#", block_start: "", block_end: "", nested_blocks: false });

/// Lexer for the RBQ language.
#[derive(Clone, Debug)]
pub struct RbqLexer<'config> {
    _config: &'config RbqLanguage,
}

impl<'config> Lexer<RbqLanguage> for RbqLexer<'config> {
    /// Tokenizes the source text into a sequence of RBQ tokens.
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<RbqLanguage>) -> LexOutput<RbqLanguage> {
        let mut state = State::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> RbqLexer<'config> {
    /// Creates a new `RbqLexer` with the given configuration.
    pub fn new(config: &'config RbqLanguage) -> Self {
        Self { _config: config }
    }

    /// Runs the main lexing loop.
    fn run<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();
            let Some(ch) = state.peek()
            else {
                break;
            };

            match ch {
                ' ' | '\t' | '\n' | '\r' => {
                    let start = state.get_position();
                    state.skip_ascii_whitespace();
                    state.add_token(RbqTokenType::Whitespace, start, state.get_position())
                }
                '#' => {
                    RBQ_COMMENT.scan(state, RbqTokenType::Comment, RbqTokenType::Comment);
                }
                '"' => self.lex_string(state),
                '0'..='9' => self.lex_number(state),
                '{' | '}' | '[' | ']' | '(' | ')' | ':' | ';' | ',' | '.' | '?' | '@' | '$' => self.lex_punctuation(state),
                '=' | '!' | '>' | '<' | '&' | '+' | '-' | '*' | '/' => self.lex_operator(state),
                '|' => self.lex_pipe(state),
                _ if ch.is_alphabetic() || ch == '_' => self.lex_ident_or_keyword(state),
                _ => {
                    state.advance(ch.len_utf8());
                    state.add_token(RbqTokenType::Error, safe_point, state.get_position())
                }
            }
            state.advance_if_dead_lock(safe_point)
        }
        Ok(())
    }

    fn lex_string<S: Source + ?Sized>(&self, state: &mut State<'_, S>) {
        let start = state.get_position();
        state.advance(1); // "
        while state.not_at_end() {
            if state.consume_if_starts_with("\"") {
                state.add_token(RbqTokenType::StringLiteral, start, state.get_position());
                return;
            }
            if state.consume_if_starts_with("\\") { state.advance(1) } else { state.advance(1) }
        }
        state.add_token(RbqTokenType::Error, start, state.get_position())
    }

    fn lex_number<S: Source + ?Sized>(&self, state: &mut State<'_, S>) {
        let start = state.get_position();
        state.take_while(|c| c.is_ascii_digit());
        if state.consume_if_starts_with(".") {
            state.take_while(|c| c.is_ascii_digit());
        }
        state.add_token(RbqTokenType::NumberLiteral, start, state.get_position())
    }

    fn lex_punctuation<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let start = state.get_position();
        let ch = state.peek().unwrap();
        let kind = match ch {
            '{' => RbqTokenType::LeftBrace,
            '}' => RbqTokenType::RightBrace,
            '[' => RbqTokenType::LeftBracket,
            ']' => RbqTokenType::RightBracket,
            '(' => RbqTokenType::LeftParen,
            ')' => RbqTokenType::RightParen,
            ':' => RbqTokenType::Colon,
            ';' => RbqTokenType::Semicolon,
            ',' => RbqTokenType::Comma,
            '.' => RbqTokenType::Dot,
            '?' => RbqTokenType::Question,
            '@' => RbqTokenType::At,
            '$' => RbqTokenType::Dollar,
            _ => unreachable!(),
        };
        state.advance(1);
        state.add_token(kind, start, state.get_position());
    }

    fn lex_operator<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let start = state.get_position();
        if state.consume_if_starts_with("==") {
            state.add_token(RbqTokenType::EqEq, start, state.get_position())
        }
        else if state.consume_if_starts_with("!=") {
            state.add_token(RbqTokenType::NotEq, start, state.get_position())
        }
        else if state.consume_if_starts_with(">=") {
            state.add_token(RbqTokenType::GtEq, start, state.get_position())
        }
        else if state.consume_if_starts_with("<=") {
            state.add_token(RbqTokenType::LtEq, start, state.get_position())
        }
        else if state.consume_if_starts_with("&&") {
            state.add_token(RbqTokenType::AndAnd, start, state.get_position())
        }
        else if state.consume_if_starts_with("->") {
            state.add_token(RbqTokenType::Arrow, start, state.get_position())
        }
        else if state.consume_if_starts_with("=") {
            state.add_token(RbqTokenType::Eq, start, state.get_position())
        }
        else if state.consume_if_starts_with("!") {
            state.add_token(RbqTokenType::Not, start, state.get_position())
        }
        else if state.consume_if_starts_with(">") {
            state.add_token(RbqTokenType::Gt, start, state.get_position())
        }
        else if state.consume_if_starts_with("<") {
            state.add_token(RbqTokenType::Lt, start, state.get_position())
        }
        else if state.consume_if_starts_with("+") {
            state.add_token(RbqTokenType::Plus, start, state.get_position())
        }
        else if state.consume_if_starts_with("-") {
            state.add_token(RbqTokenType::Minus, start, state.get_position())
        }
        else if state.consume_if_starts_with("*") {
            state.add_token(RbqTokenType::Star, start, state.get_position())
        }
        else if state.consume_if_starts_with("/") {
            state.add_token(RbqTokenType::Slash, start, state.get_position())
        }
        else if state.consume_if_starts_with("&") {
            state.add_token(RbqTokenType::Ampersand, start, state.get_position())
        }
    }

    fn lex_pipe<S: Source + ?Sized>(&self, state: &mut State<'_, S>) {
        let start = state.get_position();
        if state.consume_if_starts_with("||") {
            state.add_token(RbqTokenType::OrOr, start, state.get_position())
        }
        else if state.consume_if_starts_with("|") {
            state.add_token(RbqTokenType::Pipe, start, state.get_position())
        }
    }

    fn lex_ident_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let start = state.get_position();
        state.take_while(|c| c.is_alphanumeric() || c == '_');
        let text = state.get_text_in(oak_core::Range { start, end: state.get_position() });
        let kind = match text.as_ref() {
            "struct" => RbqTokenType::StructKw,
            "class" => RbqTokenType::ClassKw,
            "enum" => RbqTokenType::EnumKw,
            "union" => RbqTokenType::UnionKw,
            "trait" => RbqTokenType::TraitKw,
            "using" => RbqTokenType::UsingKw,
            "namespace" => RbqTokenType::NamespaceKw,
            "use" => RbqTokenType::UseKw,
            "type" => RbqTokenType::TypeKw,
            "micro" => RbqTokenType::MicroKw,
            "utf8" => RbqTokenType::Utf8Kw,
            "true" => RbqTokenType::TrueKw,
            "false" => RbqTokenType::FalseKw,
            _ => RbqTokenType::Ident,
        };
        state.add_token(kind, start, state.get_position());
    }
}
