#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::PascalLanguage, lexer::token_type::PascalTokenType};
use oak_core::{
    Lexer, LexerCache, LexerState, OakError,
    lexer::{CommentConfig, LexOutput, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<'s, S> = LexerState<'s, S, PascalLanguage>;

static PASCAL_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static PASCAL_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: "//", block_start: "{", block_end: "}", nested_blocks: false });

#[derive(Clone, Debug)]
pub struct PascalLexer<'config> {
    _config: &'config PascalLanguage,
}

impl<'config> PascalLexer<'config> {
    pub fn new(config: &'config PascalLanguage) -> Self {
        Self { _config: config }
    }

    fn skip_whitespace<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        PASCAL_WHITESPACE.scan(state, PascalTokenType::Whitespace)
    }

    fn skip_comment<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();

        // Line comment starting with //
        if state.rest().starts_with("//") {
            return PASCAL_COMMENT.scan(state, PascalTokenType::Comment, PascalTokenType::Comment);
        }

        // Block comment: { ... }
        if state.current() == Some('{') {
            state.advance(1);
            while let Some(ch) = state.peek() {
                if ch == '}' {
                    state.advance(1);
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(PascalTokenType::Comment, start, state.get_position());
            return true;
        }

        // Block comment: (* ... *)
        if state.rest().starts_with("(*") {
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch == '*' && state.peek_next_n(1) == Some(')') {
                    state.advance(2);
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(PascalTokenType::Comment, start, state.get_position());
            return true;
        }

        false
    }

    fn lex_string<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();

        // Pascal 字符串字面量：'...'
        if state.current() == Some('\'') {
            state.advance(1);
            while let Some(ch) = state.peek() {
                if ch == '\'' {
                    // 检查是否是转义的单引号 ''
                    if state.peek_next_n(1) == Some('\'') {
                        state.advance(2); // 跳过 ''
                        continue;
                    }
                    else {
                        state.advance(1); // 结束引号
                        break;
                    }
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(PascalTokenType::StringLiteral, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_identifier_or_keyword<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                let start_pos = state.get_position();
                let mut text = String::new();

                // 读取标识符
                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        text.push(ch);
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 检查是否是关键字
                let kind = match text.to_lowercase().as_str() {
                    "program" => PascalTokenType::Program,
                    "var" => PascalTokenType::Var,
                    "const" => PascalTokenType::Const,
                    "type" => PascalTokenType::Type,
                    "procedure" => PascalTokenType::Procedure,
                    "function" => PascalTokenType::Function,
                    "begin" => PascalTokenType::Begin,
                    "end" => PascalTokenType::End,
                    "if" => PascalTokenType::If,
                    "then" => PascalTokenType::Then,
                    "else" => PascalTokenType::Else,
                    "while" => PascalTokenType::While,
                    "do" => PascalTokenType::Do,
                    "for" => PascalTokenType::For,
                    "to" => PascalTokenType::To,
                    "downto" => PascalTokenType::Downto,
                    "repeat" => PascalTokenType::Repeat,
                    "until" => PascalTokenType::Until,
                    "case" => PascalTokenType::Case,
                    "of" => PascalTokenType::Of,
                    "with" => PascalTokenType::With,
                    "record" => PascalTokenType::Record,
                    "array" => PascalTokenType::Array,
                    "set" => PascalTokenType::Set,
                    "file" => PascalTokenType::File,
                    "packed" => PascalTokenType::Packed,
                    "nil" => PascalTokenType::Nil,
                    "true" => PascalTokenType::True,
                    "false" => PascalTokenType::False,
                    "and" => PascalTokenType::And,
                    "or" => PascalTokenType::Or,
                    "not" => PascalTokenType::Not,
                    "div" => PascalTokenType::Div,
                    "mod" => PascalTokenType::Mod,
                    "in" => PascalTokenType::In,

                    _ => PascalTokenType::Identifier,
                };

                state.add_token(kind, start_pos, state.get_position());
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    fn lex_number<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                let start_pos = state.get_position();
                let mut has_dot = false;

                // 读取数字
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else if ch == '.' && !has_dot {
                        has_dot = true;
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                let kind = if has_dot { PascalTokenType::RealLiteral } else { PascalTokenType::IntegerLiteral };

                state.add_token(kind, start_pos, state.get_position());
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    fn lex_operators_and_punctuation<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        if let Some(ch) = state.peek() {
            let start_pos = state.get_position();

            let kind = match ch {
                '+' => {
                    state.advance(1);
                    PascalTokenType::Plus
                }
                '-' => {
                    state.advance(1);
                    PascalTokenType::Minus
                }
                '*' => {
                    state.advance(1);
                    PascalTokenType::Multiply
                }
                '/' => {
                    state.advance(1);
                    PascalTokenType::Divide
                }
                '=' => {
                    state.advance(1);
                    PascalTokenType::Equal
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PascalTokenType::LessEqual
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        PascalTokenType::NotEqual
                    }
                    else {
                        PascalTokenType::Less
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PascalTokenType::GreaterEqual
                    }
                    else {
                        PascalTokenType::Greater
                    }
                }
                ':' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PascalTokenType::Assign
                    }
                    else {
                        PascalTokenType::Colon
                    }
                }
                ';' => {
                    state.advance(1);
                    PascalTokenType::Semicolon
                }
                ',' => {
                    state.advance(1);
                    PascalTokenType::Comma
                }
                '.' => {
                    state.advance(1);
                    if let Some('.') = state.peek() {
                        state.advance(1);
                        PascalTokenType::Range
                    }
                    else {
                        PascalTokenType::Dot
                    }
                }
                '(' => {
                    state.advance(1);
                    PascalTokenType::LeftParen
                }
                ')' => {
                    state.advance(1);
                    PascalTokenType::RightParen
                }
                '[' => {
                    state.advance(1);
                    PascalTokenType::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    PascalTokenType::RightBracket
                }
                '^' => {
                    state.advance(1);
                    PascalTokenType::Caret
                }
                '\n' => {
                    state.advance(1);
                    PascalTokenType::Newline
                }
                _ => {
                    state.advance(ch.len_utf8());
                    PascalTokenType::Error
                }
            };

            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl Lexer<PascalLanguage> for PascalLexer<'_> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl LexerCache<PascalLanguage>) -> LexOutput<PascalLanguage> {
        let mut state = State::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl PascalLexer<'_> {
    fn run<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> Result<(), OakError> {
        let safe_point = state.get_position();
        while state.not_at_end() {
            // 跳过空白字符
            if self.skip_whitespace(state) {
                continue;
            }

            // 处理注释
            if self.skip_comment(state) {
                continue;
            }

            // 处理字符串
            if self.lex_string(state) {
                continue;
            }

            // 处理标识符和关键字
            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            // 处理数字
            if self.lex_number(state) {
                continue;
            }

            // 处理操作符和标点符号
            if self.lex_operators_and_punctuation(state) {
                continue;
            }

            // 如果没有匹配任何模式，创建错误 token
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(PascalTokenType::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point);
        }

        // 添加 EOF token
        Ok(())
    }
}
