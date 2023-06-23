#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::ElixirLanguage, lexer::token_type::ElixirTokenType};
use oak_core::{
    Lexer, LexerCache, LexerState,
    errors::OakError,
    lexer::{CommentConfig, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<'s, S> = LexerState<'s, S, ElixirLanguage>;

static ELIXIR_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static ELIXIR_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: "#", block_start: "", block_end: "", nested_blocks: false });
static ELIXIR_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });
static ELIXIR_CHAR: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['\''], escape: None });

#[derive(Clone, Debug)]
pub struct ElixirLexer<'config> {
    config: &'config ElixirLanguage,
}

impl<'config> Lexer<ElixirLanguage> for ElixirLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl LexerCache<ElixirLanguage>) -> LexOutput<ElixirLanguage> {
        let mut state = State::new_with_cache(source, 0, cache);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> ElixirLexer<'config> {
    pub fn new(config: &'config ElixirLanguage) -> Self {
        Self { config }
    }

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

            if self.lex_char_literal(state) {
                continue;
            }

            if self.lex_sigil(state) {
                continue;
            }

            if self.lex_number_literal(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_atom(state) {
                continue;
            }

            if self.lex_operators(state) {
                continue;
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        ELIXIR_WHITESPACE.scan(state, ElixirTokenType::Whitespace)
    }

    fn skip_comment<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        ELIXIR_COMMENT.scan(state, ElixirTokenType::Comment, ElixirTokenType::Comment)
    }

    fn lex_string_literal<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        ELIXIR_STRING.scan(state, ElixirTokenType::String)
    }

    fn lex_char_literal<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        ELIXIR_CHAR.scan(state, ElixirTokenType::Character)
    }

    fn lex_sigil<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        if state.peek() == Some('~') {
            state.advance(1);
            state.take_while(|c| c.is_alphabetic());
            if let Some(ch) = state.peek() {
                if "\"\'([{<".contains(ch) {
                    // Simplified sigil lexing
                    state.advance(ch.len_utf8());
                    let closer = match ch {
                        '(' => ')',
                        '[' => ']',
                        '{' => '}',
                        '<' => '>',
                        c => c,
                    };
                    state.take_while(|c| c != closer);
                    if state.peek() == Some(closer) {
                        state.advance(closer.len_utf8());
                    }
                }
            }
            state.add_token(ElixirTokenType::Sigil, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_number_literal<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.take_while(|c| c.is_ascii_digit() || c == '_');
                if state.peek() == Some('.') {
                    state.advance(1);
                    state.take_while(|c| c.is_ascii_digit() || c == '_');
                    state.add_token(ElixirTokenType::Float, start, state.get_position());
                }
                else {
                    state.add_token(ElixirTokenType::Number, start, state.get_position());
                }
                return true;
            }
        }
        false
    }

    fn lex_identifier_or_keyword<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                state.take_while(|c| c.is_alphanumeric() || c == '_' || c == '!' || c == '?');
                let text = state.get_text_in(oak_core::Range { start, end: state.get_position() });
                let token_type = match text.as_ref() {
                    "after" => ElixirTokenType::After,
                    "and" => ElixirTokenType::And,
                    "case" => ElixirTokenType::Case,
                    "catch" => ElixirTokenType::Catch,
                    "cond" => ElixirTokenType::Cond,
                    "def" => ElixirTokenType::Def,
                    "defp" => ElixirTokenType::Defp,
                    "defmodule" => ElixirTokenType::Defmodule,
                    "defstruct" => ElixirTokenType::Defstruct,
                    "defprotocol" => ElixirTokenType::Defprotocol,
                    "defimpl" => ElixirTokenType::Defimpl,
                    "defmacro" => ElixirTokenType::Defmacro,
                    "defmacrop" => ElixirTokenType::Defmacrop,
                    "do" => ElixirTokenType::Do,
                    "else" => ElixirTokenType::Else,
                    "elsif" => ElixirTokenType::Elsif,
                    "end" => ElixirTokenType::End,
                    "false" => ElixirTokenType::False,
                    "fn" => ElixirTokenType::Fn,
                    "if" => ElixirTokenType::If,
                    "in" => ElixirTokenType::In,
                    "not" => ElixirTokenType::Not,
                    "or" => ElixirTokenType::Or,
                    "receive" => ElixirTokenType::Receive,
                    "rescue" => ElixirTokenType::Rescue,
                    "true" => ElixirTokenType::True,
                    "try" => ElixirTokenType::Try,
                    "unless" => ElixirTokenType::Unless,
                    "when" => ElixirTokenType::When,
                    "with" => ElixirTokenType::With,
                    _ if text.chars().next().map_or(false, |c| c.is_uppercase()) => ElixirTokenType::Variable,
                    _ => ElixirTokenType::Identifier,
                };
                state.add_token(token_type, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_atom<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        if state.peek() == Some(':') {
            state.advance(1);
            if let Some(ch) = state.peek() {
                if ch.is_alphabetic() || ch == '_' {
                    state.take_while(|c| c.is_alphanumeric() || c == '_' || c == '!' || c == '?');
                }
                else if ch == '"' {
                    state.advance(1);
                    state.take_while(|c| c != '"');
                    if state.peek() == Some('"') {
                        state.advance(1);
                    }
                }
            }
            state.add_token(ElixirTokenType::Atom, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_operators<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        let operators = [
            ("|>", ElixirTokenType::Pipeline),
            ("++", ElixirTokenType::PlusPlus),
            ("--", ElixirTokenType::MinusMinus),
            ("<>", ElixirTokenType::Concat),
            ("==", ElixirTokenType::EqEq),
            ("!=", ElixirTokenType::Ne),
            ("<=", ElixirTokenType::Le),
            (">=", ElixirTokenType::Ge),
            ("&&", ElixirTokenType::AndAnd),
            ("||", ElixirTokenType::OrOr),
            ("<<", ElixirTokenType::LeftDoubleBracket),
            (">>", ElixirTokenType::RightDoubleBracket),
            ("->", ElixirTokenType::Arrow),
            ("+", ElixirTokenType::Plus),
            ("-", ElixirTokenType::Minus),
            ("*", ElixirTokenType::Mul),
            ("/", ElixirTokenType::Div),
            (".", ElixirTokenType::Dot),
            (",", ElixirTokenType::Comma),
            (";", ElixirTokenType::Semicolon),
            (":", ElixirTokenType::Colon),
            ("(", ElixirTokenType::LeftParen),
            (")", ElixirTokenType::RightParen),
            ("{", ElixirTokenType::LeftBrace),
            ("}", ElixirTokenType::RightBrace),
            ("[", ElixirTokenType::LeftBracket),
            ("]", ElixirTokenType::RightBracket),
            ("|", ElixirTokenType::Pipe),
            ("=", ElixirTokenType::Eq),
            ("<", ElixirTokenType::Lt),
            (">", ElixirTokenType::Gt),
            ("!", ElixirTokenType::Bang),
            ("@", ElixirTokenType::At),
            ("%", ElixirTokenType::Percent),
        ];

        for (op, token_type) in operators {
            if state.starts_with(op) {
                state.advance(op.len());
                state.add_token(token_type, start, state.get_position());
                return true;
            }
        }
        false
    }
}
