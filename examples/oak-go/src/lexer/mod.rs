#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::GoLanguage, lexer::token_type::GoTokenType};
use oak_core::{Lexer, LexerCache, LexerState, OakError, lexer::LexOutput, source::Source};

type State<'a, S> = LexerState<'a, S, GoLanguage>;

#[derive(Clone)]
pub struct GoLexer<'config> {
    _config: &'config GoLanguage,
}

impl<'config> Lexer<GoLanguage> for GoLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl LexerCache<GoLanguage>) -> LexOutput<GoLanguage> {
        let mut state = State::new_with_cache(source, 0, cache);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> GoLexer<'config> {
    pub fn new(config: &'config GoLanguage) -> Self {
        Self { _config: config }
    }

    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.skip_whitespace(state) {
                continue;
            }

            if self.skip_comment(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_literal(state) {
                continue;
            }

            if self.lex_operator_or_delimiter(state) {
                continue;
            }

            // Fallback
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(GoTokenType::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point)
        }

        Ok(())
    }

    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        while let Some(ch) = state.peek() {
            if ch.is_whitespace() {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }
        if state.get_position() > start {
            state.add_token(GoTokenType::Whitespace, start, state.get_position());
            return true;
        }
        false
    }

    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if state.consume_if_starts_with("//") {
            while let Some(ch) = state.peek() {
                if ch == '\n' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(GoTokenType::Comment, start, state.get_position());
            return true;
        }
        if state.consume_if_starts_with("/*") {
            while let Some(ch) = state.peek() {
                if state.consume_if_starts_with("*/") {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(GoTokenType::Comment, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());
                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = state.get_text_in(oak_core::Range { start, end: state.get_position() });
                let kind = match text.as_ref() {
                    "package" => GoTokenType::Package,
                    "import" => GoTokenType::Import,
                    "func" => GoTokenType::Func,
                    "var" => GoTokenType::Var,
                    "const" => GoTokenType::Const,
                    "type" => GoTokenType::Type,
                    "struct" => GoTokenType::Struct,
                    "interface" => GoTokenType::Interface,
                    "map" => GoTokenType::Map,
                    "chan" => GoTokenType::Chan,
                    "if" => GoTokenType::If,
                    "else" => GoTokenType::Else,
                    "for" => GoTokenType::For,
                    "range" => GoTokenType::Range,
                    "return" => GoTokenType::Return,
                    "break" => GoTokenType::Break,
                    "continue" => GoTokenType::Continue,
                    "goto" => GoTokenType::Goto,
                    "switch" => GoTokenType::Switch,
                    "case" => GoTokenType::Case,
                    "default" => GoTokenType::Default,
                    "defer" => GoTokenType::Defer,
                    "go" => GoTokenType::Go,
                    "select" => GoTokenType::Select,
                    "fallthrough" => GoTokenType::Fallthrough,
                    "true" => GoTokenType::BoolLiteral,
                    "false" => GoTokenType::BoolLiteral,
                    "nil" => GoTokenType::NilLiteral,
                    _ => GoTokenType::Identifier,
                };
                state.add_token(kind, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.peek() {
            // String literal
            if ch == '"' {
                state.advance(ch.len_utf8());
                while let Some(ch) = state.peek() {
                    if ch == '"' {
                        state.advance(ch.len_utf8());
                        break;
                    }
                    if ch == '\\' {
                        state.advance(ch.len_utf8());
                        if let Some(next) = state.peek() {
                            state.advance(next.len_utf8());
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }
                state.add_token(GoTokenType::StringLiteral, start, state.get_position());
                return true;
            }
            // Raw string literal
            if ch == '`' {
                state.advance(ch.len_utf8());
                while let Some(ch) = state.peek() {
                    if ch == '`' {
                        state.advance(ch.len_utf8());
                        break;
                    }
                    state.advance(ch.len_utf8());
                }
                state.add_token(GoTokenType::StringLiteral, start, state.get_position());
                return true;
            }
            // Rune literal
            if ch == '\'' {
                state.advance(ch.len_utf8());
                while let Some(ch) = state.peek() {
                    if ch == '\'' {
                        state.advance(ch.len_utf8());
                        break;
                    }
                    if ch == '\\' {
                        state.advance(ch.len_utf8());
                        if let Some(next) = state.peek() {
                            state.advance(next.len_utf8());
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }
                state.add_token(GoTokenType::RuneLiteral, start, state.get_position());
                return true;
            }
            // Number literal
            if ch.is_ascii_digit() {
                state.advance(ch.len_utf8());
                let mut has_dot = false;
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(ch.len_utf8());
                    }
                    else if ch == '.' && !has_dot {
                        has_dot = true;
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }
                let kind = if has_dot { GoTokenType::FloatLiteral } else { GoTokenType::IntLiteral };
                state.add_token(kind, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_operator_or_delimiter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let kind = if state.consume_if_starts_with(":=") {
            GoTokenType::ColonAssign
        }
        else if state.consume_if_starts_with("...") {
            GoTokenType::Ellipsis
        }
        else if state.consume_if_starts_with("<<=") {
            GoTokenType::LeftShiftAssign
        }
        else if state.consume_if_starts_with(">>=") {
            GoTokenType::RightShiftAssign
        }
        else if state.consume_if_starts_with("&^=") {
            GoTokenType::AmpersandCaretAssign
        }
        else if state.consume_if_starts_with("==") {
            GoTokenType::Equal
        }
        else if state.consume_if_starts_with("!=") {
            GoTokenType::NotEqual
        }
        else if state.consume_if_starts_with("<=") {
            GoTokenType::LessEqual
        }
        else if state.consume_if_starts_with(">=") {
            GoTokenType::GreaterEqual
        }
        else if state.consume_if_starts_with("&&") {
            GoTokenType::LogicalAnd
        }
        else if state.consume_if_starts_with("||") {
            GoTokenType::LogicalOr
        }
        else if state.consume_if_starts_with("<<") {
            GoTokenType::LeftShift
        }
        else if state.consume_if_starts_with(">>") {
            GoTokenType::RightShift
        }
        else if state.consume_if_starts_with("&^") {
            GoTokenType::AmpersandCaret
        }
        else if state.consume_if_starts_with("++") {
            GoTokenType::Increment
        }
        else if state.consume_if_starts_with("--") {
            GoTokenType::Decrement
        }
        else if state.consume_if_starts_with("+=") {
            GoTokenType::PlusAssign
        }
        else if state.consume_if_starts_with("-=") {
            GoTokenType::MinusAssign
        }
        else if state.consume_if_starts_with("*=") {
            GoTokenType::StarAssign
        }
        else if state.consume_if_starts_with("/=") {
            GoTokenType::SlashAssign
        }
        else if state.consume_if_starts_with("%=") {
            GoTokenType::PercentAssign
        }
        else if state.consume_if_starts_with("&=") {
            GoTokenType::AmpersandAssign
        }
        else if state.consume_if_starts_with("|=") {
            GoTokenType::PipeAssign
        }
        else if state.consume_if_starts_with("^=") {
            GoTokenType::CaretAssign
        }
        else if state.consume_if_starts_with("<-") {
            GoTokenType::Arrow
        }
        else if state.consume_if_starts_with("{") {
            GoTokenType::LeftBrace
        }
        else if state.consume_if_starts_with("}") {
            GoTokenType::RightBrace
        }
        else if state.consume_if_starts_with("(") {
            GoTokenType::LeftParen
        }
        else if state.consume_if_starts_with(")") {
            GoTokenType::RightParen
        }
        else if state.consume_if_starts_with("[") {
            GoTokenType::LeftBracket
        }
        else if state.consume_if_starts_with("]") {
            GoTokenType::RightBracket
        }
        else if state.consume_if_starts_with(".") {
            GoTokenType::Dot
        }
        else if state.consume_if_starts_with(",") {
            GoTokenType::Comma
        }
        else if state.consume_if_starts_with(";") {
            GoTokenType::Semicolon
        }
        else if state.consume_if_starts_with(":") {
            GoTokenType::Colon
        }
        else if state.consume_if_starts_with("+") {
            GoTokenType::Plus
        }
        else if state.consume_if_starts_with("-") {
            GoTokenType::Minus
        }
        else if state.consume_if_starts_with("*") {
            GoTokenType::Star
        }
        else if state.consume_if_starts_with("/") {
            GoTokenType::Slash
        }
        else if state.consume_if_starts_with("%") {
            GoTokenType::Percent
        }
        else if state.consume_if_starts_with("&") {
            GoTokenType::Ampersand
        }
        else if state.consume_if_starts_with("|") {
            GoTokenType::Pipe
        }
        else if state.consume_if_starts_with("^") {
            GoTokenType::Caret
        }
        else if state.consume_if_starts_with("!") {
            GoTokenType::LogicalNot
        }
        else if state.consume_if_starts_with("<") {
            GoTokenType::Less
        }
        else if state.consume_if_starts_with(">") {
            GoTokenType::Greater
        }
        else if state.consume_if_starts_with("=") {
            GoTokenType::Assign
        }
        else {
            return false;
        };

        state.add_token(kind, start, state.get_position());
        true
    }
}
