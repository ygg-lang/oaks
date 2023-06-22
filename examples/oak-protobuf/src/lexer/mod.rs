use crate::{kind::ProtobufSyntaxKind, language::ProtobufLanguage};
use oak_core::{
    Lexer, LexerCache, LexerState, OakError,
    lexer::LexOutput,
    source::{Source, TextEdit},
};

type State<'a, S> = LexerState<'a, S, ProtobufLanguage>;

#[derive(Clone)]
pub struct ProtobufLexer<'config> {
    _config: &'config ProtobufLanguage,
}

impl<'config> ProtobufLexer<'config> {
    pub fn new(config: &'config ProtobufLanguage) -> Self {
        Self { _config: config }
    }

    fn run<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> Result<(), OakError> {
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

            if self.lex_string_literal(state) {
                continue;
            }

            if self.lex_number_literal(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_operators_and_delimiters(state) {
                continue;
            }

            // 如果没有匹配任何规则，跳过当前字符
            if let Some(ch) = state.peek() {
                let start_pos = state.get_position();
                state.advance(ch.len_utf8());
                state.add_token(ProtobufSyntaxKind::Error, start_pos, state.get_position());
            }
            else {
                // 如果已到达文件末尾，退出循环
                break;
            }

            state.advance_if_dead_lock(safe_point);
        }

        // Add EOF token
        let pos = state.get_position();
        state.add_token(ProtobufSyntaxKind::Eof, pos, pos);

        Ok(())
    }

    fn skip_whitespace<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
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

    fn lex_newline<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
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

    fn lex_comment<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            state.advance(1);
            if let Some('/') = state.peek() {
                state.advance(1);
                // 单行注释
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }
                state.add_token(ProtobufSyntaxKind::Comment, start_pos, state.get_position());
                true
            }
            else if let Some('*') = state.peek() {
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
                        state.advance(ch.len_utf8());
                    }
                }
                state.add_token(ProtobufSyntaxKind::Comment, start_pos, state.get_position());
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

    fn lex_string_literal<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(quote_char) = state.peek() {
            if quote_char == '"' || quote_char == '\'' {
                state.advance(1); // 跳过开始引号

                let mut escaped = false;
                while let Some(ch) = state.peek() {
                    if escaped {
                        escaped = false;
                        state.advance(ch.len_utf8());
                    }
                    else if ch == '\\' {
                        escaped = true;
                        state.advance(1);
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
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(ProtobufSyntaxKind::StringLiteral, start_pos, state.get_position());
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

    fn lex_number_literal<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() || (ch == '-' && state.peek_next_n(1).map_or(false, |c| c.is_ascii_digit())) {
                let start_pos = state.get_position();

                // 处理负号
                if ch == '-' {
                    state.advance(1);
                }

                // 读取整数部分
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                // 检查小数点
                if let Some('.') = state.peek() {
                    if state.peek_next_n(1).map_or(false, |c| c.is_ascii_digit()) {
                        state.advance(1);
                        // 读取小数部分
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                    }
                }

                // 检查科学记数法
                if let Some(ch) = state.peek() {
                    if ch == 'e' || ch == 'E' {
                        state.advance(1);
                        if let Some(ch) = state.peek() {
                            if ch == '+' || ch == '-' {
                                state.advance(1);
                            }
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

    fn lex_identifier_or_keyword<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                let start_pos = state.get_position();
                let mut text = String::new();

                // 读取标识符
                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        text.push(ch);
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 检查是否是关键字
                let kind = match text.as_str() {
                    "kind" => ProtobufSyntaxKind::Syntax,
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
                    // 布尔字面量
                    "true" | "false" => ProtobufSyntaxKind::BooleanLiteral,
                    _ => ProtobufSyntaxKind::Identifier,
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

    fn lex_operators_and_delimiters<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        if let Some(ch) = state.peek() {
            let start_pos = state.get_position();

            let kind = match ch {
                '=' => {
                    state.advance(1);
                    ProtobufSyntaxKind::Assign
                }
                ';' => {
                    state.advance(1);
                    ProtobufSyntaxKind::Semicolon
                }
                ',' => {
                    state.advance(1);
                    ProtobufSyntaxKind::Comma
                }
                '.' => {
                    state.advance(1);
                    ProtobufSyntaxKind::Dot
                }
                '(' => {
                    state.advance(1);
                    ProtobufSyntaxKind::LeftParen
                }
                ')' => {
                    state.advance(1);
                    ProtobufSyntaxKind::RightParen
                }
                '[' => {
                    state.advance(1);
                    ProtobufSyntaxKind::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    ProtobufSyntaxKind::RightBracket
                }
                '{' => {
                    state.advance(1);
                    ProtobufSyntaxKind::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    ProtobufSyntaxKind::RightBrace
                }
                '<' => {
                    state.advance(1);
                    ProtobufSyntaxKind::LeftAngle
                }
                '>' => {
                    state.advance(1);
                    ProtobufSyntaxKind::RightAngle
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

impl<'config> Lexer<ProtobufLanguage> for ProtobufLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<ProtobufLanguage>) -> LexOutput<ProtobufLanguage> {
        let mut state = State::new(source);
        let result = self.run(&mut state);
        state.finish_with_cache(result, cache)
    }
}
