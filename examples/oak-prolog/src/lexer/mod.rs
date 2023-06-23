#![doc = include_str!("readme.md")]
pub mod token_type;
pub use token_type::PrologTokenType;

use crate::language::PrologLanguage;
use oak_core::{Lexer, LexerCache, LexerState, OakError, lexer::LexOutput, source::Source};

type State<'s, S> = LexerState<'s, S, PrologLanguage>;

#[derive(Clone, Debug)]
pub struct PrologLexer<'config> {
    _config: &'config PrologLanguage,
}

impl<'config> PrologLexer<'config> {
    pub fn new(config: &'config PrologLanguage) -> Self {
        Self { _config: config }
    }

    fn run<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

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

            if self.lex_atom_or_keyword(state) {
                continue;
            }

            if self.lex_variable(state) {
                continue;
            }

            if self.lex_operators_and_punctuation(state) {
                continue;
            }

            // 如果没有匹配任何规则，跳过当前字符
            if let Some(ch) = state.peek() {
                let start_pos = state.get_position();
                state.advance(ch.len_utf8());
                state.add_token(PrologTokenType::Error, start_pos, state.get_position())
            }

            state.advance_if_dead_lock(safe_point)
        }

        Ok(())
    }

    fn skip_whitespace<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' { state.advance(ch.len_utf8()) } else { break }
        }

        if state.get_position() > start_pos {
            state.add_token(PrologTokenType::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_newline<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(PrologTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1)
            }
            state.add_token(PrologTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_comment<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('%') = state.peek() {
            state.advance(1);
            // 单行注释
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8())
            }
            state.add_token(PrologTokenType::Comment, start_pos, state.get_position());
            true
        }
        else if let Some('/') = state.peek() {
            state.advance(1);
            if let Some('*') = state.peek() {
                state.advance(1);
                // 多行注释 /* ... */
                while let Some(ch) = state.peek() {
                    if ch == '*' {
                        state.advance(1);
                        if let Some('/') = state.peek() {
                            state.advance(1);
                            break;
                        }
                    }
                    else {
                        state.advance(ch.len_utf8())
                    }
                }
                state.add_token(PrologTokenType::Comment, start_pos, state.get_position());
                true
            }
            else {
                // 回退，这不是注释
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    fn lex_string<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(quote_char) = state.peek() {
            if quote_char == '"' || quote_char == '\'' {
                state.advance(1); // 跳过开始引号

                let mut escaped = false;
                while let Some(ch) = state.peek() {
                    if escaped {
                        escaped = false;
                        state.advance(ch.len_utf8())
                    }
                    else if ch == '\\' {
                        escaped = true;
                        state.advance(1)
                    }
                    else if ch == quote_char {
                        state.advance(1); // 跳过结束引号
                        break;
                    }
                    else if ch == '\n' || ch == '\r' {
                        // 字符串不能跨行
                        break;
                    }
                    else {
                        state.advance(ch.len_utf8())
                    }
                }

                state.add_token(PrologTokenType::String, start_pos, state.get_position());
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

                // 读取整数部分
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() { state.advance(1) } else { break }
                }

                // 检查小数点
                if let Some('.') = state.peek() {
                    state.advance(1);
                    // 读取小数部分
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() { state.advance(1) } else { break }
                    }
                }

                // 检查科学记数法
                if let Some(ch) = state.peek() {
                    if ch == 'e' || ch == 'E' {
                        state.advance(1);
                        if let Some(ch) = state.peek() {
                            if ch == '+' || ch == '-' {
                                state.advance(1)
                            }
                        }
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() { state.advance(1) } else { break }
                        }
                    }
                }

                state.add_token(PrologTokenType::Integer, start_pos, state.get_position());
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

    fn lex_atom_or_keyword<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_ascii_lowercase() || ch == '_' {
                let start_pos = state.get_position();
                let mut text = String::new();

                // 读取原子
                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        text.push(ch);
                        state.advance(ch.len_utf8())
                    }
                    else {
                        break;
                    }
                }

                // 检查是否是关键字
                let kind = match text.as_str() {
                    "is" => PrologTokenType::Is,
                    "mod" => PrologTokenType::Modulo,
                    _ => PrologTokenType::Atom,
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

    fn lex_variable<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_ascii_uppercase() || ch == '_' {
                let start_pos = state.get_position();

                // 读取变量名
                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' { state.advance(ch.len_utf8()) } else { break }
                }

                state.add_token(PrologTokenType::Variable, start_pos, state.get_position());
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
                    PrologTokenType::Plus
                }
                '-' => {
                    state.advance(1);
                    PrologTokenType::Minus
                }
                '*' => {
                    state.advance(1);
                    if let Some('*') = state.peek() {
                        state.advance(1);
                        PrologTokenType::Power
                    }
                    else {
                        PrologTokenType::Multiply
                    }
                }
                '/' => {
                    state.advance(1);
                    if let Some('/') = state.peek() {
                        state.advance(1);
                        PrologTokenType::IntDivide
                    }
                    else {
                        PrologTokenType::Divide
                    }
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PrologTokenType::Equal
                    }
                    else if let Some(':') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            PrologTokenType::ArithEqual
                        }
                        else {
                            // 回退
                            state.set_position(start_pos + 1);
                            PrologTokenType::Unify
                        }
                    }
                    else if let Some('\\') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            PrologTokenType::NotUnify
                        }
                        else {
                            // 回退
                            state.set_position(start_pos + 1);
                            PrologTokenType::Unify
                        }
                    }
                    else if let Some('<') = state.peek() {
                        state.advance(1);
                        PrologTokenType::ArithNotEqual
                    }
                    else {
                        PrologTokenType::Unify
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PrologTokenType::LessEqual
                    }
                    else {
                        PrologTokenType::Less
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PrologTokenType::GreaterEqual
                    }
                    else {
                        PrologTokenType::Greater
                    }
                }
                '\\' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            PrologTokenType::NotEqual
                        }
                        else {
                            PrologTokenType::NotUnify
                        }
                    }
                    else {
                        PrologTokenType::BitwiseNot
                    }
                }
                '!' => {
                    state.advance(1);
                    PrologTokenType::Cut
                }
                '?' => {
                    state.advance(1);
                    PrologTokenType::Question
                }
                ':' => {
                    state.advance(1);
                    if let Some('-') = state.peek() {
                        state.advance(1);
                        PrologTokenType::ColonMinus
                    }
                    else {
                        PrologTokenType::Colon
                    }
                }
                ';' => {
                    state.advance(1);
                    PrologTokenType::Semicolon
                }
                ',' => {
                    state.advance(1);
                    PrologTokenType::Comma
                }
                '.' => {
                    state.advance(1);
                    PrologTokenType::Dot
                }
                '(' => {
                    state.advance(1);
                    PrologTokenType::LeftParen
                }
                ')' => {
                    state.advance(1);
                    PrologTokenType::RightParen
                }
                '[' => {
                    state.advance(1);
                    PrologTokenType::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    PrologTokenType::RightBracket
                }
                '{' => {
                    state.advance(1);
                    PrologTokenType::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    PrologTokenType::RightBrace
                }
                '|' => {
                    state.advance(1);
                    PrologTokenType::Pipe
                }
                '^' => {
                    state.advance(1);
                    PrologTokenType::BitwiseXor
                }
                _ => return false,
            };

            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<PrologLanguage> for PrologLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl LexerCache<PrologLanguage>) -> LexOutput<PrologLanguage> {
        let mut state = State::new_with_cache(source, 0, cache);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}
