use crate::{kind::ZigSyntaxKind, language::ZigLanguage};
use oak_core::{
    Lexer, LexerCache, LexerState, OakError,
    lexer::{LexOutput, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<'a, S> = LexerState<'a, S, ZigLanguage>;

static ZIG_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });

#[derive(Clone)]
pub struct ZigLexer<'config> {
    _config: &'config ZigLanguage,
}

impl<'config> Lexer<ZigLanguage> for ZigLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl LexerCache<ZigLanguage>) -> LexOutput<ZigLanguage> {
        let mut state = State::new_with_cache(source, 0, cache);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> ZigLexer<'config> {
    pub fn new(config: &'config ZigLanguage) -> Self {
        Self { _config: config }
    }

    /// 主要的词法分析循环
    fn run<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.skip_whitespace(state) {
                continue;
            }

            if self.skip_comment(state) {
                continue;
            }

            if self.lex_string_literal(state) {
                continue;
            }

            if self.lex_char_literal(state) {
                continue;
            }

            if self.lex_number_literal(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_builtin(state) {
                continue;
            }

            if self.lex_operators(state) {
                continue;
            }

            if self.lex_single_char_tokens(state) {
                continue;
            }

            // 如果没有匹配到任何规则，前进一个字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(ZigSyntaxKind::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        ZIG_WHITESPACE.scan(state, ZigSyntaxKind::Whitespace)
    }

    /// 跳过注释
    fn skip_comment<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // 行注释: // ... 直到换行
        if rest.starts_with("//") {
            state.advance(2);

            // 检查是否是文档注释 ///
            let is_doc_comment = if state.peek() == Some('/') {
                state.advance(1);
                true
            }
            else {
                false
            };

            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            let kind = if is_doc_comment { ZigSyntaxKind::DocComment } else { ZigSyntaxKind::Comment };
            state.add_token(kind, start, state.get_position());
            return true;
        }

        false
    }

    /// 解析字符串字面量
    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        // 多行字符串: \\...
        if state.rest().starts_with("\\\\") {
            state.advance(2);

            // 跳过到行尾
            while let Some(ch) = state.peek() {
                if ch == '\n' {
                    state.advance(1);
                    break;
                }
                state.advance(ch.len_utf8());
            }

            // 读取多行字符串内容
            while state.not_at_end() {
                let _line_start = state.get_position();

                // 检查是否是续行
                if !state.rest().starts_with("\\\\") {
                    break;
                }

                state.advance(2);

                // 读取到行尾
                while let Some(ch) = state.peek() {
                    if ch == '\n' {
                        state.advance(1);
                        break;
                    }
                    state.advance(ch.len_utf8());
                }
            }

            state.add_token(ZigSyntaxKind::StringLiteral, start, state.get_position());
            return true;
        }

        // 普通字符串: "..."
        if state.current() == Some('"') {
            state.advance(1);
            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1);
                    break;
                }
                if ch == '\\' {
                    state.advance(1);
                    if let Some(next) = state.peek() {
                        state.advance(next.len_utf8());
                    }
                    continue;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(ZigSyntaxKind::StringLiteral, start, state.get_position());
            return true;
        }

        false
    }

    /// 解析字符字面量
    fn lex_char_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if state.current() == Some('\'') {
            state.advance(1);
            while let Some(ch) = state.peek() {
                if ch == '\'' {
                    state.advance(1);
                    break;
                }
                if ch == '\\' {
                    state.advance(1);
                    if let Some(next) = state.peek() {
                        state.advance(next.len_utf8());
                    }
                    continue;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(ZigSyntaxKind::CharLiteral, start, state.get_position());
            return true;
        }
        false
    }

    /// 解析数字字面量
    fn lex_number_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let ch = state.current();
        let mut is_float = false;

        if let Some(ch) = ch {
            if ch.is_ascii_digit() {
                state.advance(1);
                // 处理十六进制、二进制、八进制
                if ch == '0' {
                    if let Some(next) = state.peek() {
                        match next {
                            'x' | 'X' => {
                                state.advance(1);
                                state.take_while(|c| c.is_ascii_hexdigit() || c == '_');
                            }
                            'b' | 'B' => {
                                state.advance(1);
                                state.take_while(|c| c == '0' || c == '1' || c == '_');
                            }
                            'o' | 'O' => {
                                state.advance(1);
                                state.take_while(|c| ('0'..='7').contains(&c) || c == '_');
                            }
                            _ => {
                                state.take_while(|c| c.is_ascii_digit() || c == '_');
                            }
                        }
                    }
                }
                else {
                    state.take_while(|c| c.is_ascii_digit() || c == '_');
                }

                // 处理小数点
                if state.current() == Some('.') {
                    if let Some(next) = state.peek() {
                        if next.is_ascii_digit() {
                            is_float = true;
                            state.advance(1);
                            state.take_while(|c| c.is_ascii_digit() || c == '_');
                        }
                    }
                }

                // 处理指数
                if let Some(c) = state.current() {
                    if c == 'e' || c == 'E' || c == 'p' || c == 'P' {
                        is_float = true;
                        state.advance(1);
                        if let Some(next) = state.peek() {
                            if next == '+' || next == '-' {
                                state.advance(1);
                            }
                        }
                        state.take_while(|c| c.is_ascii_digit() || c == '_');
                    }
                }

                let kind = if is_float { ZigSyntaxKind::FloatLiteral } else { ZigSyntaxKind::IntegerLiteral };
                state.add_token(kind, start, state.get_position());
                return true;
            }
        }
        false
    }

    /// 解析标识符或关键字
    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.current() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());
                state.take_while(|c| c.is_ascii_alphanumeric() || c == '_');

                let end = state.get_position();
                let text = state.get_text_in((start..end).into());
                let kind = self.get_keyword_or_identifier(&text);
                state.add_token(kind, start, state.get_position());
                return true;
            }
        }
        false
    }

    /// 获取关键字或标识符类型
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

            // 控制流
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
            "noreturn" => ZigSyntaxKind::NoReturn,

            // 错误处理
            "try" => ZigSyntaxKind::TryKeyword,
            "catch" => ZigSyntaxKind::CatchKeyword,
            "orelse" => ZigSyntaxKind::OrElse,
            "error" => ZigSyntaxKind::ErrorKeyword,

            // 测试和异步
            "test" => ZigSyntaxKind::Test,
            "async" => ZigSyntaxKind::Async,
            "await" => ZigSyntaxKind::AwaitKeyword,
            "suspend" => ZigSyntaxKind::Suspend,
            "resume" => ZigSyntaxKind::Resume,
            "cancel" => ZigSyntaxKind::Cancel,

            // 内存管理
            "undefined" => ZigSyntaxKind::Undefined,
            "null" => ZigSyntaxKind::Null,
            "volatile" => ZigSyntaxKind::Volatile,
            "allowzero" => ZigSyntaxKind::AllowZero,
            "noalias" => ZigSyntaxKind::NoAlias,

            // 逻辑运算
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
            "c_short" => ZigSyntaxKind::CShort,
            "c_ushort" => ZigSyntaxKind::CUshort,
            "c_int" => ZigSyntaxKind::CInt,
            "c_uint" => ZigSyntaxKind::CUint,
            "c_long" => ZigSyntaxKind::CLong,
            "c_ulong" => ZigSyntaxKind::CUlong,
            "c_longlong" => ZigSyntaxKind::CLongLong,
            "c_ulonglong" => ZigSyntaxKind::CUlongLong,
            "c_longdouble" => ZigSyntaxKind::CLongDouble,
            "c_void" => ZigSyntaxKind::CVoid,
            "void" => ZigSyntaxKind::Void,
            "comptime_int" => ZigSyntaxKind::ComptimeInt,
            "comptime_float" => ZigSyntaxKind::ComptimeFloat,

            // 布尔字面量
            "true" | "false" => ZigSyntaxKind::BooleanLiteral,

            _ => ZigSyntaxKind::Identifier,
        }
    }

    /// 解析内置标识符 (@import 等)
    fn lex_builtin<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if state.current() == Some('@') {
            state.advance(1);
            if let Some(ch) = state.current() {
                if ch.is_ascii_alphabetic() || ch == '_' {
                    state.advance(ch.len_utf8());
                    state.take_while(|c| c.is_ascii_alphanumeric() || c == '_');
                    state.add_token(ZigSyntaxKind::BuiltinIdentifier, start, state.get_position());
                    return true;
                }
            }
        }
        false
    }

    /// 解析操作符
    fn lex_operators<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // 尝试匹配最长的操作符
        let ops = [
            ("<<=", ZigSyntaxKind::LessLessAssign),
            (">>=", ZigSyntaxKind::GreaterGreaterAssign),
            ("...", ZigSyntaxKind::DotDotDot),
            ("==", ZigSyntaxKind::Equal),
            ("!=", ZigSyntaxKind::NotEqual),
            ("<=", ZigSyntaxKind::LessEqual),
            (">=", ZigSyntaxKind::GreaterEqual),
            ("&&", ZigSyntaxKind::AndAnd),
            ("||", ZigSyntaxKind::OrOr),
            ("+=", ZigSyntaxKind::PlusAssign),
            ("-=", ZigSyntaxKind::MinusAssign),
            ("*=", ZigSyntaxKind::StarAssign),
            ("/=", ZigSyntaxKind::SlashAssign),
            ("%=", ZigSyntaxKind::PercentAssign),
            ("&=", ZigSyntaxKind::AmpersandAssign),
            ("|=", ZigSyntaxKind::PipeAssign),
            ("^=", ZigSyntaxKind::CaretAssign),
            ("++", ZigSyntaxKind::PlusPlus),
            ("--", ZigSyntaxKind::MinusMinus),
            ("**", ZigSyntaxKind::StarStar),
            ("->", ZigSyntaxKind::Arrow),
            ("=>", ZigSyntaxKind::FatArrow),
            ("<<", ZigSyntaxKind::LessLess),
            (">>", ZigSyntaxKind::GreaterGreater),
            (".?", ZigSyntaxKind::DotQuestion),
            (".*", ZigSyntaxKind::DotStar),
        ];

        for (op, kind) in ops {
            if rest.starts_with(op) {
                state.advance(op.len());
                state.add_token(kind, start, state.get_position());
                return true;
            }
        }

        false
    }

    /// 解析单字符标记
    fn lex_single_char_tokens<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.current() {
            let kind = match ch {
                '(' => ZigSyntaxKind::LeftParen,
                ')' => ZigSyntaxKind::RightParen,
                '{' => ZigSyntaxKind::LeftBrace,
                '}' => ZigSyntaxKind::RightBrace,
                '[' => ZigSyntaxKind::LeftBracket,
                ']' => ZigSyntaxKind::RightBracket,
                ',' => ZigSyntaxKind::Comma,
                '.' => ZigSyntaxKind::Dot,
                ':' => ZigSyntaxKind::Colon,
                ';' => ZigSyntaxKind::Semicolon,
                '+' => ZigSyntaxKind::Plus,
                '-' => ZigSyntaxKind::Minus,
                '*' => ZigSyntaxKind::Star,
                '/' => ZigSyntaxKind::Slash,
                '%' => ZigSyntaxKind::Percent,
                '&' => ZigSyntaxKind::Ampersand,
                '|' => ZigSyntaxKind::Pipe,
                '^' => ZigSyntaxKind::Caret,
                '~' => ZigSyntaxKind::Tilde,
                '!' => ZigSyntaxKind::Exclamation,
                '?' => ZigSyntaxKind::Question,
                '<' => ZigSyntaxKind::Less,
                '>' => ZigSyntaxKind::Greater,
                '=' => ZigSyntaxKind::Assign,
                _ => return false,
            };
            state.advance(1);
            state.add_token(kind, start, state.get_position());
            return true;
        }
        false
    }
}
