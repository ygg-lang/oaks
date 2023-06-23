#![doc = include_str!("readme.md")]
pub mod token_type;
pub use token_type::CobolTokenType;

use crate::language::CobolLanguage;
use oak_core::{
    Lexer, LexerCache, LexerState, OakError,
    lexer::LexOutput,
    source::{Source, TextEdit},
};

type State<'a, S> = LexerState<'a, S, CobolLanguage>;

pub struct CobolLexer<'config> {
    _config: &'config CobolLanguage,
}

impl<'config> Lexer<CobolLanguage> for CobolLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[TextEdit], mut cache: &'a mut impl LexerCache<CobolLanguage>) -> LexOutput<CobolLanguage> {
        let mut state: State<'_, S> = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, &mut cache)
    }
}

impl<'config> CobolLexer<'config> {
    pub fn new(config: &'config CobolLanguage) -> Self {
        Self { _config: config }
    }

    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            // 尝试各种词法规则
            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_newline(state) {
                continue;
            }

            if self.lex_comment(state) {
                continue;
            }

            if self.lex_string(state) {
                continue;
            }

            if self.lex_number(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_operator_or_delimiter(state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(CobolTokenType::Error, start_pos, state.get_position())
            }

            state.advance_if_dead_lock(safe_point)
        }

        Ok(())
    }

    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_whitespace() && ch != '\n' && ch != '\r' {
                let start_pos = state.get_position();
                while let Some(ch) = state.peek() {
                    if ch.is_whitespace() && ch != '\n' && ch != '\r' { state.advance(ch.len_utf8()) } else { break }
                }
                state.add_token(CobolTokenType::Whitespace, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch == '\n' || ch == '\r' {
                let start_pos = state.get_position();
                state.advance(ch.len_utf8());
                if ch == '\r' && state.peek() == Some('\n') {
                    state.advance(1)
                }
                state.add_token(CobolTokenType::Newline, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut LexerState<'a, S, CobolLanguage>) -> bool {
        // COBOL 注释通常在第7列（索引6），但我们这里简化处理
        if state.peek() == Some('*') || state.peek() == Some('/') {
            let start_pos = state.get_position();
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8())
            }
            state.add_token(CobolTokenType::Comment, start_pos, state.get_position());
            return true;
        }
        false
    }

    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut LexerState<'a, S, CobolLanguage>) -> bool {
        if let Some(quote) = state.peek() {
            if quote == '"' || quote == '\'' {
                let start_pos = state.get_position();
                state.advance(1);
                while let Some(ch) = state.peek() {
                    if ch == quote {
                        state.advance(1);
                        // 处理双引号转义
                        if state.peek() == Some(quote) { state.advance(1) } else { break }
                    }
                    else if ch == '\n' || ch == '\r' {
                        break;
                    }
                    else {
                        state.advance(ch.len_utf8())
                    }
                }
                state.add_token(CobolTokenType::StringLiteral, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut LexerState<'a, S, CobolLanguage>) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() || ch == '.' {
                let start_pos = state.get_position();
                let mut has_dot = false;
                if ch == '.' {
                    has_dot = true;
                    state.advance(1);
                    if !state.peek().map_or(false, |c| c.is_ascii_digit()) {
                        state.set_position(start_pos);
                        return false;
                    }
                }

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1)
                    }
                    else if ch == '.' && !has_dot {
                        has_dot = true;
                        state.advance(1)
                    }
                    else {
                        break;
                    }
                }
                state.add_token(CobolTokenType::NumberLiteral, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut LexerState<'a, S, CobolLanguage>) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() {
                let start_pos = state.get_position();
                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '-' { state.advance(1) } else { break }
                }
                let end_pos = state.get_position();
                let text = state.get_text_in((start_pos..end_pos).into()).to_uppercase();

                let kind = match text.as_str() {
                    "ACCEPT" => CobolTokenType::Accept,
                    "ADD" => CobolTokenType::Add,
                    "CALL" => CobolTokenType::Call,
                    "CANCEL" => CobolTokenType::Cancel,
                    "CLOSE" => CobolTokenType::Close,
                    "COMPUTE" => CobolTokenType::Compute,
                    "CONTINUE" => CobolTokenType::Continue,
                    "DELETE" => CobolTokenType::Delete,
                    "DISPLAY" => CobolTokenType::Display,
                    "DIVIDE" => CobolTokenType::Divide,
                    "EVALUATE" => CobolTokenType::Evaluate,
                    "EXIT" => CobolTokenType::Exit,
                    "GO" => {
                        if state.peek() == Some(' ') {
                            // 检查是否是 GO TO
                            let save = state.get_position();
                            state.advance(1);
                            while state.peek() == Some(' ') {
                                state.advance(1)
                            }
                            let next_start = state.get_position();
                            while let Some(c) = state.peek() {
                                if c.is_alphanumeric() { state.advance(1) } else { break }
                            }
                            if state.get_text_in((next_start..state.get_position()).into()).to_uppercase() == "TO" {
                                CobolTokenType::GoTo
                            }
                            else {
                                state.set_position(save);
                                CobolTokenType::Identifier
                            }
                        }
                        else {
                            CobolTokenType::Identifier
                        }
                    }
                    "IF" => CobolTokenType::If,
                    "INITIALIZE" => CobolTokenType::Initialize,
                    "INSPECT" => CobolTokenType::Inspect,
                    "MOVE" => CobolTokenType::Move,
                    "MULTIPLY" => CobolTokenType::Multiply,
                    "OPEN" => CobolTokenType::Open,
                    "PERFORM" => CobolTokenType::Perform,
                    "READ" => CobolTokenType::Read,
                    "RETURN" => CobolTokenType::Return,
                    "REWRITE" => CobolTokenType::Rewrite,
                    "SEARCH" => CobolTokenType::Search,
                    "SET" => CobolTokenType::Set,
                    "SORT" => CobolTokenType::Sort,
                    "START" => CobolTokenType::Start,
                    "STOP" => CobolTokenType::Stop,
                    "STRING" => CobolTokenType::String,
                    "SUBTRACT" => CobolTokenType::Subtract,
                    "UNSTRING" => CobolTokenType::Unstring,
                    "WRITE" => CobolTokenType::Write,
                    "DATA" => CobolTokenType::Data,
                    "DIVISION" => CobolTokenType::Division,
                    "SECTION" => CobolTokenType::Section,
                    "WORKING-STORAGE" => CobolTokenType::WorkingStorage,
                    "FILE-CONTROL" => CobolTokenType::File, // 简化
                    "PROCEDURE" => CobolTokenType::Procedure,
                    "PROGRAM-ID" => CobolTokenType::Program,
                    "IDENTIFICATION" => CobolTokenType::Identification,
                    "ENVIRONMENT" => CobolTokenType::Environment,
                    "CONFIGURATION" => CobolTokenType::Configuration,
                    "INPUT-OUTPUT" => CobolTokenType::InputOutput,
                    "PIC" => CobolTokenType::Pic,
                    "PICTURE" => CobolTokenType::Picture,
                    "VALUE" => CobolTokenType::Value,
                    "OCCURS" => CobolTokenType::Occurs,
                    "REDEFINES" => CobolTokenType::Redefines,
                    "USAGE" => CobolTokenType::Usage,
                    "COMP" => CobolTokenType::Comp,
                    "BINARY" => CobolTokenType::Binary,
                    "PACKED-DECIMAL" => CobolTokenType::Packed,
                    "AND" => CobolTokenType::And,
                    "OR" => CobolTokenType::Or,
                    "NOT" => CobolTokenType::Not,
                    "EQUAL" => CobolTokenType::Equal,
                    "GREATER" => CobolTokenType::Greater,
                    "LESS" => CobolTokenType::Less,
                    _ => CobolTokenType::Identifier,
                };

                state.add_token(kind, start_pos, end_pos);
                return true;
            }
        }
        false
    }

    fn lex_operator_or_delimiter<'a, S: Source + ?Sized>(&self, state: &mut LexerState<'a, S, CobolLanguage>) -> bool {
        if let Some(ch) = state.peek() {
            let start_pos = state.get_position();
            let kind = match ch {
                '+' => CobolTokenType::Plus,
                '-' => CobolTokenType::Minus,
                '*' => {
                    state.advance(1);
                    if state.peek() == Some('*') {
                        state.advance(1);
                        state.add_token(CobolTokenType::Power, start_pos, state.get_position());
                        return true;
                    }
                    CobolTokenType::Star
                }
                '/' => CobolTokenType::Slash,
                '=' => CobolTokenType::EqualSign,
                '>' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        state.add_token(CobolTokenType::GreaterEqual, start_pos, state.get_position());
                        return true;
                    }
                    CobolTokenType::GreaterThan
                }
                '<' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        state.add_token(CobolTokenType::LessEqual, start_pos, state.get_position());
                        return true;
                    }
                    if state.peek() == Some('>') {
                        state.advance(1);
                        state.add_token(CobolTokenType::NotEqual, start_pos, state.get_position());
                        return true;
                    }
                    CobolTokenType::LessThan
                }
                '(' => CobolTokenType::LeftParen,
                ')' => CobolTokenType::RightParen,
                ',' => CobolTokenType::Comma,
                '.' => CobolTokenType::Period,
                ';' => CobolTokenType::Semicolon,
                ':' => CobolTokenType::Colon,
                _ => return false,
            };

            state.advance(1);
            state.add_token(kind, start_pos, state.get_position());
            return true;
        }
        false
    }
}
