use crate::{kind::SoliditySyntaxKind, language::SolidityLanguage};
use oak_core::{Lexer, LexerCache, LexerState, OakError, TextEdit, lexer::LexOutput, source::Source};

type State<'a, S> = LexerState<'a, S, SolidityLanguage>;

#[derive(Clone)]
pub struct SolidityLexer<'config> {
    _config: &'config SolidityLanguage,
}

impl<'config> Lexer<SolidityLanguage> for SolidityLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<SolidityLanguage>) -> LexOutput<SolidityLanguage> {
        let mut state = State::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> SolidityLexer<'config> {
    pub fn new(config: &'config SolidityLanguage) -> Self {
        Self { _config: config }
    }

    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_newline(state) {
                continue;
            }

            if self.lex_line_comment(state) {
                continue;
            }

            if self.lex_block_comment(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_number(state) {
                continue;
            }

            if self.lex_string(state) {
                continue;
            }

            if self.lex_operator(state) {
                continue;
            }

            if self.lex_delimiter(state) {
                continue;
            }

            // 如果没有匹配任何规则，跳过当前字符并标记错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(SoliditySyntaxKind::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
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
            state.add_token(SoliditySyntaxKind::Whitespace, start_pos, state.get_position());
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

    /// 处理单行注释
    fn lex_line_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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

    /// 处理块注释
    fn lex_block_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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

    /// 处理标识符或关键字
    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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

                let text = state.get_text_from(start_pos);
                let token_kind = self.keyword_or_identifier(&text);
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

    /// 处理数字
    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(ch.len_utf8());

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

    /// 处理字符串
    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '"' || quote == '\'' {
                state.advance(1);
                let mut found_end = false;

                while let Some(ch) = state.peek() {
                    if ch == quote {
                        state.advance(1);
                        found_end = true;
                        break;
                    }
                    else if ch == '\\' {
                        state.advance(1);
                        if let Some(_) = state.peek() {
                            state.advance(1);
                        }
                    }
                    else if ch == '\n' || ch == '\r' {
                        break; // 字符串不能跨行
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                if found_end {
                    state.add_token(SoliditySyntaxKind::StringLiteral, start_pos, state.get_position());
                }
                else {
                    state.add_token(SoliditySyntaxKind::Error, start_pos, state.get_position());
                }
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

    /// 处理操作符
    fn lex_operator<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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

    /// 处理分隔符
    fn lex_delimiter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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
