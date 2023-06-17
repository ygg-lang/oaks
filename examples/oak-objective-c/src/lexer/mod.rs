use crate::{kind::ObjectiveCLanguageSyntaxKind, language::ObjectiveCLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, ObjectiveCLanguage>;

pub struct ObjectiveCLexer<'config> {
    config: &'config ObjectiveCLanguage,
}

impl<'config> ObjectiveCLexer<'config> {
    pub fn new(config: &'config ObjectiveCLanguage) -> Self {
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
            state.add_token(ObjectiveCLanguageSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(ObjectiveCLanguageSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(ObjectiveCLanguageSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        // 单行注释 //
        if let Some('/') = state.peek() {
            if let Some('/') = state.peek_next_n(1) {
                state.advance(2);

                // 读取到行
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }

                state.add_token(ObjectiveCLanguageSyntaxKind::CommentToken, start_pos, state.get_position());
                return true;
            }
            // 多行注释 /* */
            else if let Some('*') = state.peek_next_n(1) {
                state.advance(2);

                while let Some(ch) = state.peek() {
                    if ch == '*' {
                        if let Some('/') = state.peek_next_n(1) {
                            state.advance(2);
                            break;
                        }
                    }
                    state.advance(ch.len_utf8());
                }

                state.add_token(ObjectiveCLanguageSyntaxKind::CommentToken, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理字符
    fn lex_string(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.advance(1); // 跳过开始引
            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1); // 跳过结束引号
                    break;
                }
                else if ch == '\\' {
                    state.advance(1); // 跳过转义字符
                    if state.peek().is_some() {
                        state.advance(state.peek().unwrap().len_utf8());
                    }
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            state.add_token(ObjectiveCLanguageSyntaxKind::String, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符字面
    fn lex_character(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('\'') = state.peek() {
            state.advance(1); // 跳过开始引
            while let Some(ch) = state.peek() {
                if ch == '\'' {
                    state.advance(1); // 跳过结束引号
                    break;
                }
                else if ch == '\\' {
                    state.advance(1); // 跳过转义字符
                    if state.peek().is_some() {
                        state.advance(state.peek().unwrap().len_utf8());
                    }
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            state.add_token(ObjectiveCLanguageSyntaxKind::Character, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理数字
    fn lex_number(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if !ch.is_ascii_digit() {
                return false;
            }

            // 处理整数部分
            while let Some(ch) = state.peek() {
                if ch.is_ascii_digit() {
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }

            // 处理小数
            if let Some('.') = state.peek() {
                if let Some(next_ch) = state.peek_next_n(1) {
                    if next_ch.is_ascii_digit() {
                        state.advance(1); // 跳过小数
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
            }

            // 处理科学计数
            if let Some(ch) = state.peek() {
                if ch == 'e' || ch == 'E' {
                    state.advance(1);
                    if let Some(sign) = state.peek() {
                        if sign == '+' || sign == '-' {
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

            // 处理后缀 (f, l, etc.)
            if let Some(ch) = state.peek() {
                if ch == 'f' || ch == 'F' || ch == 'l' || ch == 'L' {
                    state.advance(ch.len_utf8());
                }
            }

            state.add_token(ObjectiveCLanguageSyntaxKind::Number, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理标识符和关键
    fn lex_identifier(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if !ch.is_ascii_alphabetic() && ch != '_' {
                return false;
            }

            // 收集标识符字
            while let Some(ch) = state.peek() {
                if ch.is_ascii_alphanumeric() || ch == '_' {
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }

            // 检查是否是关键
            let text = source.get_text_in((start_pos..state.get_position()).into()).unwrap_or("");
            let token_kind = match text {
                "@interface" => ObjectiveCLanguageSyntaxKind::InterfaceKeyword,
                "@implementation" => ObjectiveCLanguageSyntaxKind::ImplementationKeyword,
                "@end" => ObjectiveCLanguageSyntaxKind::EndKeyword,
                "@property" => ObjectiveCLanguageSyntaxKind::PropertyKeyword,
                "@synthesize" => ObjectiveCLanguageSyntaxKind::SynthesizeKeyword,
                "@dynamic" => ObjectiveCLanguageSyntaxKind::DynamicKeyword,
                "@protocol" => ObjectiveCLanguageSyntaxKind::ProtocolKeyword,
                "@category" => ObjectiveCLanguageSyntaxKind::CategoryKeyword,
                "#import" => ObjectiveCLanguageSyntaxKind::ImportKeyword,
                "#include" => ObjectiveCLanguageSyntaxKind::IncludeKeyword,
                "if" => ObjectiveCLanguageSyntaxKind::IfKeyword,
                "else" => ObjectiveCLanguageSyntaxKind::ElseKeyword,
                "for" => ObjectiveCLanguageSyntaxKind::ForKeyword,
                "while" => ObjectiveCLanguageSyntaxKind::WhileKeyword,
                "do" => ObjectiveCLanguageSyntaxKind::DoKeyword,
                "switch" => ObjectiveCLanguageSyntaxKind::SwitchKeyword,
                "case" => ObjectiveCLanguageSyntaxKind::CaseKeyword,
                "default" => ObjectiveCLanguageSyntaxKind::DefaultKeyword,
                "break" => ObjectiveCLanguageSyntaxKind::BreakKeyword,
                "continue" => ObjectiveCLanguageSyntaxKind::ContinueKeyword,
                "return" => ObjectiveCLanguageSyntaxKind::ReturnKeyword,
                "void" => ObjectiveCLanguageSyntaxKind::VoidKeyword,
                "int" => ObjectiveCLanguageSyntaxKind::IntKeyword,
                "float" => ObjectiveCLanguageSyntaxKind::FloatKeyword,
                "double" => ObjectiveCLanguageSyntaxKind::DoubleKeyword,
                "char" => ObjectiveCLanguageSyntaxKind::CharKeyword,
                "BOOL" => ObjectiveCLanguageSyntaxKind::BoolKeyword,
                "id" => ObjectiveCLanguageSyntaxKind::IdKeyword,
                "self" => ObjectiveCLanguageSyntaxKind::SelfKeyword,
                "super" => ObjectiveCLanguageSyntaxKind::SuperKeyword,
                "nil" => ObjectiveCLanguageSyntaxKind::NilKeyword,
                "YES" => ObjectiveCLanguageSyntaxKind::YesKeyword,
                "NO" => ObjectiveCLanguageSyntaxKind::NoKeyword,
                _ => ObjectiveCLanguageSyntaxKind::Identifier,
            };

            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理操作
    fn lex_operator(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '+' => ObjectiveCLanguageSyntaxKind::Plus,
                '-' => ObjectiveCLanguageSyntaxKind::Minus,
                '*' => ObjectiveCLanguageSyntaxKind::Star,
                '/' => ObjectiveCLanguageSyntaxKind::Slash,
                '%' => ObjectiveCLanguageSyntaxKind::Percent,
                '=' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        state.add_token(ObjectiveCLanguageSyntaxKind::EqualEqual, start_pos, state.get_position());
                        return true;
                    }
                    ObjectiveCLanguageSyntaxKind::Equal
                }
                '!' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        state.add_token(ObjectiveCLanguageSyntaxKind::NotEqual, start_pos, state.get_position());
                        return true;
                    }
                    ObjectiveCLanguageSyntaxKind::Not
                }
                '<' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        state.add_token(ObjectiveCLanguageSyntaxKind::LessEqual, start_pos, state.get_position());
                        return true;
                    }
                    ObjectiveCLanguageSyntaxKind::Less
                }
                '>' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        state.add_token(ObjectiveCLanguageSyntaxKind::GreaterEqual, start_pos, state.get_position());
                        return true;
                    }
                    ObjectiveCLanguageSyntaxKind::Greater
                }
                '&' => {
                    if let Some('&') = state.peek_next_n(1) {
                        state.advance(2);
                        state.add_token(ObjectiveCLanguageSyntaxKind::And, start_pos, state.get_position());
                        return true;
                    }
                    return false;
                }
                '|' => {
                    if let Some('|') = state.peek_next_n(1) {
                        state.advance(2);
                        state.add_token(ObjectiveCLanguageSyntaxKind::Or, start_pos, state.get_position());
                        return true;
                    }
                    return false;
                }
                '?' => ObjectiveCLanguageSyntaxKind::Question,
                '@' => ObjectiveCLanguageSyntaxKind::At,
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

    /// 处理分隔
    fn lex_delimiter(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => ObjectiveCLanguageSyntaxKind::LeftParen,
                ')' => ObjectiveCLanguageSyntaxKind::RightParen,
                '{' => ObjectiveCLanguageSyntaxKind::LeftBrace,
                '}' => ObjectiveCLanguageSyntaxKind::RightBrace,
                '[' => ObjectiveCLanguageSyntaxKind::LeftBracket,
                ']' => ObjectiveCLanguageSyntaxKind::RightBracket,
                ';' => ObjectiveCLanguageSyntaxKind::Semicolon,
                ',' => ObjectiveCLanguageSyntaxKind::Comma,
                '.' => ObjectiveCLanguageSyntaxKind::Dot,
                ':' => ObjectiveCLanguageSyntaxKind::Colon,
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

impl<'config> Lexer<ObjectiveCLanguage> for ObjectiveCLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<ObjectiveCLanguageSyntaxKind> {
        let mut state = LexerState::new(source);

        while state.not_at_end() {
            // 尝试各种词法规则
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_comment(&mut state) {
                continue;
            }

            if self.lex_string(&mut state) {
                continue;
            }

            if self.lex_character(&mut state) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_identifier(&mut state, source) {
                continue;
            }

            if self.lex_operator(&mut state) {
                continue;
            }

            if self.lex_delimiter(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(ObjectiveCLanguageSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(ObjectiveCLanguageSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
