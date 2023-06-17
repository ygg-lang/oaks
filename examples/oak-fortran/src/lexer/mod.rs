use crate::{kind::FortranSyntaxKind, language::FortranLanguage};
use alloc::string::String;
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, FortranLanguage>;

pub struct FortranLexer<'config> {
    config: &'config FortranLanguage,
}

impl<'config> FortranLexer<'config> {
    pub fn new(config: &'config FortranLanguage) -> Self {
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
            state.add_token(FortranSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(FortranSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(FortranSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some('!') = state.peek() {
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(FortranSyntaxKind::Comment, start_pos, state.get_position());
            true
        }
        else if let Some('C') = state.peek() {
            // Fortran 77 风格注释（第一列的 C）
            if start_pos == 0 || source.get_char_at(start_pos - 1) == Some('\n') {
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }

                state.add_token(FortranSyntaxKind::Comment, start_pos, state.get_position());
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

    /// 处理标识符或关键
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

                let end_pos = state.get_position();
                let text = source.get_text_in((start_pos..end_pos).into()).unwrap_or("").to_lowercase();

                let token_kind = match text.as_str() {
                    "program" => FortranSyntaxKind::Program,
                    "end" => FortranSyntaxKind::End,
                    "subroutine" => FortranSyntaxKind::Subroutine,
                    "function" => FortranSyntaxKind::Function,
                    "integer" => FortranSyntaxKind::Integer,
                    "real" => FortranSyntaxKind::Real,
                    "double" => FortranSyntaxKind::Double,
                    "precision" => FortranSyntaxKind::Precision,
                    "complex" => FortranSyntaxKind::Complex,
                    "logical" => FortranSyntaxKind::Logical,
                    "character" => FortranSyntaxKind::Character,
                    "dimension" => FortranSyntaxKind::Dimension,
                    "parameter" => FortranSyntaxKind::Parameter,
                    "common" => FortranSyntaxKind::Common,
                    "equivalence" => FortranSyntaxKind::Equivalence,
                    "external" => FortranSyntaxKind::External,
                    "intrinsic" => FortranSyntaxKind::Intrinsic,
                    "save" => FortranSyntaxKind::Save,
                    "data" => FortranSyntaxKind::Data,
                    "implicit" => FortranSyntaxKind::Implicit,
                    "none" => FortranSyntaxKind::None,
                    "if" => FortranSyntaxKind::If,
                    "then" => FortranSyntaxKind::Then,
                    "else" => FortranSyntaxKind::Else,
                    "elseif" => FortranSyntaxKind::ElseIf,
                    "endif" => FortranSyntaxKind::EndIf,
                    "do" => FortranSyntaxKind::Do,
                    "enddo" => FortranSyntaxKind::EndDo,
                    "while" => FortranSyntaxKind::While,
                    "continue" => FortranSyntaxKind::Continue,
                    "stop" => FortranSyntaxKind::Stop,
                    "return" => FortranSyntaxKind::Return,
                    "call" => FortranSyntaxKind::Call,
                    "goto" => FortranSyntaxKind::Goto,
                    "assign" => FortranSyntaxKind::Assign,
                    "to" => FortranSyntaxKind::To,
                    "read" => FortranSyntaxKind::Read,
                    "write" => FortranSyntaxKind::Write,
                    "print" => FortranSyntaxKind::Print,
                    "open" => FortranSyntaxKind::Open,
                    "close" => FortranSyntaxKind::Close,
                    "format" => FortranSyntaxKind::Format,
                    "inquire" => FortranSyntaxKind::Inquire,
                    "backspace" => FortranSyntaxKind::Backspace,
                    "rewind" => FortranSyntaxKind::Rewind,
                    "endfile" => FortranSyntaxKind::EndFile,
                    "true" => FortranSyntaxKind::True,
                    "false" => FortranSyntaxKind::False,
                    _ => FortranSyntaxKind::Identifier,
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

    /// 处理数字
    fn lex_number(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(ch.len_utf8());

                // 处理整数部分
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 处理小数部分
                if let Some('.') = state.peek() {
                    let dot_pos = state.get_position();
                    state.advance(1);

                    if let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            while let Some(ch) = state.peek() {
                                if ch.is_ascii_digit() {
                                    state.advance(ch.len_utf8());
                                }
                                else {
                                    break;
                                }
                            }
                        }
                        else {
                            // 回退点号
                            state.set_position(dot_pos);
                        }
                    }
                    else {
                        // 回退点号
                        state.set_position(dot_pos);
                    }
                }

                // 处理科学计数
                if let Some(ch) = state.peek() {
                    if ch == 'e' || ch == 'E' || ch == 'd' || ch == 'D' {
                        state.advance(1);
                        if let Some(ch) = state.peek() {
                            if ch == '+' || ch == '-' {
                                state.advance(1);
                            }
                        }
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() {
                                state.advance(ch.len_utf8());
                            }
                            else {
                                break;
                            }
                        }
                    }
                }

                state.add_token(FortranSyntaxKind::Number, start_pos, state.get_position());
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

    /// 处理字符
    fn lex_string(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '"' || quote == '\'' {
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch == quote {
                        state.advance(1);
                        // 检查是否是双引号转义
                        if let Some(next_ch) = state.peek() {
                            if next_ch == quote {
                                state.advance(1);
                                continue;
                            }
                        }
                        state.add_token(FortranSyntaxKind::String, start_pos, state.get_position());
                        return true;
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                // 未闭合的字符                state.add_token(FortranSyntaxKind::Error, start_pos, state.get_position());
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

    /// 处理逻辑操作
    fn lex_logical_operator(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some('.') = state.peek() {
            let mut temp_pos = state.get_position() + 1;
            let mut operator_text = String::new();

            // 读取点号之间的内容
            while temp_pos < state.get_length() {
                if let Some(ch) = source.get_char_at(temp_pos) {
                    if ch == '.' {
                        // 找到结束的点号
                        let end_pos = temp_pos + 1;
                        let text = source.get_text_in((start_pos..end_pos).into()).unwrap_or("").to_lowercase();

                        let token_kind = match text.as_str() {
                            ".and." => FortranSyntaxKind::And,
                            ".or." => FortranSyntaxKind::Or,
                            ".not." => FortranSyntaxKind::Not,
                            ".eq." => FortranSyntaxKind::Equal,
                            ".ne." => FortranSyntaxKind::NotEqual,
                            ".lt." => FortranSyntaxKind::Less,
                            ".le." => FortranSyntaxKind::LessEqual,
                            ".gt." => FortranSyntaxKind::Greater,
                            ".ge." => FortranSyntaxKind::GreaterEqual,
                            ".eqv." => FortranSyntaxKind::Equivalent,
                            ".neqv." => FortranSyntaxKind::NotEquivalent,
                            ".true." => FortranSyntaxKind::True,
                            ".false." => FortranSyntaxKind::False,
                            _ => return false,
                        };

                        state.set_position(end_pos);
                        state.add_token(token_kind, start_pos, state.get_position());
                        return true;
                    }
                    else if ch.is_ascii_alphabetic() {
                        operator_text.push(ch);
                        temp_pos += 1;
                    }
                    else {
                        break;
                    }
                }
                else {
                    break;
                }
            }
        }

        false
    }

    /// 处理操作
    fn lex_operator(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            match ch {
                '+' => {
                    state.advance(1);
                    state.add_token(FortranSyntaxKind::Plus, start_pos, state.get_position());
                    true
                }
                '-' => {
                    state.advance(1);
                    state.add_token(FortranSyntaxKind::Minus, start_pos, state.get_position());
                    true
                }
                '*' => {
                    let next_pos = state.get_position() + 1;
                    if next_pos < state.get_length() {
                        let next_ch = source.get_char_at(next_pos);
                        if next_ch == Some('*') {
                            state.advance(1);
                            state.advance(1);
                            state.add_token(FortranSyntaxKind::Power, start_pos, state.get_position());
                            true
                        }
                        else {
                            state.advance(1);
                            state.add_token(FortranSyntaxKind::Star, start_pos, state.get_position());
                            true
                        }
                    }
                    else {
                        state.advance(1);
                        state.add_token(FortranSyntaxKind::Star, start_pos, state.get_position());
                        true
                    }
                }
                '/' => {
                    let next_pos = state.get_position() + 1;
                    if next_pos < state.get_length() {
                        let next_ch = source.get_char_at(next_pos);
                        if next_ch == Some('/') {
                            state.advance(1);
                            state.advance(1);
                            state.add_token(FortranSyntaxKind::Concatenate, start_pos, state.get_position());
                            true
                        }
                        else {
                            state.advance(1);
                            state.add_token(FortranSyntaxKind::Slash, start_pos, state.get_position());
                            true
                        }
                    }
                    else {
                        state.advance(1);
                        state.add_token(FortranSyntaxKind::Slash, start_pos, state.get_position());
                        true
                    }
                }
                '=' => {
                    let next_pos = state.get_position() + 1;
                    if next_pos < state.get_length() {
                        let next_ch = source.get_char_at(next_pos);
                        if next_ch == Some('=') {
                            state.advance(1);
                            state.advance(1);
                            state.add_token(FortranSyntaxKind::EqualEqual, start_pos, state.get_position());
                            true
                        }
                        else {
                            state.advance(1);
                            state.add_token(FortranSyntaxKind::Assign, start_pos, state.get_position());
                            true
                        }
                    }
                    else {
                        state.advance(1);
                        state.add_token(FortranSyntaxKind::Assign, start_pos, state.get_position());
                        true
                    }
                }
                '<' => {
                    let next_pos = state.get_position() + 1;
                    if next_pos < state.get_length() {
                        let next_ch = source.get_char_at(next_pos);
                        if next_ch == Some('=') {
                            state.advance(1);
                            state.advance(1);
                            state.add_token(FortranSyntaxKind::LessEqual, start_pos, state.get_position());
                            true
                        }
                        else {
                            state.advance(1);
                            state.add_token(FortranSyntaxKind::Less, start_pos, state.get_position());
                            true
                        }
                    }
                    else {
                        state.advance(1);
                        state.add_token(FortranSyntaxKind::Less, start_pos, state.get_position());
                        true
                    }
                }
                '>' => {
                    let next_pos = state.get_position() + 1;
                    if next_pos < state.get_length() {
                        let next_ch = source.get_char_at(next_pos);
                        if next_ch == Some('=') {
                            state.advance(1);
                            state.advance(1);
                            state.add_token(FortranSyntaxKind::GreaterEqual, start_pos, state.get_position());
                            true
                        }
                        else {
                            state.advance(1);
                            state.add_token(FortranSyntaxKind::Greater, start_pos, state.get_position());
                            true
                        }
                    }
                    else {
                        state.advance(1);
                        state.add_token(FortranSyntaxKind::Greater, start_pos, state.get_position());
                        true
                    }
                }
                _ => false,
            }
        }
        else {
            false
        }
    }

    /// 处理分隔
    fn lex_delimiter(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => FortranSyntaxKind::LeftParen,
                ')' => FortranSyntaxKind::RightParen,
                '[' => FortranSyntaxKind::LeftBracket,
                ']' => FortranSyntaxKind::RightBracket,
                ',' => FortranSyntaxKind::Comma,
                ':' => {
                    let next_pos = state.get_position() + 1;
                    if next_pos < state.get_length() {
                        let next_ch = source.get_char_at(next_pos);
                        if next_ch == Some(':') {
                            state.advance(1);
                            state.advance(1);
                            state.add_token(FortranSyntaxKind::DoubleColon, start_pos, state.get_position());
                            return true;
                        }
                        else {
                            FortranSyntaxKind::Colon
                        }
                    }
                    else {
                        FortranSyntaxKind::Colon
                    }
                }
                ';' => FortranSyntaxKind::Semicolon,
                '%' => FortranSyntaxKind::Percent,
                '&' => FortranSyntaxKind::Ampersand,
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

impl<'config> Lexer<FortranLanguage> for FortranLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<FortranSyntaxKind> {
        let mut state = State::new(source);

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

            if self.lex_identifier_or_keyword(&mut state, source) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_string(&mut state) {
                continue;
            }

            if self.lex_logical_operator(&mut state, source) {
                continue;
            }

            if self.lex_operator(&mut state, source) {
                continue;
            }

            if self.lex_delimiter(&mut state, source) {
                continue;
            }

            // 如果所有规则都不匹配，处理错误字符
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(FortranSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(FortranSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
