#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::IdlLanguage, lexer::token_type::IdlTokenType};
use oak_core::{
    Lexer, LexerCache, LexerState, OakError, Source,
    lexer::{LexOutput, WhitespaceConfig},
    source::TextEdit,
};
use std::sync::LazyLock;

type State<'s, S> = LexerState<'s, S, IdlLanguage>;

static IDL_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });

#[derive(Clone, Debug)]
pub struct IdlLexer<'config> {
    config: &'config IdlLanguage,
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
        Self { config }
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

            state.advance_if_dead_lock(safe_point)
        }

        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        IDL_WHITESPACE.scan(state, IdlTokenType::Whitespace)
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
            state.add_token(IdlTokenType::Comment, start, state.get_position());
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
            state.add_token(IdlTokenType::Comment, start, state.get_position());
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
            state.add_token(IdlTokenType::StringLiteral, start, state.get_position());
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

        state.add_token(IdlTokenType::NumberLiteral, start, state.get_position());
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
            "void" => IdlTokenType::Void,
            "boolean" => IdlTokenType::Boolean,
            "byte" => IdlTokenType::Byte,
            "octet" => IdlTokenType::Octet,
            "short" => IdlTokenType::Short,
            "unsigned" => IdlTokenType::UnsignedShort, // 简化处理
            "long" => IdlTokenType::Long,
            "float" => IdlTokenType::Float,
            "double" => IdlTokenType::Double,
            "char" => IdlTokenType::Char,
            "wchar" => IdlTokenType::WChar,
            "string" => IdlTokenType::String,
            "wstring" => IdlTokenType::WString,
            "any" => IdlTokenType::Any,
            "Object" => IdlTokenType::Object,
            "ValueBase" => IdlTokenType::ValueBase,

            // 复合类型关键字
            "struct" => IdlTokenType::Struct,
            "union" => IdlTokenType::Union,
            "enum" => IdlTokenType::Enum,
            "interface" => IdlTokenType::Interface,
            "module" => IdlTokenType::Module,
            "exception" => IdlTokenType::Exception,
            "typedef" => IdlTokenType::Typedef,
            "sequence" => IdlTokenType::Sequence,
            "fixed" => IdlTokenType::Fixed,

            // 修饰符
            "const" => IdlTokenType::Const,
            "readonly" => IdlTokenType::Readonly,
            "attribute" => IdlTokenType::Attribute,
            "oneway" => IdlTokenType::Oneway,
            "in" => IdlTokenType::In,
            "out" => IdlTokenType::Out,
            "inout" => IdlTokenType::Inout,
            "raises" => IdlTokenType::Raises,
            "context" => IdlTokenType::Context,
            "local" => IdlTokenType::Local,
            "abstract" => IdlTokenType::Abstract,
            "custom" => IdlTokenType::Custom,
            "private" => IdlTokenType::Private,
            "public" => IdlTokenType::Public,
            "truncatable" => IdlTokenType::Truncatable,
            "supports" => IdlTokenType::Supports,
            "valuetype" => IdlTokenType::ValueType,
            "native" => IdlTokenType::Native,
            "factory" => IdlTokenType::Factory,

            // 布尔字面量
            "TRUE" | "FALSE" => IdlTokenType::BooleanLiteral,

            _ => IdlTokenType::Identifier,
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
                "include" => IdlTokenType::Include,
                "pragma" => IdlTokenType::Pragma,
                "define" => IdlTokenType::Define,
                "ifdef" => IdlTokenType::Ifdef,
                "ifndef" => IdlTokenType::Ifndef,
                "endif" => IdlTokenType::Endif,
                "else" => IdlTokenType::Else,
                "elif" => IdlTokenType::Elif,
                "undef" => IdlTokenType::Undef,
                _ => IdlTokenType::Hash,
            }
        }
        else {
            IdlTokenType::Hash
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
        let patterns: &[(&str, IdlTokenType)] = &[
            ("::", IdlTokenType::DoubleColon),
            ("<<", IdlTokenType::LeftShift),
            (">>", IdlTokenType::RightShift),
            ("<=", IdlTokenType::LessEqual),
            (">=", IdlTokenType::GreaterEqual),
            ("==", IdlTokenType::Equal),
            ("!=", IdlTokenType::NotEqual),
            ("&&", IdlTokenType::LogicalAnd),
            ("||", IdlTokenType::LogicalOr),
            ("->", IdlTokenType::Arrow),
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
                '+' => Some(IdlTokenType::Plus),
                '-' => Some(IdlTokenType::Minus),
                '*' => Some(IdlTokenType::Multiply),
                '/' => Some(IdlTokenType::Divide),
                '%' => Some(IdlTokenType::Modulo),
                '&' => Some(IdlTokenType::BitwiseAnd),
                '|' => Some(IdlTokenType::BitwiseOr),
                '^' => Some(IdlTokenType::BitwiseXor),
                '~' => Some(IdlTokenType::BitwiseNot),
                '!' => Some(IdlTokenType::LogicalNot),
                '=' => Some(IdlTokenType::Assign),
                '<' => Some(IdlTokenType::Less),
                '>' => Some(IdlTokenType::Greater),
                '.' => Some(IdlTokenType::Dot),
                ':' => Some(IdlTokenType::Colon),
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
                '(' => IdlTokenType::LeftParen,
                ')' => IdlTokenType::RightParen,
                '{' => IdlTokenType::LeftBrace,
                '}' => IdlTokenType::RightBrace,
                '[' => IdlTokenType::LeftBracket,
                ']' => IdlTokenType::RightBracket,
                ',' => IdlTokenType::Comma,
                ';' => IdlTokenType::Semicolon,
                '#' => IdlTokenType::Hash,
                _ => return false,
            };

            state.advance(ch.len_utf8());
            state.add_token(kind, start, state.get_position());
            return true;
        }

        false
    }
}
