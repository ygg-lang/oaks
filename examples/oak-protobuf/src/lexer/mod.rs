use crate::{kind::ProtobufSyntaxKind, language::ProtobufLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, ProtobufLanguage>;

pub struct ProtobufLexer<'config> {
    config: &'config ProtobufLanguage,
}

impl<'config> ProtobufLexer<'config> {
    pub fn new(config: &'config ProtobufLanguage) -> Self {
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
            state.add_token(ProtobufSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(ProtobufSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(ProtobufSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        // // 单行注释
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
                state.add_token(ProtobufSyntaxKind::Comment, start_pos, state.get_position());
                return true;
            }
        }

        // /* */ 多行注释
        if let Some('/') = state.peek() {
            if let Some('*') = source.get_char_at(start_pos + 1) {
                state.advance(2);
                while let Some(ch) = state.peek() {
                    if ch == '*' {
                        if let Some('/') = source.get_char_at(state.get_position() + 1) {
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
                state.add_token(ProtobufSyntaxKind::Comment, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理字符串字面量
    fn lex_string_literal(&self, state: &mut State) -> bool {
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
                        break; // 字符串不能跨行（除非转义
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                if found_end {
                    state.add_token(ProtobufSyntaxKind::StringLiteral, start_pos, state.get_position());
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
        else {
            false
        }
    }

    /// 处理数字字面
    fn lex_number_literal(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() || ch == '-' {
                // 处理负号
                if ch == '-' {
                    state.advance(1);
                    if let Some(next_ch) = state.peek() {
                        if !next_ch.is_ascii_digit() {
                            // 不是数字，回退
                            state.set_position(start_pos);
                            return false;
                        }
                    }
                    else {
                        // 没有后续字符，回退
                        state.set_position(start_pos);
                        return false;
                    }
                }

                // 整数部分
                while let Some(digit) = state.peek() {
                    if digit.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                // 检查小数点
                if let Some('.') = state.peek() {
                    let next_pos = state.get_position() + 1;
                    if let Some(next_ch) = state.get_char_at(next_pos) {
                        if next_ch.is_ascii_digit() {
                            state.advance(1);

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

                        if state.get_position() == exp_start {
                            // 没有有效的指数，回退
                            state.set_position(saved_pos);
                        }
                    }
                }

                state.add_token(ProtobufSyntaxKind::NumberLiteral, start_pos, state.get_position());
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

    /// 处理标识符和关键
    fn lex_identifier_or_keyword(&self, state: &mut State) -> bool {
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

                let text = state.source.get_text(start_pos, state.get_position());
                let token_kind = match text {
                    // 关键                    "kind" => ProtobufSyntaxKind::Syntax,
                    "package" => ProtobufSyntaxKind::Package,
                    "import" => ProtobufSyntaxKind::Import,
                    "option" => ProtobufSyntaxKind::Option,
                    "message" => ProtobufSyntaxKind::Message,
                    "enum" => ProtobufSyntaxKind::Enum,
                    "service" => ProtobufSyntaxKind::Service,
                    "rpc" => ProtobufSyntaxKind::Rpc,
                    "returns" => ProtobufSyntaxKind::Returns,
                    "stream" => ProtobufSyntaxKind::Stream,
                    "repeated" => ProtobufSyntaxKind::Repeated,
                    "optional" => ProtobufSyntaxKind::Optional,
                    "required" => ProtobufSyntaxKind::Required,
                    "oneof" => ProtobufSyntaxKind::Oneof,
                    "map" => ProtobufSyntaxKind::Map,
                    "reserved" => ProtobufSyntaxKind::Reserved,
                    "extensions" => ProtobufSyntaxKind::Extensions,
                    "extend" => ProtobufSyntaxKind::Extend,
                    "group" => ProtobufSyntaxKind::Group,
                    "public" => ProtobufSyntaxKind::Public,
                    "weak" => ProtobufSyntaxKind::Weak,

                    // 数据类型
                    "double" => ProtobufSyntaxKind::Double,
                    "float" => ProtobufSyntaxKind::Float,
                    "int32" => ProtobufSyntaxKind::Int32,
                    "int64" => ProtobufSyntaxKind::Int64,
                    "uint32" => ProtobufSyntaxKind::Uint32,
                    "uint64" => ProtobufSyntaxKind::Uint64,
                    "sint32" => ProtobufSyntaxKind::Sint32,
                    "sint64" => ProtobufSyntaxKind::Sint64,
                    "fixed32" => ProtobufSyntaxKind::Fixed32,
                    "fixed64" => ProtobufSyntaxKind::Fixed64,
                    "sfixed32" => ProtobufSyntaxKind::Sfixed32,
                    "sfixed64" => ProtobufSyntaxKind::Sfixed64,
                    "bool" => ProtobufSyntaxKind::Bool,
                    "string" => ProtobufSyntaxKind::String,
                    "bytes" => ProtobufSyntaxKind::Bytes,

                    // 布尔字面                    "true" | "false" => ProtobufSyntaxKind::BooleanLiteral,
                    _ => ProtobufSyntaxKind::Identifier,
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

    /// 处理运算符和分隔
    fn lex_operators_and_delimiters(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '=' => ProtobufSyntaxKind::Assign,
                ';' => ProtobufSyntaxKind::Semicolon,
                ',' => ProtobufSyntaxKind::Comma,
                '.' => ProtobufSyntaxKind::Dot,
                '(' => ProtobufSyntaxKind::LeftParen,
                ')' => ProtobufSyntaxKind::RightParen,
                '[' => ProtobufSyntaxKind::LeftBracket,
                ']' => ProtobufSyntaxKind::RightBracket,
                '{' => ProtobufSyntaxKind::LeftBrace,
                '}' => ProtobufSyntaxKind::RightBrace,
                '<' => ProtobufSyntaxKind::LeftAngle,
                '>' => ProtobufSyntaxKind::RightAngle,
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

impl<'config> Lexer<ProtobufLanguage> for ProtobufLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<ProtobufSyntaxKind> {
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

            if self.lex_number_literal(&mut state) {
                continue;
            }

            if self.lex_identifier_or_keyword(&mut state) {
                continue;
            }

            if self.lex_operators_and_delimiters(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(ProtobufSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(ProtobufSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
