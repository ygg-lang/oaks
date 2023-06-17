use crate::{kind::ZigSyntaxKind, language::ZigLanguage};
use oak_core::{
    lexer::{LexOutput, Lexer, LexerState, SourceText, State},
};

pub struct ZigLexer;

impl ZigLexer {
    pub fn new() -> Self {
        Self
    }

    /// 跳过空白字符
    fn skip_whitespace(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' {
                state.advance(ch.len_utf8());
            } else {
                break;
            }
        }

        if state.get_position() > start_pos {
            state.add_token(ZigSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        } else {
            false
        }
    }

    /// 处理换行

    fn lex_newline(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(ZigSyntaxKind::Newline, start_pos, state.get_position());
            true
        } else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(ZigSyntaxKind::Newline, start_pos, state.get_position());
            true
        } else {
            false
        }
    }

    /// 处理 Zig 注释 // ... 和文档注
/// ...
    fn lex_comment(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            if let Some('/') = state.peek_at(1) {
                state.advance(2);

                // 检查是否是文档注释
                let is_doc_comment = if let Some('/') = state.peek() {
                    state.advance(1);
                    true
                } else {
                    false
                };

                // 读取到行

                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }

                let token_kind = if is_doc_comment {
                    ZigSyntaxKind::DocComment
                } else {
                    ZigSyntaxKind::Comment
                };

                state.add_token(token_kind, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理字符串字面量
    fn lex_string(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.advance(1); // 跳过开始引

            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1); // 跳过结束引号
                    state.add_token(ZigSyntaxKind::StringLiteral, start_pos, state.get_position());
                    return true;
                } else if ch == '\\' {
                    // 处理转义字符
                    state.advance(1);
                    if let Some(_) = state.peek() {
                        state.advance(1);
                    }
                } else if ch == '\n' || ch == '\r' {
                    // Zig 字符串不能跨行（除非转义

                    break;
                } else {
                    state.advance(ch.len_utf8());
                }
            }

            // 未闭合的字符

            state.add_token(ZigSyntaxKind::Error, start_pos, state.get_position());
            return true;
        }

        false
    }

    /// 处理多行字符串字面量 \\...
    fn lex_multiline_string(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('\\') = state.peek() {
            if let Some('\\') = state.peek_at(1) {
                state.advance(2);

                // 读取到行

                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }

                state.add_token(ZigSyntaxKind::MultilineStringContent, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理字符字面

    fn lex_char(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('\'') = state.peek() {
            state.advance(1); // 跳过开始引

            // 处理字符内容
            if let Some(ch) = state.peek() {
                if ch == '\\' {
                    // 转义字符
                    state.advance(1);
                    if let Some(_) = state.peek() {
                        state.advance(1);
                    }
                } else if ch != '\'' && ch != '\n' && ch != '\r' {
                    state.advance(ch.len_utf8());
                }
            }

            // 检查结束引

            if let Some('\'') = state.peek() {
                state.advance(1);
                state.add_token(ZigSyntaxKind::CharLiteral, start_pos, state.get_position());
                return true;
            }

            // 未闭合的字符字面

            state.add_token(ZigSyntaxKind::Error, start_pos, state.get_position());
            return true;
        }

        false
    }

    /// 处理数字字面

    fn lex_number(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                // 检查十六进

                if ch == '0' {
                    if let Some(next_ch) = state.peek_at(1) {
                        if next_ch == 'x' || next_ch == 'X' {
                            state.advance(2); // 跳过 0x
                            while let Some(ch) = state.peek() {
                                if ch.is_ascii_hexdigit() || ch == '_' {
                                    state.advance(1);
                                } else {
                                    break;
                                }
                            }
                            state.add_token(ZigSyntaxKind::IntegerLiteral, start_pos, state.get_position());
                            return true;
                        } else if next_ch == 'b' || next_ch == 'B' {
                            // 二进

                            state.advance(2); // 跳过 0b
                            while let Some(ch) = state.peek() {
                                if ch == '0' || ch == '1' || ch == '_' {
                                    state.advance(1);
                                } else {
                                    break;
                                }
                            }
                            state.add_token(ZigSyntaxKind::IntegerLiteral, start_pos, state.get_position());
                            return true;
                        } else if next_ch == 'o' || next_ch == 'O' {
                            // 八进

                            state.advance(2); // 跳过 0o
                            while let Some(ch) = state.peek() {
                                if ch >= '0' && ch <= '7' || ch == '_' {
                                    state.advance(1);
                                } else {
                                    break;
                                }
                            }
                            state.add_token(ZigSyntaxKind::IntegerLiteral, start_pos, state.get_position());
                            return true;
                        }
                    }
                }

                // 十进制数

                let mut is_float = false;
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() || ch == '_' {
                        state.advance(1);
                    } else if ch == '.' && !is_float {
                        // 检查是否是浮点

                        if let Some(next_ch) = state.peek_at(1) {
                            if next_ch.is_ascii_digit() {
                                is_float = true;
                                state.advance(1); // 跳过小数

                            } else {
                                break; // 不是浮点数，可能是范围操作符
                            }
                        } else {
                            break;
                        }
                    } else if (ch == 'e' || ch == 'E') && (is_float || state.get_position() > start_pos + 1) {
                        // 科学计数

                        is_float = true;
                        state.advance(1);
                        if let Some(sign) = state.peek() {
                            if sign == '+' || sign == '-' {
                                state.advance(1);
                            }
                        }
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() || ch == '_' {
                                state.advance(1);
                            } else {
                                break;
                            }
                        }
                        break;
                    } else {
                        break;
                    }
                }

                let token_kind = if is_float {
                    ZigSyntaxKind::FloatLiteral
                } else {
                    ZigSyntaxKind::IntegerLiteral
                };

                state.add_token(token_kind, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理标识符和关键
    fn lex_identifier(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    } else {
                        break;
                    }
                }

                // 检查是否是关键
                let text = source.get_text_in(core::range::Range { start: start_pos, end: state.get_position() }).unwrap_or("");
                let token_kind = self.get_keyword_or_identifier(text);

                state.add_token(token_kind, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 获取关键字或标识符类

    fn get_keyword_or_identifier(&self, text: &str) -> ZigSyntaxKind {
        match text {
            // 基本结构
            "const" => ZigSyntaxKind::Const,
            "var" => ZigSyntaxKind::Var,
            "fn" => ZigSyntaxKind::Fn,
            "struct" => ZigSyntaxKind::Struct,
            "union" => ZigSyntaxKind::Union,
            "enum" => ZigSyntaxKind::Enum,
            "opaque" => ZigSyntaxKind::Opaque,
            "type" => ZigSyntaxKind::Type,
            "comptime" => ZigSyntaxKind::Comptime,
            "inline" => ZigSyntaxKind::Inline,
            "noinline" => ZigSyntaxKind::NoInline,
            "pub" => ZigSyntaxKind::Pub,
            "export" => ZigSyntaxKind::Export,
            "extern" => ZigSyntaxKind::Extern,
            "packed" => ZigSyntaxKind::Packed,
            "align" => ZigSyntaxKind::Align,
            "callconv" => ZigSyntaxKind::CallConv,
            "linksection" => ZigSyntaxKind::LinkSection,

            // 控制

            "if" => ZigSyntaxKind::If,
            "else" => ZigSyntaxKind::Else,
            "switch" => ZigSyntaxKind::Switch,
            "while" => ZigSyntaxKind::While,
            "for" => ZigSyntaxKind::For,
            "break" => ZigSyntaxKind::Break,
            "continue" => ZigSyntaxKind::Continue,
            "return" => ZigSyntaxKind::Return,
            "defer" => ZigSyntaxKind::Defer,
            "errdefer" => ZigSyntaxKind::ErrDefer,
            "unreachable" => ZigSyntaxKind::Unreachable,

            // 错误处理
            "try" => ZigSyntaxKind::Try,
            "catch" => ZigSyntaxKind::Catch,
            "orelse" => ZigSyntaxKind::Orelse,

            // 测试和异

            "test" => ZigSyntaxKind::Test,
            "async" => ZigSyntaxKind::Async,
            "await" => ZigSyntaxKind::Await,
            "suspend" => ZigSyntaxKind::Suspend,
            "resume" => ZigSyntaxKind::Resume,
            "cancel" => ZigSyntaxKind::Cancel,

            // 内存管理
            "undefined" => ZigSyntaxKind::Undefined,
            "null" => ZigSyntaxKind::Null,
            "volatile" => ZigSyntaxKind::Volatile,
            "allowzero" => ZigSyntaxKind::AllowZero,
            "noalias" => ZigSyntaxKind::NoAlias,

            // 逻辑操作
            "and" => ZigSyntaxKind::And,
            "or" => ZigSyntaxKind::Or,

            // 其他
            "anyframe" => ZigSyntaxKind::AnyFrame,
            "anytype" => ZigSyntaxKind::AnyType,
            "threadlocal" => ZigSyntaxKind::ThreadLocal,

            // 基本类型
            "bool" => ZigSyntaxKind::Bool,
            "i8" => ZigSyntaxKind::I8,
            "i16" => ZigSyntaxKind::I16,
            "i32" => ZigSyntaxKind::I32,
            "i64" => ZigSyntaxKind::I64,
            "i128" => ZigSyntaxKind::I128,
            "isize" => ZigSyntaxKind::Isize,
            "u8" => ZigSyntaxKind::U8,
            "u16" => ZigSyntaxKind::U16,
            "u32" => ZigSyntaxKind::U32,
            "u64" => ZigSyntaxKind::U64,
            "u128" => ZigSyntaxKind::U128,
            "usize" => ZigSyntaxKind::Usize,
            "f16" => ZigSyntaxKind::F16,
            "f32" => ZigSyntaxKind::F32,
            "f64" => ZigSyntaxKind::F64,
            "f80" => ZigSyntaxKind::F80,
            "f128" => ZigSyntaxKind::F128,
            "c_short" => ZigSyntaxKind::C_Short,
            "c_ushort" => ZigSyntaxKind::C_UShort,
            "c_int" => ZigSyntaxKind::C_Int,
            "c_uint" => ZigSyntaxKind::C_UInt,
            "c_long" => ZigSyntaxKind::C_Long,
            "c_ulong" => ZigSyntaxKind::C_ULong,
            "c_longlong" => ZigSyntaxKind::C_LongLong,
            "c_ulonglong" => ZigSyntaxKind::C_ULongLong,
            "c_longdouble" => ZigSyntaxKind::C_LongDouble,
            "c_void" => ZigSyntaxKind::C_Void,
            "void" => ZigSyntaxKind::Void,
            "noreturn" => ZigSyntaxKind::NoReturn,
            "comptime_int" => ZigSyntaxKind::Comptime_Int,
            "comptime_float" => ZigSyntaxKind::Comptime_Float,

            // 布尔字面

            "true" | "false" => ZigSyntaxKind::BooleanLiteral,

            _ => ZigSyntaxKind::Identifier,
        }
    }

    /// 处理内置函数前缀 @
    fn lex_builtin(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('@') = state.peek() {
            state.advance(1);
            state.add_token(ZigSyntaxKind::At, start_pos, state.get_position());
            true
        } else {
            false
        }
    }

    /// 处理标点符号和操作符
    fn lex_punctuation(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '+' => {
                    state.advance(1);
                    if let Some(next_ch) = state.peek() {
                        match next_ch {
                            '+' => {
                                state.advance(1);
                                ZigSyntaxKind::PlusPlus
                            }
                            '=' => {
                                state.advance(1);
                                ZigSyntaxKind::PlusAssign
                            }
                            '%' => {
                                state.advance(1);
                                ZigSyntaxKind::PlusPercent
                            }
                            _ => ZigSyntaxKind::Plus,
                        }
                    } else {
                        ZigSyntaxKind::Plus
                    }
                }
                '-' => {
                    state.advance(1);
                    if let Some(next_ch) = state.peek() {
                        match next_ch {
                            '=' => {
                                state.advance(1);
                                ZigSyntaxKind::MinusAssign
                            }
                            '%' => {
                                state.advance(1);
                                ZigSyntaxKind::MinusPercent
                            }
                            _ => ZigSyntaxKind::Minus,
                        }
                    } else {
                        ZigSyntaxKind::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    if let Some(next_ch) = state.peek() {
                        match next_ch {
                            '*' => {
                                state.advance(1);
                                ZigSyntaxKind::StarStar
                            }
                            '=' => {
                                state.advance(1);
                                ZigSyntaxKind::StarAssign
                            }
                            '%' => {
                                state.advance(1);
                                ZigSyntaxKind::StarPercent
                            }
                            _ => ZigSyntaxKind::Star,
                        }
                    } else {
                        ZigSyntaxKind::Star
                    }
                }
                '/' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        ZigSyntaxKind::SlashAssign
                    } else {
                        ZigSyntaxKind::Slash
                    }
                }
                '%' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        ZigSyntaxKind::PercentAssign
                    } else {
                        ZigSyntaxKind::Percent
                    }
                }
                '&' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        ZigSyntaxKind::AmpersandAssign
                    } else {
                        ZigSyntaxKind::Ampersand
                    }
                }
                '|' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        ZigSyntaxKind::PipeAssign
                    } else {
                        ZigSyntaxKind::Pipe
                    }
                }
                '^' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        ZigSyntaxKind::CaretAssign
                    } else {
                        ZigSyntaxKind::Caret
                    }
                }
                '~' => {
                    state.advance(1);
                    ZigSyntaxKind::Tilde
                }
                '<' => {
                    state.advance(1);
                    if let Some(next_ch) = state.peek() {
                        match next_ch {
                            '<' => {
                                state.advance(1);
                                if let Some('=') = state.peek() {
                                    state.advance(1);
                                    ZigSyntaxKind::LessLessAssign
                                } else {
                                    ZigSyntaxKind::LessLess
                                }
                            }
                            '=' => {
                                state.advance(1);
                                ZigSyntaxKind::LessEqual
                            }
                            _ => ZigSyntaxKind::Less,
                        }
                    } else {
                        ZigSyntaxKind::Less
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some(next_ch) = state.peek() {
                        match next_ch {
                            '>' => {
                                state.advance(1);
                                if let Some('=') = state.peek() {
                                    state.advance(1);
                                    ZigSyntaxKind::GreaterGreaterAssign
                                } else {
                                    ZigSyntaxKind::GreaterGreater
                                }
                            }
                            '=' => {
                                state.advance(1);
                                ZigSyntaxKind::GreaterEqual
                            }
                            _ => ZigSyntaxKind::Greater,
                        }
                    } else {
                        ZigSyntaxKind::Greater
                    }
                }
                '=' => {
                    state.advance(1);
                    if let Some(next_ch) = state.peek() {
                        match next_ch {
                            '=' => {
                                state.advance(1);
                                ZigSyntaxKind::Equal
                            }
                            '>' => {
                                state.advance(1);
                                ZigSyntaxKind::Arrow
                            }
                            _ => ZigSyntaxKind::Assign,
                        }
                    } else {
                        ZigSyntaxKind::Assign
                    }
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        ZigSyntaxKind::NotEqual
                    } else {
                        ZigSyntaxKind::Exclamation
                    }
                }
                '(' => {
                    state.advance(1);
                    ZigSyntaxKind::LeftParen
                }
                ')' => {
                    state.advance(1);
                    ZigSyntaxKind::RightParen
                }
                '{' => {
                    state.advance(1);
                    ZigSyntaxKind::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    ZigSyntaxKind::RightBrace
                }
                '[' => {
                    state.advance(1);
                    ZigSyntaxKind::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    ZigSyntaxKind::RightBracket
                }
                ';' => {
                    state.advance(1);
                    ZigSyntaxKind::Semicolon
                }
                ',' => {
                    state.advance(1);
                    ZigSyntaxKind::Comma
                }
                '.' => {
                    state.advance(1);
                    if let Some(next_ch) = state.peek() {
                        if next_ch == '.' {
                            state.advance(1);
                            if let Some('.') = state.peek() {
                                state.advance(1);
                                ZigSyntaxKind::DotDotDot
                            } else {
                                ZigSyntaxKind::DotDot
                            }
                        } else {
                            ZigSyntaxKind::Dot
                        }
                    } else {
                        ZigSyntaxKind::Dot
                    }
                }
                ':' => {
                    state.advance(1);
                    ZigSyntaxKind::Colon
                }
                '?' => {
                    state.advance(1);
                    ZigSyntaxKind::Question
                }
                _ => return false,
            };

            state.add_token(token_kind, start_pos, state.get_position());
            true
        } else {
            false
        }
    }

    /// 处理普通文本内

    fn lex_text(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            // 遇到特殊字符时停

            match ch {
                ' ' | '\t' | '\n' | '\r' | '/' | '"' | '\'' | '\\' | '@' | '+' | '-' | '*' | '%' | '&' | '|' | '^' | '~' | '<' | '>' | '=' | '!' | '(' | ')' | '{' | '}' | '[' | ']' | ';' | ',' | '.' | ':' | '?' => break,
                _ if ch.is_ascii_alphanumeric() || ch == '_' => break, // 让标识符处理器处

                _ if ch.is_ascii_digit() => break, // 让数字处理器处理
                _ => {
                    state.advance(ch.len_utf8());
                }
            }
        }

        if state.get_position() > start_pos {
            state.add_token(ZigSyntaxKind::Text, start_pos, state.get_position());
            true
        } else {
            false
        }
    }
}

impl Lexer<ZigLanguage> for ZigLexer {
    fn lex(&self, source: &SourceText) -> LexOutput<ZigSyntaxKind> {
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

            if self.lex_multiline_string(&mut state) {
                continue;
            }

            if self.lex_string(&mut state) {
                continue;
            }

            if self.lex_char(&mut state) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_builtin(&mut state) {
                continue;
            }

            if self.lex_identifier(&mut state, source) {
                continue;
            }

            if self.lex_punctuation(&mut state) {
                continue;
            }

            if self.lex_text(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(ZigSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(ZigSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
