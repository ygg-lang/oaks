use crate::{kind::SoliditySyntaxKind, language::SolidityLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, SolidityLanguage>;

pub struct SolidityLexer<'config> {
    config: &'config SolidityLanguage,
}

impl<'config> SolidityLexer<'config> {
    pub fn new(config: &'config SolidityLanguage) -> Self {
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
            state.add_token(SoliditySyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(SoliditySyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(SoliditySyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理行注
    fn lex_line_comment(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            state.advance(1);
            if let Some('/') = state.peek() {
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(SoliditySyntaxKind::LineComment, start_pos, state.get_position());
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

    /// 处理块注
    fn lex_block_comment(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            state.advance(1);
            if let Some('*') = state.peek() {
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch == '*' {
                        state.advance(1);
                        if let Some('/') = state.peek() {
                            state.advance(1);
                            break;
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(SoliditySyntaxKind::BlockComment, start_pos, state.get_position());
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

                let text = source.get_text_in((start_pos..state.get_position()).into()).unwrap_or("");
                let token_kind = self.keyword_or_identifier(text);
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

    /// 判断是关键字还是标识
    fn keyword_or_identifier(&self, text: &str) -> SoliditySyntaxKind {
        match text {
            "contract" => SoliditySyntaxKind::Contract,
            "interface" => SoliditySyntaxKind::Interface,
            "library" => SoliditySyntaxKind::Library,
            "function" => SoliditySyntaxKind::Function,
            "modifier" => SoliditySyntaxKind::Modifier,
            "event" => SoliditySyntaxKind::Event,
            "struct" => SoliditySyntaxKind::Struct,
            "enum" => SoliditySyntaxKind::Enum,
            "mapping" => SoliditySyntaxKind::Mapping,
            "public" => SoliditySyntaxKind::Public,
            "private" => SoliditySyntaxKind::Private,
            "internal" => SoliditySyntaxKind::Internal,
            "external" => SoliditySyntaxKind::External,
            "pure" => SoliditySyntaxKind::Pure,
            "view" => SoliditySyntaxKind::View,
            "payable" => SoliditySyntaxKind::Payable,
            "constant" => SoliditySyntaxKind::Constant,
            "bool" => SoliditySyntaxKind::Bool,
            "string" => SoliditySyntaxKind::String,
            "bytes" => SoliditySyntaxKind::Bytes,
            "address" => SoliditySyntaxKind::Address,
            "uint" => SoliditySyntaxKind::Uint,
            "int" => SoliditySyntaxKind::Int,
            "fixed" => SoliditySyntaxKind::Fixed,
            "ufixed" => SoliditySyntaxKind::Ufixed,
            "if" => SoliditySyntaxKind::If,
            "else" => SoliditySyntaxKind::Else,
            "for" => SoliditySyntaxKind::For,
            "while" => SoliditySyntaxKind::While,
            "do" => SoliditySyntaxKind::Do,
            "break" => SoliditySyntaxKind::Break,
            "continue" => SoliditySyntaxKind::Continue,
            "return" => SoliditySyntaxKind::Return,
            "try" => SoliditySyntaxKind::Try,
            "catch" => SoliditySyntaxKind::Catch,
            "import" => SoliditySyntaxKind::Import,
            "pragma" => SoliditySyntaxKind::Pragma,
            "using" => SoliditySyntaxKind::Using,
            "is" => SoliditySyntaxKind::Is,
            "override" => SoliditySyntaxKind::Override,
            "virtual" => SoliditySyntaxKind::Virtual,
            "abstract" => SoliditySyntaxKind::Abstract,
            "true" | "false" => SoliditySyntaxKind::BooleanLiteral,
            _ => SoliditySyntaxKind::Identifier,
        }
    }

    /// 处理数字字面
    fn lex_number(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);

                // 处理十六进制
                if ch == '0' {
                    if let Some('x') | Some('X') = state.peek() {
                        state.advance(1);
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_hexdigit() {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                        state.add_token(SoliditySyntaxKind::HexLiteral, start_pos, state.get_position());
                        return true;
                    }
                }

                // 处理十进制数
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                // 处理小数
                if let Some('.') = state.peek() {
                    state.advance(1);
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }

                // 处理科学计数
                if let Some('e') | Some('E') = state.peek() {
                    state.advance(1);
                    if let Some('+') | Some('-') = state.peek() {
                        state.advance(1);
                    }
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }

                state.add_token(SoliditySyntaxKind::NumberLiteral, start_pos, state.get_position());
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

    /// 处理字符串字面量
    fn lex_string(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '"' || quote == '\'' {
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch == quote {
                        state.advance(1);
                        break;
                    }
                    else if ch == '\\' {
                        state.advance(1);
                        if let Some(_) = state.peek() {
                            state.advance(1);
                        }
                    }
                    else if ch == '\n' || ch == '\r' {
                        break; // 字符串不能跨
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(SoliditySyntaxKind::StringLiteral, start_pos, state.get_position());
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

    /// 处理操作
    fn lex_operator(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '+' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        SoliditySyntaxKind::PlusAssign
                    }
                    else {
                        SoliditySyntaxKind::Plus
                    }
                }
                '-' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        SoliditySyntaxKind::MinusAssign
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        SoliditySyntaxKind::Arrow
                    }
                    else {
                        SoliditySyntaxKind::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        SoliditySyntaxKind::StarAssign
                    }
                    else if let Some('*') = state.peek() {
                        state.advance(1);
                        SoliditySyntaxKind::Power
                    }
                    else {
                        SoliditySyntaxKind::Star
                    }
                }
                '/' => {
                    // 这里不处理注释，因为已经在其他地方处理了
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        SoliditySyntaxKind::SlashAssign
                    }
                    else {
                        SoliditySyntaxKind::Slash
                    }
                }
                '%' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        SoliditySyntaxKind::PercentAssign
                    }
                    else {
                        SoliditySyntaxKind::Percent
                    }
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        SoliditySyntaxKind::Equal
                    }
                    else {
                        SoliditySyntaxKind::Assign
                    }
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        SoliditySyntaxKind::NotEqual
                    }
                    else {
                        SoliditySyntaxKind::Not
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        SoliditySyntaxKind::LessEqual
                    }
                    else if let Some('<') = state.peek() {
                        state.advance(1);
                        SoliditySyntaxKind::LeftShift
                    }
                    else {
                        SoliditySyntaxKind::Less
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        SoliditySyntaxKind::GreaterEqual
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        SoliditySyntaxKind::RightShift
                    }
                    else {
                        SoliditySyntaxKind::Greater
                    }
                }
                '&' => {
                    state.advance(1);
                    if let Some('&') = state.peek() {
                        state.advance(1);
                        SoliditySyntaxKind::And
                    }
                    else {
                        SoliditySyntaxKind::BitAnd
                    }
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.peek() {
                        state.advance(1);
                        SoliditySyntaxKind::Or
                    }
                    else {
                        SoliditySyntaxKind::BitOr
                    }
                }
                '^' => {
                    state.advance(1);
                    SoliditySyntaxKind::BitXor
                }
                '~' => {
                    state.advance(1);
                    SoliditySyntaxKind::BitNot
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
    fn lex_delimiter(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => SoliditySyntaxKind::LeftParen,
                ')' => SoliditySyntaxKind::RightParen,
                '{' => SoliditySyntaxKind::LeftBrace,
                '}' => SoliditySyntaxKind::RightBrace,
                '[' => SoliditySyntaxKind::LeftBracket,
                ']' => SoliditySyntaxKind::RightBracket,
                ';' => SoliditySyntaxKind::Semicolon,
                ',' => SoliditySyntaxKind::Comma,
                '.' => SoliditySyntaxKind::Dot,
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

impl<'config> Lexer<SolidityLanguage> for SolidityLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<SoliditySyntaxKind> {
        let mut state = LexerState::new(source);

        while state.not_at_end() {
            // 尝试各种词法规则
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_line_comment(&mut state) {
                continue;
            }

            if self.lex_block_comment(&mut state) {
                continue;
            }

            if self.lex_string(&mut state) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_identifier_or_keyword(&mut state, source) {
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
                state.add_token(SoliditySyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(SoliditySyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
