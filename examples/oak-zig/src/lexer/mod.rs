use crate::{kind::ZigSyntaxKind, language::ZigLanguage};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{CommentLine, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S: Source> = LexerState<S, ZigLanguage>;

static ZIG_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static ZIG_COMMENT: LazyLock<CommentLine> = LazyLock::new(|| CommentLine { line_markers: &["//"] });
static ZIG_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });
static ZIG_CHAR: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['\''], escape: Some('\\') });

#[derive(Clone)]
pub struct ZigLexer<'config> {
    config: &'config ZigLanguage,
}

impl<'config> Lexer<ZigLanguage> for ZigLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<ZigLanguage>,
    ) -> LexOutput<ZigLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> ZigLexer<'config> {
    pub fn new(config: &'config ZigLanguage) -> Self {
        Self { config }
    }

    /// 主要的词法分析循环
    fn run<S: Source>(&self, state: &mut State<S>) -> Result<(), OakError> {
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

            state.safe_check(safe_point);
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(ZigSyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        match ZIG_WHITESPACE.scan(state.rest(), state.get_position(), ZigSyntaxKind::Whitespace) {
            Some(token) => {
                state.advance_with(token);
                return true;
            }
            None => {}
        }
        false
    }

    /// 跳过注释
    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
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
    fn lex_string_literal<S: Source>(&self, state: &mut State<S>) -> bool {
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
            let mut escaped = false;

            while let Some(ch) = state.peek() {
                if ch == '"' && !escaped {
                    state.advance(1); // consume closing quote
                    break;
                }

                state.advance(ch.len_utf8());

                if escaped {
                    escaped = false;
                    continue;
                }

                if ch == '\\' {
                    escaped = true;
                    continue;
                }

                if ch == '\n' || ch == '\r' {
                    break;
                }
            }

            state.add_token(ZigSyntaxKind::StringLiteral, start, state.get_position());
            return true;
        }

        false
    }

    /// 解析字符字面量
    fn lex_char_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if state.current() != Some('\'') {
            return false;
        }

        state.advance(1); // opening '

        if let Some('\\') = state.peek() {
            state.advance(1);
            if let Some(c) = state.peek() {
                state.advance(c.len_utf8());
            }
        }
        else if let Some(c) = state.peek() {
            state.advance(c.len_utf8());
        }
        else {
            state.set_position(start);
            return false;
        }

        if state.peek() == Some('\'') {
            state.advance(1);
            state.add_token(ZigSyntaxKind::CharLiteral, start, state.get_position());
            return true;
        }

        state.set_position(start);
        false
    }

    /// 解析数字字面量
    fn lex_number_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let first = match state.current() {
            Some(c) => c,
            None => return false,
        };

        if !first.is_ascii_digit() {
            return false;
        }

        let mut is_float = false;

        // 处理不同进制
        if first == '0' {
            match state.peek_next_n(1) {
                Some('x') | Some('X') => {
                    state.advance(2);
                    while let Some(c) = state.peek() {
                        if c.is_ascii_hexdigit() || c == '_' {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }
                Some('b') | Some('B') => {
                    state.advance(2);
                    while let Some(c) = state.peek() {
                        if c == '0' || c == '1' || c == '_' {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }
                Some('o') | Some('O') => {
                    state.advance(2);
                    while let Some(c) = state.peek() {
                        if ('0'..='7').contains(&c) || c == '_' {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }
                _ => {
                    state.advance(1);
                    while let Some(c) = state.peek() {
                        if c.is_ascii_digit() || c == '_' {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }
            }
        }
        else {
            state.advance(1);
            while let Some(c) = state.peek() {
                if c.is_ascii_digit() || c == '_' {
                    state.advance(1);
                }
                else {
                    break;
                }
            }
        }

        // 小数部分
        if state.peek() == Some('.') {
            let n1 = state.peek_next_n(1);
            if n1.map(|c| c.is_ascii_digit()).unwrap_or(false) {
                is_float = true;
                state.advance(1); // consume '.'
                while let Some(c) = state.peek() {
                    if c.is_ascii_digit() || c == '_' {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }
            }
        }

        // 指数部分
        if let Some(c) = state.peek() {
            if c == 'e' || c == 'E' {
                let n1 = state.peek_next_n(1);
                if n1 == Some('+') || n1 == Some('-') || n1.map(|d| d.is_ascii_digit()).unwrap_or(false) {
                    is_float = true;
                    state.advance(1);
                    if let Some(sign) = state.peek() {
                        if sign == '+' || sign == '-' {
                            state.advance(1);
                        }
                    }
                    while let Some(d) = state.peek() {
                        if d.is_ascii_digit() || d == '_' {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }
            }
        }

        let end = state.get_position();
        state.add_token(if is_float { ZigSyntaxKind::FloatLiteral } else { ZigSyntaxKind::IntegerLiteral }, start, end);
        true
    }

    /// 解析标识符或关键字
    fn lex_identifier_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let ch = match state.current() {
            Some(c) => c,
            None => return false,
        };

        if !(ch.is_ascii_alphabetic() || ch == '_') {
            return false;
        }

        state.advance(1);
        while let Some(c) = state.current() {
            if c.is_ascii_alphanumeric() || c == '_' {
                state.advance(1);
            }
            else {
                break;
            }
        }

        let end = state.get_position();
        let text = state.get_text_in((start..end).into());
        let kind = self.get_keyword_or_identifier(text);
        state.add_token(kind, start, state.get_position());
        true
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
            "comptime_int" => ZigSyntaxKind::Comptime_Int,
            "comptime_float" => ZigSyntaxKind::Comptime_Float,

            // 布尔字面量
            "true" | "false" => ZigSyntaxKind::BooleanLiteral,

            _ => ZigSyntaxKind::Identifier,
        }
    }

    /// 解析内置函数 @...
    fn lex_builtin<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if state.current() != Some('@') {
            return false;
        }

        state.advance(1); // consume '@'

        // 读取内置函数名
        while let Some(c) = state.peek() {
            if c.is_ascii_alphanumeric() || c == '_' {
                state.advance(1);
            }
            else {
                break;
            }
        }

        state.add_token(ZigSyntaxKind::At, start, state.get_position());
        true
    }

    /// 解析操作符
    fn lex_operators<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // 优先匹配较长的操作符
        let patterns: &[(&str, ZigSyntaxKind)] = &[
            ("**", ZigSyntaxKind::StarStar),
            ("+%", ZigSyntaxKind::PlusPercent),
            ("-%", ZigSyntaxKind::MinusPercent),
            ("*%", ZigSyntaxKind::StarPercent),
            ("++", ZigSyntaxKind::PlusPlus),
            ("<<", ZigSyntaxKind::LessLess),
            (">>", ZigSyntaxKind::GreaterGreater),
            ("==", ZigSyntaxKind::Equal),
            ("!=", ZigSyntaxKind::NotEqual),
            ("<=", ZigSyntaxKind::LessEqual),
            (">=", ZigSyntaxKind::GreaterEqual),
            ("+=", ZigSyntaxKind::PlusAssign),
            ("-=", ZigSyntaxKind::MinusAssign),
            ("*=", ZigSyntaxKind::StarAssign),
            ("/=", ZigSyntaxKind::SlashAssign),
            ("%=", ZigSyntaxKind::PercentAssign),
            ("&=", ZigSyntaxKind::AmpersandAssign),
            ("|=", ZigSyntaxKind::PipeAssign),
            ("^=", ZigSyntaxKind::CaretAssign),
            ("<<=", ZigSyntaxKind::LessLessAssign),
            (">>=", ZigSyntaxKind::GreaterGreaterAssign),
            ("...", ZigSyntaxKind::DotDotDot),
            ("..", ZigSyntaxKind::DotDot),
            ("=>", ZigSyntaxKind::FatArrow),
        ];

        for (pat, kind) in patterns {
            if rest.starts_with(pat) {
                state.advance(pat.len());
                state.add_token(*kind, start, state.get_position());
                return true;
            }
        }

        // 单字符操作符
        if let Some(ch) = state.current() {
            let kind = match ch {
                '+' => Some(ZigSyntaxKind::Plus),
                '-' => Some(ZigSyntaxKind::Minus),
                '*' => Some(ZigSyntaxKind::Star),
                '/' => Some(ZigSyntaxKind::Slash),
                '%' => Some(ZigSyntaxKind::Percent),
                '&' => Some(ZigSyntaxKind::Ampersand),
                '|' => Some(ZigSyntaxKind::Pipe),
                '^' => Some(ZigSyntaxKind::Caret),
                '~' => Some(ZigSyntaxKind::Tilde),
                '=' => Some(ZigSyntaxKind::Assign),
                '<' => Some(ZigSyntaxKind::Less),
                '>' => Some(ZigSyntaxKind::Greater),
                '.' => Some(ZigSyntaxKind::Dot),
                '!' => Some(ZigSyntaxKind::Exclamation),
                '?' => Some(ZigSyntaxKind::Question),
                _ => None,
            };

            if let Some(k) = kind {
                state.advance(ch.len_utf8());
                state.add_token(k, start, state.get_position());
                return true;
            }
        }

        false
    }

    /// 解析单字符token
    fn lex_single_char_tokens<S: Source>(&self, state: &mut State<S>) -> bool {
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
                ';' => ZigSyntaxKind::Semicolon,
                ':' => ZigSyntaxKind::Colon,
                _ => return false,
            };

            state.advance(ch.len_utf8());
            state.add_token(kind, start, state.get_position());
            return true;
        }

        false
    }
}
