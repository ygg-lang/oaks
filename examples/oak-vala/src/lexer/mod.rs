#![doc = include_str!("readme.md")]
use oak_core::Source;
pub mod token_type;

use crate::{language::ValaLanguage, lexer::token_type::ValaTokenType};
use oak_core::{
    Lexer, LexerCache, LexerState, OakError,
    lexer::{CommentConfig, LexOutput, StringConfig, WhitespaceConfig},
};
use std::sync::LazyLock;

type State<'a, S> = LexerState<'a, S, ValaLanguage>;

static VALA_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static VALA_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: "//", block_start: "/*", block_end: "*/", nested_blocks: true });
static VALA_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });
static VALA_CHAR: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['\''], escape: Some('\\') });

#[derive(Clone, Debug)]
pub struct ValaLexer<'config> {
    _config: &'config ValaLanguage,
}

impl<'config> Lexer<ValaLanguage> for ValaLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[oak_core::TextEdit], cache: &'a mut impl LexerCache<ValaLanguage>) -> LexOutput<ValaLanguage> {
        let mut state: State<'_, S> = LexerState::new(source);
        let result = self.run(&mut state);
        state.finish_with_cache(result, cache)
    }
}

impl<'config> ValaLexer<'config> {
    pub fn new(config: &'config ValaLanguage) -> Self {
        Self { _config: config }
    }

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

            if self.lex_operators(state) {
                continue;
            }

            if self.lex_single_char_tokens(state) {
                continue;
            }

            state.advance_if_dead_lock(safe_point);
        }

        // 添加 EOF token
        state.add_eof();
        Ok(())
    }

    fn skip_whitespace<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        VALA_WHITESPACE.scan(state, ValaTokenType::Whitespace)
    }

    fn skip_comment<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        VALA_COMMENT.scan(state, ValaTokenType::LineComment, ValaTokenType::BlockComment)
    }

    fn lex_string_literal<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        VALA_STRING.scan(state, ValaTokenType::StringLiteral)
    }

    fn lex_char_literal<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        VALA_CHAR.scan(state, ValaTokenType::CharLiteral)
    }

    fn lex_number_literal<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();
        let first = match state.peek() {
            Some(c) => c,
            None => return false,
        };

        if !first.is_ascii_digit() {
            return false;
        }

        let mut is_float = false;

        // 处理十六进制、八进制、二进制
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

        // 后缀字母 (e.g., f, d, l)
        while let Some(c) = state.peek() {
            if c.is_ascii_alphabetic() {
                state.advance(1);
            }
            else {
                break;
            }
        }

        let end = state.get_position();
        state.add_token(if is_float { ValaTokenType::FloatLiteral } else { ValaTokenType::IntegerLiteral }, start, end);
        true
    }

    fn lex_identifier_or_keyword<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();
        let ch = match state.peek() {
            Some(c) => c,
            None => return false,
        };

        if !(ch.is_ascii_alphabetic() || ch == '_') {
            return false;
        }

        state.advance(ch.len_utf8());
        while let Some(c) = state.peek() {
            if c.is_ascii_alphanumeric() || c == '_' {
                state.advance(c.len_utf8());
            }
            else {
                break;
            }
        }

        let end = state.get_position();
        let text = state.get_text_in(oak_core::Range { start, end });
        let kind = match text.as_ref() {
            "abstract" => ValaTokenType::AbstractKw,
            "as" => ValaTokenType::AsKw,
            "base" => ValaTokenType::BaseKw,
            "break" => ValaTokenType::BreakKw,
            "case" => ValaTokenType::CaseKw,
            "catch" => ValaTokenType::CatchKw,
            "class" => ValaTokenType::ClassKw,
            "const" => ValaTokenType::ConstKw,
            "construct" => ValaTokenType::ConstructKw,
            "continue" => ValaTokenType::ContinueKw,
            "default" => ValaTokenType::DefaultKw,
            "delegate" => ValaTokenType::DelegateKw,
            "delete" => ValaTokenType::DeleteKw,
            "do" => ValaTokenType::DoKw,
            "else" => ValaTokenType::ElseKw,
            "enum" => ValaTokenType::EnumKw,
            "ensures" => ValaTokenType::EnsuresKw,
            "errordomain" => ValaTokenType::ErrordomainKw,
            "extern" => ValaTokenType::ExternKw,
            "false" => ValaTokenType::FalseKw,
            "finally" => ValaTokenType::FinallyKw,
            "for" => ValaTokenType::ForKw,
            "foreach" => ValaTokenType::ForeachKw,
            "get" => ValaTokenType::GetKw,
            "if" => ValaTokenType::IfKw,
            "in" => ValaTokenType::InKw,
            "inline" => ValaTokenType::InlineKw,
            "interface" => ValaTokenType::InterfaceKw,
            "internal" => ValaTokenType::InternalKw,
            "is" => ValaTokenType::IsKw,
            "lock" => ValaTokenType::LockKw,
            "namespace" => ValaTokenType::NamespaceKw,
            "new" => ValaTokenType::NewKw,
            "null" => ValaTokenType::NullKw,
            "out" => ValaTokenType::OutKw,
            "override" => ValaTokenType::OverrideKw,
            "owned" => ValaTokenType::OwnedKw,
            "private" => ValaTokenType::PrivateKw,
            "protected" => ValaTokenType::ProtectedKw,
            "public" => ValaTokenType::PublicKw,
            "ref" => ValaTokenType::RefKw,
            "requires" => ValaTokenType::RequiresKw,
            "return" => ValaTokenType::ReturnKw,
            "set" => ValaTokenType::SetKw,
            "sizeof" => ValaTokenType::SizeofKw,
            "static" => ValaTokenType::StaticKw,
            "struct" => ValaTokenType::StructKw,
            "switch" => ValaTokenType::SwitchKw,
            "this" => ValaTokenType::ThisKw,
            "throw" => ValaTokenType::ThrowKw,
            "throws" => ValaTokenType::ThrowsKw,
            "true" => ValaTokenType::TrueKw,
            "try" => ValaTokenType::TryKw,
            "typeof" => ValaTokenType::TypeofKw,
            "unowned" => ValaTokenType::UnownedKw,
            "using" => ValaTokenType::UsingKw,
            "var" => ValaTokenType::VarKw,
            "virtual" => ValaTokenType::VirtualKw,
            "void" => ValaTokenType::VoidKw,
            "volatile" => ValaTokenType::VolatileKw,
            "weak" => ValaTokenType::WeakKw,
            "while" => ValaTokenType::WhileKw,
            "yield" => ValaTokenType::YieldKw,
            // 基本类型
            "bool" => ValaTokenType::BoolKw,
            "char" => ValaTokenType::CharKw,
            "uchar" => ValaTokenType::UcharKw,
            "int" => ValaTokenType::IntKw,
            "uint" => ValaTokenType::UintKw,
            "short" => ValaTokenType::ShortKw,
            "ushort" => ValaTokenType::UshortKw,
            "long" => ValaTokenType::LongKw,
            "ulong" => ValaTokenType::UlongKw,
            "int8" => ValaTokenType::Int8Kw,
            "uint8" => ValaTokenType::Uint8Kw,
            "int16" => ValaTokenType::Int16Kw,
            "uint16" => ValaTokenType::Uint16Kw,
            "int32" => ValaTokenType::Int32Kw,
            "uint32" => ValaTokenType::Uint32Kw,
            "int64" => ValaTokenType::Int64Kw,
            "uint64" => ValaTokenType::Uint64Kw,
            "float" => ValaTokenType::FloatKw,
            "double" => ValaTokenType::DoubleKw,
            "string" => ValaTokenType::StringKw,
            _ => ValaTokenType::Identifier,
        };

        state.add_token(kind, start, state.get_position());
        true
    }

    fn lex_operators<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();

        // 优先匹配较长的操作符
        let patterns: &[(&str, ValaTokenType)] = &[
            ("<<", ValaTokenType::LeftShift),
            (">>", ValaTokenType::RightShift),
            ("==", ValaTokenType::EqEq),
            ("!=", ValaTokenType::NotEq),
            ("<=", ValaTokenType::LessEq),
            (">=", ValaTokenType::GreaterEq),
            ("&&", ValaTokenType::AndAnd),
            ("||", ValaTokenType::OrOr),
            ("++", ValaTokenType::PlusPlus),
            ("--", ValaTokenType::MinusMinus),
            ("+=", ValaTokenType::PlusEq),
            ("-=", ValaTokenType::MinusEq),
            ("*=", ValaTokenType::StarEq),
            ("/=", ValaTokenType::SlashEq),
            ("%=", ValaTokenType::PercentEq),
            ("->", ValaTokenType::Arrow),
        ];

        for (pat, kind) in patterns {
            if state.starts_with(pat) {
                state.advance(pat.len());
                state.add_token(*kind, start, state.get_position());
                return true;
            }
        }

        if let Some(ch) = state.current() {
            let kind = match ch {
                '+' => Some(ValaTokenType::Plus),
                '-' => Some(ValaTokenType::Minus),
                '*' => Some(ValaTokenType::Star),
                '/' => Some(ValaTokenType::Slash),
                '%' => Some(ValaTokenType::Percent),
                '^' => Some(ValaTokenType::Caret),
                '!' => Some(ValaTokenType::Bang),
                '&' => Some(ValaTokenType::Ampersand),
                '|' => Some(ValaTokenType::Pipe),
                '=' => Some(ValaTokenType::Eq),
                '>' => Some(ValaTokenType::GreaterThan),
                '<' => Some(ValaTokenType::LessThan),
                '.' => Some(ValaTokenType::Dot),
                ':' => Some(ValaTokenType::Colon),
                '?' => Some(ValaTokenType::Question),
                '~' => Some(ValaTokenType::Tilde),
                '\\' => Some(ValaTokenType::Backslash),
                '@' => Some(ValaTokenType::At),
                '#' => Some(ValaTokenType::Hash),
                '$' => Some(ValaTokenType::Dollar),
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

    fn lex_single_char_tokens<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.current() {
            let kind = match ch {
                '(' => Some(ValaTokenType::LeftParen),
                ')' => Some(ValaTokenType::RightParen),
                '{' => Some(ValaTokenType::LeftBrace),
                '}' => Some(ValaTokenType::RightBrace),
                '[' => Some(ValaTokenType::LeftBracket),
                ']' => Some(ValaTokenType::RightBracket),
                ',' => Some(ValaTokenType::Comma),
                ';' => Some(ValaTokenType::Semicolon),
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
}
