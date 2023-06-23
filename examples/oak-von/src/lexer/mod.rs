#![doc = include_str!("readme.md")]
use oak_core::{
    Lexer, LexerState, Source, TextEdit,
    lexer::{LexOutput, LexerCache},
};

pub mod token_type;
use crate::language::VonLanguage;
pub use token_type::{VonToken, VonTokenType};

type State<'a, S> = LexerState<'a, S, VonLanguage>;

#[derive(Clone, Debug)]
pub struct VonLexer<'config> {
    _config: &'config VonLanguage,
}

impl<'config> VonLexer<'config> {
    pub fn new(config: &'config VonLanguage) -> Self {
        Self { _config: config }
    }

    /// 跳过空白字符
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        if state.get_position() > start_pos {
            state.add_token(VonTokenType::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(VonTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(VonTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        // 单行注释 #
        if let Some('#') = state.peek() {
            state.advance(1);

            // 读取到行尾
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(VonTokenType::Comment, start_pos, state.get_position());
            return true;
        }
        false
    }

    /// 处理对称引号字符串字面量或原始字符串
    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        // 检查原始字符串 raw"..."
        let mut is_raw = false;
        if let Some('r') = state.peek() {
            if let Some('a') = state.peek_next_n(1) {
                if let Some('w') = state.peek_next_n(2) {
                    if let Some(c) = state.peek_next_n(3) {
                        if c == '"' || c == '\'' {
                            is_raw = true;
                            // 注意：这里不要直接 advance，而是让后面的逻辑处理引号
                        }
                    }
                }
            }
        }

        let quote = if is_raw {
            state.peek_next_n(3).unwrap()
        }
        else {
            match state.peek() {
                Some(c) if c == '"' || c == '\'' => c,
                _ => return false,
            }
        };

        if is_raw {
            state.advance(3);
        }

        let mut quote_count = 0;
        while let Some(c) = state.peek() {
            if c == quote {
                quote_count += 1;
                state.advance(c.len_utf8());
            }
            else {
                break;
            }
        }

        // "" 或 '' 是空字符串
        if quote_count == 2 {
            state.add_token(VonTokenType::StringLiteral, start, state.get_position());
            return true;
        }

        if quote_count == 0 {
            state.set_position(start);
            return false;
        }

        let mut current_consecutive = 0;
        let mut escaped = false;

        while let Some(c) = state.peek() {
            if !is_raw && escaped {
                escaped = false;
                state.advance(c.len_utf8());
                current_consecutive = 0;
                continue;
            }

            if !is_raw && c == '\\' && quote_count == 1 {
                escaped = true;
                state.advance(1);
                current_consecutive = 0;
                continue;
            }

            if c == quote {
                current_consecutive += 1;
                state.advance(c.len_utf8());
                if current_consecutive == quote_count {
                    state.add_token(VonTokenType::StringLiteral, start, state.get_position());
                    return true;
                }
            }
            else {
                current_consecutive = 0;
                state.advance(c.len_utf8());
            }
        }

        // 未闭合的字符串，标记为错误以提醒用户，但在语法高亮中仍可视为字符串
        state.add_token(VonTokenType::Error, start, state.get_position());
        true
    }

    /// 处理数字字面量
    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            // eprintln!("lex_number peeks '{}' at {}", ch, start_pos);
            // 数字必须以数字、负号或小数点（后面跟数字）开始
            let is_number_start = ch.is_ascii_digit() || (ch == '-' && state.peek_next_n(1).map_or(false, |c| c.is_ascii_digit())) || (ch == '.' && state.peek_next_n(1).map_or(false, |c| c.is_ascii_digit()));

            if !is_number_start {
                return false;
            }

            if ch == '-' {
                state.advance(1);
            }

            // 整数部分
            if let Some(first) = state.peek() {
                if first.is_ascii_digit() {
                    while let Some(digit) = state.peek() {
                        if digit.is_ascii_digit() || digit == '_' {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }
            }

            // 检查小数点
            if let Some('.') = state.peek() {
                let mut lookahead = 1;
                while let Some(c) = state.peek_next_n(lookahead) {
                    if c == '_' {
                        lookahead += 1;
                    }
                    else {
                        break;
                    }
                }
                if let Some(next_ch) = state.peek_next_n(lookahead) {
                    if next_ch.is_ascii_digit() {
                        state.advance(1); // 跳过小数点
                        while let Some(digit) = state.peek() {
                            if digit.is_ascii_digit() || digit == '_' {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                    }
                }
            }

            // 检查指数
            if let Some(e) = state.peek() {
                if e == 'e' || e == 'E' {
                    // 确保指数后面跟着数字（或符号+数字）
                    let mut lookahead = 1;
                    if let Some(sign) = state.peek_next_n(lookahead) {
                        if sign == '+' || sign == '-' {
                            lookahead += 1;
                        }
                    }

                    let has_digits = state.peek_next_n(lookahead).map_or(false, |c| c.is_ascii_digit() || (c == '_' && state.peek_next_n(lookahead + 1).map_or(false, |n| n.is_ascii_digit())));

                    if has_digits {
                        state.advance(1); // 跳过 e/E

                        // 可选的符号
                        if let Some(sign) = state.peek() {
                            if sign == '+' || sign == '-' {
                                state.advance(1);
                            }
                        }

                        // 指数数字
                        while let Some(digit) = state.peek() {
                            if digit.is_ascii_digit() || digit == '_' {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                    }
                }
            }

            // 只有当至少消费了一个数字或者是负号后跟数字时，才认为是数字
            // 还要检查后面不能直接跟字母，否则可能是标识符（如 version）
            if state.get_position() > start_pos {
                if let Some(next) = state.peek() {
                    if next.is_ascii_alphabetic() || next == '_' {
                        state.set_position(start_pos);
                        return false;
                    }
                }
                state.add_token(VonTokenType::NumberLiteral, start_pos, state.get_position());
                return true;
            }
            false
        }
        else {
            false
        }
    }

    /// 处理标识符和关键字
    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                // 如果是 'r'，可能是 'raw'，需要检查是否是原始字符串的开始
                if ch == 'r' {
                    if let Some('a') = state.peek_next_n(1) {
                        if let Some('w') = state.peek_next_n(2) {
                            if let Some(c) = state.peek_next_n(3) {
                                if c == '"' || c == '\'' {
                                    // 这是原始字符串，由 lex_string 处理
                                    return false;
                                }
                            }
                        }
                    }
                }

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = state.get_text_in((start_pos..state.get_position()).into());
                let token_kind = match text.as_ref() {
                    "true" | "false" => VonTokenType::BoolLiteral,
                    "null" => VonTokenType::NullLiteral,
                    _ => VonTokenType::Identifier,
                };

                state.add_token(token_kind, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理操作符和标点符号
    fn lex_operator<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '[' => {
                    state.advance(1);
                    VonTokenType::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    VonTokenType::RightBracket
                }
                '{' => {
                    state.advance(1);
                    VonTokenType::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    VonTokenType::RightBrace
                }
                ',' => {
                    state.advance(1);
                    VonTokenType::Comma
                }
                ':' => {
                    state.advance(1);
                    VonTokenType::Colon
                }
                '=' => {
                    state.advance(1);
                    VonTokenType::Eq
                }
                _ => return false,
            };
            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<VonLanguage> for VonLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[TextEdit], _cache: &'a mut impl LexerCache<VonLanguage>) -> LexOutput<VonLanguage> {
        let mut state = State::new(source);
        while state.not_at_end() {
            if self.skip_whitespace(&mut state) {
                continue;
            }
            if self.lex_newline(&mut state) {
                continue;
            }
            if self.lex_comment(&mut state) {
                continue;
            }
            if self.lex_identifier_or_keyword(&mut state) {
                continue;
            }
            if self.lex_number(&mut state) {
                continue;
            }
            if self.lex_string(&mut state) {
                continue;
            }
            if self.lex_operator(&mut state) {
                continue;
            }

            // 如果都没有匹配，按错误处理并跳过一个字符
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(VonTokenType::Error, start_pos, state.get_position());
            }
            else {
                break;
            }
        }

        state.finish(Ok(()))
    }
}
