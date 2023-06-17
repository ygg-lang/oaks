use crate::{kind::PascalSyntaxKind, language::PascalLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, PascalLanguage>;

pub struct PascalLexer<'config> {
    config: &'config PascalLanguage,
}

impl<'config> PascalLexer<'config> {
    pub fn new(config: &'config PascalLanguage) -> Self {
        Self { config }
    }

    /// 跳过空白字符
    fn skip_whitespace(&self, state: &mut State) -> bool {
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
            state.add_token(PascalSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(PascalSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(PascalSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        // Pascal 风格注释 { ... }
        if let Some('{') = state.peek() {
            state.advance(1);
            while let Some(ch) = state.peek() {
                if ch == '}' {
                    state.advance(1);
                    break;
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }
            state.add_token(PascalSyntaxKind::Comment, start_pos, state.get_position());
            return true;
        }

        // (* ... *) 风格注释
        if let Some('(') = state.peek() {
            if let Some('*') = source.get_char_at(start_pos + 1) {
                state.advance(2);
                while let Some(ch) = state.peek() {
                    if ch == '*' {
                        if let Some(')') = source.get_char_at(state.get_position() + 1) {
                            state.advance(2);
                            break;
                        }
                        else {
                            state.advance(1);
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }
                state.add_token(PascalSyntaxKind::Comment, start_pos, state.get_position());
                return true;
            }
        }

        // // 风格注释
        if let Some('/') = state.peek() {
            if let Some('/') = source.get_char_at(start_pos + 1) {
                state.advance(2);
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }
                state.add_token(PascalSyntaxKind::Comment, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理字符串字面量
    fn lex_string_literal(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('\'') = state.peek() {
            state.advance(1);
            let mut found_end = false;

            while let Some(ch) = state.peek() {
                if ch == '\'' {
                    state.advance(1);
                    // 检查是否是转义的单引号 ''
                    if let Some('\'') = state.peek() {
                        state.advance(1);
                    }
                    else {
                        found_end = true;
                        break;
                    }
                }
                else if ch == '\n' || ch == '\r' {
                    break; // 字符串不能跨
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            if found_end {
                state.add_token(PascalSyntaxKind::StringLiteral, start_pos, state.get_position());
                true
            }
            else {
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// 处理数字字面
    fn lex_number_literal(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                // 整数部分
                while let Some(digit) = state.peek() {
                    if digit.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                let mut is_real = false;

                // 检查小数点
                if let Some('.') = state.peek() {
                    // 确保不是范围操作..
                    let next_pos = state.get_position() + 1;
                    if let Some(next_ch) = source.get_char_at(next_pos) {
                        if next_ch != '.' && next_ch.is_ascii_digit() {
                            state.advance(1);
                            is_real = true;

                            // 小数部分
                            while let Some(digit) = state.peek() {
                                if digit.is_ascii_digit() {
                                    state.advance(1);
                                }
                                else {
                                    break;
                                }
                            }
                        }
                    }
                }

                // 检查科学计数法
                if let Some(e_char) = state.peek() {
                    if e_char == 'e' || e_char == 'E' {
                        let saved_pos = state.get_position();
                        state.advance(1);

                        // 可选的符号
                        if let Some(sign) = state.peek() {
                            if sign == '+' || sign == '-' {
                                state.advance(1);
                            }
                        }

                        // 指数部分
                        let exp_start = state.get_position();
                        while let Some(digit) = state.peek() {
                            if digit.is_ascii_digit() {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }

                        if state.get_position() > exp_start {
                            is_real = true;
                        }
                        else {
                            // 没有有效的指数，回退
                            state.set_position(saved_pos);
                        }
                    }
                }

                let token_kind = if is_real { PascalSyntaxKind::RealLiteral } else { PascalSyntaxKind::IntegerLiteral };
                state.add_token(token_kind, start_pos, state.get_position());
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

    /// 处理标识符和关键字
    fn lex_identifier_or_keyword(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = source.get_text_in((start_pos..state.get_position()).into()).unwrap_or("");
                let token_kind = match text.to_lowercase().as_str() {
                    "program" => PascalSyntaxKind::Program,
                    "begin" => PascalSyntaxKind::Begin,
                    "end" => PascalSyntaxKind::End,
                    "var" => PascalSyntaxKind::Var,
                    "const" => PascalSyntaxKind::Const,
                    "type" => PascalSyntaxKind::Type,
                    "function" => PascalSyntaxKind::Function,
                    "procedure" => PascalSyntaxKind::Procedure,
                    "if" => PascalSyntaxKind::If,
                    "then" => PascalSyntaxKind::Then,
                    "else" => PascalSyntaxKind::Else,
                    "while" => PascalSyntaxKind::While,
                    "do" => PascalSyntaxKind::Do,
                    "for" => PascalSyntaxKind::For,
                    "to" => PascalSyntaxKind::To,
                    "downto" => PascalSyntaxKind::Downto,
                    "repeat" => PascalSyntaxKind::Repeat,
                    "until" => PascalSyntaxKind::Until,
                    "case" => PascalSyntaxKind::Case,
                    "of" => PascalSyntaxKind::Of,
                    "with" => PascalSyntaxKind::With,
                    "record" => PascalSyntaxKind::Record,
                    "array" => PascalSyntaxKind::Array,
                    "set" => PascalSyntaxKind::Set,
                    "file" => PascalSyntaxKind::File,
                    "packed" => PascalSyntaxKind::Packed,
                    "nil" => PascalSyntaxKind::Nil,
                    "true" => PascalSyntaxKind::True,
                    "false" => PascalSyntaxKind::False,
                    "and" => PascalSyntaxKind::And,
                    "or" => PascalSyntaxKind::Or,
                    "not" => PascalSyntaxKind::Not,
                    "div" => PascalSyntaxKind::Div,
                    "mod" => PascalSyntaxKind::Mod,
                    "in" => PascalSyntaxKind::In,
                    _ => PascalSyntaxKind::Identifier,
                };

                state.add_token(token_kind, start_pos, state.get_position());
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

    /// 处理运算
    fn lex_operators(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '+' => {
                    state.advance(1);
                    PascalSyntaxKind::Plus
                }
                '-' => {
                    state.advance(1);
                    PascalSyntaxKind::Minus
                }
                '*' => {
                    state.advance(1);
                    PascalSyntaxKind::Multiply
                }
                '/' => {
                    state.advance(1);
                    PascalSyntaxKind::Divide
                }
                '=' => {
                    state.advance(1);
                    PascalSyntaxKind::Equal
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PascalSyntaxKind::LessEqual
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        PascalSyntaxKind::NotEqual
                    }
                    else {
                        PascalSyntaxKind::Less
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PascalSyntaxKind::GreaterEqual
                    }
                    else {
                        PascalSyntaxKind::Greater
                    }
                }
                ':' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PascalSyntaxKind::Assign
                    }
                    else {
                        PascalSyntaxKind::Colon
                    }
                }
                '.' => {
                    state.advance(1);
                    if let Some('.') = state.peek() {
                        state.advance(1);
                        PascalSyntaxKind::Range
                    }
                    else {
                        PascalSyntaxKind::Dot
                    }
                }
                '^' => {
                    state.advance(1);
                    PascalSyntaxKind::Caret
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

    /// 处理分隔
    fn lex_delimiters(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => PascalSyntaxKind::LeftParen,
                ')' => PascalSyntaxKind::RightParen,
                '[' => PascalSyntaxKind::LeftBracket,
                ']' => PascalSyntaxKind::RightBracket,
                ';' => PascalSyntaxKind::Semicolon,
                ',' => PascalSyntaxKind::Comma,
                _ => return false,
            };

            state.advance(ch.len_utf8());
            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<PascalLanguage> for PascalLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<PascalSyntaxKind> {
        let mut state = LexerState::new(source);

        while state.not_at_end() {
            // 尝试各种词法规则
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_comment(&mut state, source) {
                continue;
            }

            if self.lex_string_literal(&mut state) {
                continue;
            }

            if self.lex_number_literal(&mut state, source) {
                continue;
            }

            if self.lex_identifier_or_keyword(&mut state, source) {
                continue;
            }

            if self.lex_operators(&mut state, source) {
                continue;
            }

            if self.lex_delimiters(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(PascalSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(PascalSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
