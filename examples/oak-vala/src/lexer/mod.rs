use crate::{kind::ValaSyntaxKind, language::ValaLanguage};
use oak_core::{
    Lexer, LexerCache, LexerState, OakError,
    lexer::{CommentConfig, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
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
        VALA_WHITESPACE.scan(state, ValaSyntaxKind::Whitespace)
    }

    fn skip_comment<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        VALA_COMMENT.scan(state, ValaSyntaxKind::LineComment, ValaSyntaxKind::BlockComment)
    }

    fn lex_string_literal<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        VALA_STRING.scan(state, ValaSyntaxKind::StringLiteral)
    }

    fn lex_char_literal<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        VALA_CHAR.scan(state, ValaSyntaxKind::CharLiteral)
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
        state.add_token(if is_float { ValaSyntaxKind::FloatLiteral } else { ValaSyntaxKind::IntegerLiteral }, start, end);
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
            "abstract" => ValaSyntaxKind::AbstractKw,
            "as" => ValaSyntaxKind::AsKw,
            "base" => ValaSyntaxKind::BaseKw,
            "break" => ValaSyntaxKind::BreakKw,
            "case" => ValaSyntaxKind::CaseKw,
            "catch" => ValaSyntaxKind::CatchKw,
            "class" => ValaSyntaxKind::ClassKw,
            "const" => ValaSyntaxKind::ConstKw,
            "construct" => ValaSyntaxKind::ConstructKw,
            "continue" => ValaSyntaxKind::ContinueKw,
            "default" => ValaSyntaxKind::DefaultKw,
            "delegate" => ValaSyntaxKind::DelegateKw,
            "delete" => ValaSyntaxKind::DeleteKw,
            "do" => ValaSyntaxKind::DoKw,
            "else" => ValaSyntaxKind::ElseKw,
            "enum" => ValaSyntaxKind::EnumKw,
            "ensures" => ValaSyntaxKind::EnsuresKw,
            "errordomain" => ValaSyntaxKind::ErrordomainKw,
            "extern" => ValaSyntaxKind::ExternKw,
            "false" => ValaSyntaxKind::FalseKw,
            "finally" => ValaSyntaxKind::FinallyKw,
            "for" => ValaSyntaxKind::ForKw,
            "foreach" => ValaSyntaxKind::ForeachKw,
            "get" => ValaSyntaxKind::GetKw,
            "if" => ValaSyntaxKind::IfKw,
            "in" => ValaSyntaxKind::InKw,
            "inline" => ValaSyntaxKind::InlineKw,
            "interface" => ValaSyntaxKind::InterfaceKw,
            "internal" => ValaSyntaxKind::InternalKw,
            "is" => ValaSyntaxKind::IsKw,
            "lock" => ValaSyntaxKind::LockKw,
            "namespace" => ValaSyntaxKind::NamespaceKw,
            "new" => ValaSyntaxKind::NewKw,
            "null" => ValaSyntaxKind::NullKw,
            "out" => ValaSyntaxKind::OutKw,
            "override" => ValaSyntaxKind::OverrideKw,
            "owned" => ValaSyntaxKind::OwnedKw,
            "private" => ValaSyntaxKind::PrivateKw,
            "protected" => ValaSyntaxKind::ProtectedKw,
            "public" => ValaSyntaxKind::PublicKw,
            "ref" => ValaSyntaxKind::RefKw,
            "requires" => ValaSyntaxKind::RequiresKw,
            "return" => ValaSyntaxKind::ReturnKw,
            "set" => ValaSyntaxKind::SetKw,
            "sizeof" => ValaSyntaxKind::SizeofKw,
            "static" => ValaSyntaxKind::StaticKw,
            "struct" => ValaSyntaxKind::StructKw,
            "switch" => ValaSyntaxKind::SwitchKw,
            "this" => ValaSyntaxKind::ThisKw,
            "throw" => ValaSyntaxKind::ThrowKw,
            "throws" => ValaSyntaxKind::ThrowsKw,
            "true" => ValaSyntaxKind::TrueKw,
            "try" => ValaSyntaxKind::TryKw,
            "typeof" => ValaSyntaxKind::TypeofKw,
            "unowned" => ValaSyntaxKind::UnownedKw,
            "using" => ValaSyntaxKind::UsingKw,
            "var" => ValaSyntaxKind::VarKw,
            "virtual" => ValaSyntaxKind::VirtualKw,
            "void" => ValaSyntaxKind::VoidKw,
            "volatile" => ValaSyntaxKind::VolatileKw,
            "weak" => ValaSyntaxKind::WeakKw,
            "while" => ValaSyntaxKind::WhileKw,
            "yield" => ValaSyntaxKind::YieldKw,
            // 基本类型
            "bool" => ValaSyntaxKind::BoolKw,
            "char" => ValaSyntaxKind::CharKw,
            "uchar" => ValaSyntaxKind::UcharKw,
            "int" => ValaSyntaxKind::IntKw,
            "uint" => ValaSyntaxKind::UintKw,
            "short" => ValaSyntaxKind::ShortKw,
            "ushort" => ValaSyntaxKind::UshortKw,
            "long" => ValaSyntaxKind::LongKw,
            "ulong" => ValaSyntaxKind::UlongKw,
            "int8" => ValaSyntaxKind::Int8Kw,
            "uint8" => ValaSyntaxKind::Uint8Kw,
            "int16" => ValaSyntaxKind::Int16Kw,
            "uint16" => ValaSyntaxKind::Uint16Kw,
            "int32" => ValaSyntaxKind::Int32Kw,
            "uint32" => ValaSyntaxKind::Uint32Kw,
            "int64" => ValaSyntaxKind::Int64Kw,
            "uint64" => ValaSyntaxKind::Uint64Kw,
            "float" => ValaSyntaxKind::FloatKw,
            "double" => ValaSyntaxKind::DoubleKw,
            "string" => ValaSyntaxKind::StringKw,
            _ => ValaSyntaxKind::Identifier,
        };

        state.add_token(kind, start, state.get_position());
        true
    }

    fn lex_operators<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();

        // 优先匹配较长的操作符
        let patterns: &[(&str, ValaSyntaxKind)] = &[
            ("<<", ValaSyntaxKind::LeftShift),
            (">>", ValaSyntaxKind::RightShift),
            ("==", ValaSyntaxKind::EqEq),
            ("!=", ValaSyntaxKind::NotEq),
            ("<=", ValaSyntaxKind::LessEq),
            (">=", ValaSyntaxKind::GreaterEq),
            ("&&", ValaSyntaxKind::AndAnd),
            ("||", ValaSyntaxKind::OrOr),
            ("++", ValaSyntaxKind::PlusPlus),
            ("--", ValaSyntaxKind::MinusMinus),
            ("+=", ValaSyntaxKind::PlusEq),
            ("-=", ValaSyntaxKind::MinusEq),
            ("*=", ValaSyntaxKind::StarEq),
            ("/=", ValaSyntaxKind::SlashEq),
            ("%=", ValaSyntaxKind::PercentEq),
            ("->", ValaSyntaxKind::Arrow),
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
                '+' => Some(ValaSyntaxKind::Plus),
                '-' => Some(ValaSyntaxKind::Minus),
                '*' => Some(ValaSyntaxKind::Star),
                '/' => Some(ValaSyntaxKind::Slash),
                '%' => Some(ValaSyntaxKind::Percent),
                '^' => Some(ValaSyntaxKind::Caret),
                '!' => Some(ValaSyntaxKind::Bang),
                '&' => Some(ValaSyntaxKind::Ampersand),
                '|' => Some(ValaSyntaxKind::Pipe),
                '=' => Some(ValaSyntaxKind::Eq),
                '>' => Some(ValaSyntaxKind::GreaterThan),
                '<' => Some(ValaSyntaxKind::LessThan),
                '.' => Some(ValaSyntaxKind::Dot),
                ':' => Some(ValaSyntaxKind::Colon),
                '?' => Some(ValaSyntaxKind::Question),
                '~' => Some(ValaSyntaxKind::Tilde),
                '\\' => Some(ValaSyntaxKind::Backslash),
                '@' => Some(ValaSyntaxKind::At),
                '#' => Some(ValaSyntaxKind::Hash),
                '$' => Some(ValaSyntaxKind::Dollar),
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
                '(' => Some(ValaSyntaxKind::LeftParen),
                ')' => Some(ValaSyntaxKind::RightParen),
                '{' => Some(ValaSyntaxKind::LeftBrace),
                '}' => Some(ValaSyntaxKind::RightBrace),
                '[' => Some(ValaSyntaxKind::LeftBracket),
                ']' => Some(ValaSyntaxKind::RightBracket),
                ',' => Some(ValaSyntaxKind::Comma),
                ';' => Some(ValaSyntaxKind::Semicolon),
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
