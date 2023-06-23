#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::SolidityLanguage, lexer::token_type::SolidityTokenType};
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
                state.add_token(SolidityTokenType::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point)
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
            state.add_token(SolidityTokenType::Whitespace, start_pos, state.get_position());
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
            state.add_token(SolidityTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(SolidityTokenType::Newline, start_pos, state.get_position());
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

                state.add_token(SolidityTokenType::LineComment, start_pos, state.get_position());
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

                state.add_token(SolidityTokenType::BlockComment, start_pos, state.get_position());
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
    fn keyword_or_identifier(&self, text: &str) -> SolidityTokenType {
        match text {
            "contract" => SolidityTokenType::Contract,
            "interface" => SolidityTokenType::Interface,
            "library" => SolidityTokenType::Library,
            "function" => SolidityTokenType::Function,
            "modifier" => SolidityTokenType::Modifier,
            "event" => SolidityTokenType::Event,
            "struct" => SolidityTokenType::Struct,
            "enum" => SolidityTokenType::Enum,
            "mapping" => SolidityTokenType::Mapping,
            "public" => SolidityTokenType::Public,
            "private" => SolidityTokenType::Private,
            "internal" => SolidityTokenType::Internal,
            "external" => SolidityTokenType::External,
            "pure" => SolidityTokenType::Pure,
            "view" => SolidityTokenType::View,
            "payable" => SolidityTokenType::Payable,
            "constant" => SolidityTokenType::Constant,
            "bool" => SolidityTokenType::Bool,
            "string" => SolidityTokenType::String,
            "bytes" => SolidityTokenType::Bytes,
            "address" => SolidityTokenType::Address,
            "uint" => SolidityTokenType::Uint,
            "int" => SolidityTokenType::Int,
            "fixed" => SolidityTokenType::Fixed,
            "ufixed" => SolidityTokenType::Ufixed,
            "if" => SolidityTokenType::If,
            "else" => SolidityTokenType::Else,
            "for" => SolidityTokenType::For,
            "while" => SolidityTokenType::While,
            "do" => SolidityTokenType::Do,
            "break" => SolidityTokenType::Break,
            "continue" => SolidityTokenType::Continue,
            "return" => SolidityTokenType::Return,
            "try" => SolidityTokenType::Try,
            "catch" => SolidityTokenType::Catch,
            "import" => SolidityTokenType::Import,
            "pragma" => SolidityTokenType::Pragma,
            "using" => SolidityTokenType::Using,
            "is" => SolidityTokenType::Is,
            "override" => SolidityTokenType::Override,
            "virtual" => SolidityTokenType::Virtual,
            "abstract" => SolidityTokenType::Abstract,
            "true" | "false" => SolidityTokenType::BooleanLiteral,
            _ => SolidityTokenType::Identifier,
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
                        state.add_token(SolidityTokenType::HexLiteral, start_pos, state.get_position());
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

                state.add_token(SolidityTokenType::NumberLiteral, start_pos, state.get_position());
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
                    state.add_token(SolidityTokenType::StringLiteral, start_pos, state.get_position());
                }
                else {
                    state.add_token(SolidityTokenType::Error, start_pos, state.get_position())
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
                        SolidityTokenType::PlusAssign
                    }
                    else {
                        SolidityTokenType::Plus
                    }
                }
                '-' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        SolidityTokenType::MinusAssign
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        SolidityTokenType::Arrow
                    }
                    else {
                        SolidityTokenType::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        SolidityTokenType::StarAssign
                    }
                    else if let Some('*') = state.peek() {
                        state.advance(1);
                        SolidityTokenType::Power
                    }
                    else {
                        SolidityTokenType::Star
                    }
                }
                '/' => {
                    // 这里不处理注释，因为已经在其他地方处理了
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        SolidityTokenType::SlashAssign
                    }
                    else {
                        SolidityTokenType::Slash
                    }
                }
                '%' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        SolidityTokenType::PercentAssign
                    }
                    else {
                        SolidityTokenType::Percent
                    }
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        SolidityTokenType::Equal
                    }
                    else {
                        SolidityTokenType::Assign
                    }
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        SolidityTokenType::NotEqual
                    }
                    else {
                        SolidityTokenType::Not
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        SolidityTokenType::LessEqual
                    }
                    else if let Some('<') = state.peek() {
                        state.advance(1);
                        SolidityTokenType::LeftShift
                    }
                    else {
                        SolidityTokenType::Less
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        SolidityTokenType::GreaterEqual
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        SolidityTokenType::RightShift
                    }
                    else {
                        SolidityTokenType::Greater
                    }
                }
                '&' => {
                    state.advance(1);
                    if let Some('&') = state.peek() {
                        state.advance(1);
                        SolidityTokenType::And
                    }
                    else {
                        SolidityTokenType::BitAnd
                    }
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.peek() {
                        state.advance(1);
                        SolidityTokenType::Or
                    }
                    else {
                        SolidityTokenType::BitOr
                    }
                }
                '^' => {
                    state.advance(1);
                    SolidityTokenType::BitXor
                }
                '~' => {
                    state.advance(1);
                    SolidityTokenType::BitNot
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
                '(' => SolidityTokenType::LeftParen,
                ')' => SolidityTokenType::RightParen,
                '{' => SolidityTokenType::LeftBrace,
                '}' => SolidityTokenType::RightBrace,
                '[' => SolidityTokenType::LeftBracket,
                ']' => SolidityTokenType::RightBracket,
                ';' => SolidityTokenType::Semicolon,
                ',' => SolidityTokenType::Comma,
                '.' => SolidityTokenType::Dot,
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
