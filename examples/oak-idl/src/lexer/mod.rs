use crate::{kind::IdlSyntaxKind, language::IdlLanguage};
use oak_core::{
    Lexer, LexerCache, LexerState, OakError,
    lexer::{LexOutput, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<'s, S> = LexerState<'s, S, IdlLanguage>;

static IDL_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });

#[derive(Clone)]
pub struct IdlLexer<'config> {
    _config: &'config IdlLanguage,
}

impl<'config> Lexer<IdlLanguage> for IdlLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[oak_core::TextEdit], cache: &'a mut impl LexerCache<IdlLanguage>) -> LexOutput<IdlLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> IdlLexer<'config> {
    pub fn new(config: &'config IdlLanguage) -> Self {
        Self { _config: config }
    }

    /// 主要的词法分析循环
    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
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

            if self.lex_number_literal(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_preprocessor(state) {
                continue;
            }

            if self.lex_operators(state) {
                continue;
            }

            if self.lex_single_char_tokens(state) {
                continue;
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        IDL_WHITESPACE.scan(state, IdlSyntaxKind::Whitespace)
    }

    /// 跳过注释
    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // 单行注释: // ... 直到换行
        if rest.starts_with("//") {
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(IdlSyntaxKind::Comment, start, state.get_position());
            return true;
        }

        // 多行注释: /* ... */
        if rest.starts_with("/*") {
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch == '*' && state.peek_next_n(1) == Some('/') {
                    state.advance(2);
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(IdlSyntaxKind::Comment, start, state.get_position());
            return true;
        }

        false
    }

    /// 处理字符串字面量
    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

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
            state.add_token(IdlSyntaxKind::StringLiteral, start, state.get_position());
            return true;
        }
        false
    }

    /// 处理数字字面量
    fn lex_number_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let first = match state.current() {
            Some(c) => c,
            None => return false,
        };

        if !first.is_ascii_digit() {
            return false;
        }

        // 处理十六进制数字
        if first == '0' && state.peek_next_n(1) == Some('x') {
            state.advance(2);
            while let Some(c) = state.peek() {
                if c.is_ascii_hexdigit() {
                    state.advance(1);
                }
                else {
                    break;
                }
            }
        }
        else {
            // 处理十进制数字
            state.advance(1);
            while let Some(c) = state.peek() {
                if c.is_ascii_digit() {
                    state.advance(1);
                }
                else {
                    break;
                }
            }

            // 处理小数点
            if state.peek() == Some('.') && state.peek_next_n(1).map(|c| c.is_ascii_digit()).unwrap_or(false) {
                state.advance(1); // consume '.'
                while let Some(c) = state.peek() {
                    if c.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }
            }

            // 处理指数
            if let Some(c) = state.peek() {
                if c == 'e' || c == 'E' {
                    let n1 = state.peek_next_n(1);
                    if n1 == Some('+') || n1 == Some('-') || n1.map(|d| d.is_ascii_digit()).unwrap_or(false) {
                        state.advance(1);
                        if let Some(sign) = state.peek() {
                            if sign == '+' || sign == '-' {
                                state.advance(1);
                            }
                        }
                        while let Some(d) = state.peek() {
                            if d.is_ascii_digit() {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                    }
                }
            }
        }

        state.add_token(IdlSyntaxKind::NumberLiteral, start, state.get_position());
        true
    }

    /// 处理标识符和关键字
    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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
        let kind = match text.as_ref() {
            // 基本数据类型
            "void" => IdlSyntaxKind::Void,
            "boolean" => IdlSyntaxKind::Boolean,
            "byte" => IdlSyntaxKind::Byte,
            "octet" => IdlSyntaxKind::Octet,
            "short" => IdlSyntaxKind::Short,
            "unsigned" => IdlSyntaxKind::UnsignedShort, // 简化处理
            "long" => IdlSyntaxKind::Long,
            "float" => IdlSyntaxKind::Float,
            "double" => IdlSyntaxKind::Double,
            "char" => IdlSyntaxKind::Char,
            "wchar" => IdlSyntaxKind::WChar,
            "string" => IdlSyntaxKind::String,
            "wstring" => IdlSyntaxKind::WString,
            "any" => IdlSyntaxKind::Any,
            "Object" => IdlSyntaxKind::Object,
            "ValueBase" => IdlSyntaxKind::ValueBase,

            // 复合类型关键字
            "struct" => IdlSyntaxKind::Struct,
            "union" => IdlSyntaxKind::Union,
            "enum" => IdlSyntaxKind::Enum,
            "interface" => IdlSyntaxKind::Interface,
            "module" => IdlSyntaxKind::Module,
            "exception" => IdlSyntaxKind::Exception,
            "typedef" => IdlSyntaxKind::Typedef,
            "sequence" => IdlSyntaxKind::Sequence,
            "fixed" => IdlSyntaxKind::Fixed,

            // 修饰符
            "const" => IdlSyntaxKind::Const,
            "readonly" => IdlSyntaxKind::Readonly,
            "attribute" => IdlSyntaxKind::Attribute,
            "oneway" => IdlSyntaxKind::Oneway,
            "in" => IdlSyntaxKind::In,
            "out" => IdlSyntaxKind::Out,
            "inout" => IdlSyntaxKind::Inout,
            "raises" => IdlSyntaxKind::Raises,
            "context" => IdlSyntaxKind::Context,
            "local" => IdlSyntaxKind::Local,
            "abstract" => IdlSyntaxKind::Abstract,
            "custom" => IdlSyntaxKind::Custom,
            "private" => IdlSyntaxKind::Private,
            "public" => IdlSyntaxKind::Public,
            "truncatable" => IdlSyntaxKind::Truncatable,
            "supports" => IdlSyntaxKind::Supports,
            "valuetype" => IdlSyntaxKind::ValueType,
            "native" => IdlSyntaxKind::Native,
            "factory" => IdlSyntaxKind::Factory,

            // 布尔字面量
            "TRUE" | "FALSE" => IdlSyntaxKind::BooleanLiteral,

            _ => IdlSyntaxKind::Identifier,
        };

        state.add_token(kind, start, state.get_position());
        true
    }

    /// 处理预处理器指令
    fn lex_preprocessor<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if state.current() != Some('#') {
            return false;
        }

        state.advance(1);

        // 跳过空白
        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' {
                state.advance(1);
            }
            else {
                break;
            }
        }

        // 读取指令名称
        let directive_start = state.get_position();
        while let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(1);
            }
            else {
                break;
            }
        }

        let kind = if state.get_position() > directive_start {
            let directive = state.get_text_in((directive_start..state.get_position()).into());
            match directive.as_ref() {
                "include" => IdlSyntaxKind::Include,
                "pragma" => IdlSyntaxKind::Pragma,
                "define" => IdlSyntaxKind::Define,
                "ifdef" => IdlSyntaxKind::Ifdef,
                "ifndef" => IdlSyntaxKind::Ifndef,
                "endif" => IdlSyntaxKind::Endif,
                "else" => IdlSyntaxKind::Else,
                "elif" => IdlSyntaxKind::Elif,
                "undef" => IdlSyntaxKind::Undef,
                _ => IdlSyntaxKind::Hash,
            }
        }
        else {
            IdlSyntaxKind::Hash
        };

        // 读取到行尾
        while let Some(ch) = state.peek() {
            if ch == '\n' || ch == '\r' {
                break;
            }
            state.advance(ch.len_utf8());
        }

        state.add_token(kind, start, state.get_position());
        true
    }

    /// 处理操作符
    fn lex_operators<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // 优先匹配较长的操作符
        let patterns: &[(&str, IdlSyntaxKind)] = &[
            ("::", IdlSyntaxKind::DoubleColon),
            ("<<", IdlSyntaxKind::LeftShift),
            (">>", IdlSyntaxKind::RightShift),
            ("<=", IdlSyntaxKind::LessEqual),
            (">=", IdlSyntaxKind::GreaterEqual),
            ("==", IdlSyntaxKind::Equal),
            ("!=", IdlSyntaxKind::NotEqual),
            ("&&", IdlSyntaxKind::LogicalAnd),
            ("||", IdlSyntaxKind::LogicalOr),
            ("->", IdlSyntaxKind::Arrow),
        ];

        for (pat, kind) in patterns {
            if rest.starts_with(pat) {
                state.advance(pat.len());
                state.add_token(*kind, start, state.get_position());
                return true;
            }
        }

        if let Some(ch) = state.current() {
            let kind = match ch {
                '+' => Some(IdlSyntaxKind::Plus),
                '-' => Some(IdlSyntaxKind::Minus),
                '*' => Some(IdlSyntaxKind::Multiply),
                '/' => Some(IdlSyntaxKind::Divide),
                '%' => Some(IdlSyntaxKind::Modulo),
                '&' => Some(IdlSyntaxKind::BitwiseAnd),
                '|' => Some(IdlSyntaxKind::BitwiseOr),
                '^' => Some(IdlSyntaxKind::BitwiseXor),
                '~' => Some(IdlSyntaxKind::BitwiseNot),
                '!' => Some(IdlSyntaxKind::LogicalNot),
                '=' => Some(IdlSyntaxKind::Assign),
                '<' => Some(IdlSyntaxKind::Less),
                '>' => Some(IdlSyntaxKind::Greater),
                '.' => Some(IdlSyntaxKind::Dot),
                ':' => Some(IdlSyntaxKind::Colon),
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

    /// 处理单字符标记
    fn lex_single_char_tokens<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.current() {
            let kind = match ch {
                '(' => IdlSyntaxKind::LeftParen,
                ')' => IdlSyntaxKind::RightParen,
                '{' => IdlSyntaxKind::LeftBrace,
                '}' => IdlSyntaxKind::RightBrace,
                '[' => IdlSyntaxKind::LeftBracket,
                ']' => IdlSyntaxKind::RightBracket,
                ',' => IdlSyntaxKind::Comma,
                ';' => IdlSyntaxKind::Semicolon,
                '#' => IdlSyntaxKind::Hash,
                _ => return false,
            };

            state.advance(ch.len_utf8());
            state.add_token(kind, start, state.get_position());
            return true;
        }

        false
    }
}
