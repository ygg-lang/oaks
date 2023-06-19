use crate::{
    kind::{ValaSyntaxKind, ValaToken},
    language::ValaLanguage,
};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{CommentLine, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S: Source> = LexerState<S, ValaLanguage>;

static VALA_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static VALA_COMMENT: LazyLock<CommentLine> = LazyLock::new(|| CommentLine { line_markers: &["//"] });
static VALA_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });
static VALA_CHAR: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['\''], escape: Some('\\') });

#[derive(Clone)]
pub struct ValaLexer<'config> {
    config: &'config ValaLanguage,
}

impl<'config> Lexer<ValaLanguage> for ValaLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<ValaLanguage>,
    ) -> LexOutput<ValaLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> ValaLexer<'config> {
    pub fn new(config: &'config ValaLanguage) -> Self {
        Self { config }
    }

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
        state.add_token(ValaSyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        match VALA_WHITESPACE.scan(state.rest(), state.get_position(), ValaSyntaxKind::Whitespace) {
            Some(token) => {
                state.advance_with(token);
                true
            }
            None => false,
        }
    }

    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // 行注释: // ... until newline
        if rest.starts_with("//") {
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(ValaSyntaxKind::LineComment, start, state.get_position());
            return true;
        }

        // 块注释: /* ... */ with nesting support
        if rest.starts_with("/*") {
            state.advance(2);
            let mut depth = 1usize;
            while let Some(ch) = state.peek() {
                if ch == '/' && state.peek_next_n(1) == Some('*') {
                    state.advance(2);
                    depth += 1;
                    continue;
                }
                if ch == '*' && state.peek_next_n(1) == Some('/') {
                    state.advance(2);
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                    continue;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(ValaSyntaxKind::BlockComment, start, state.get_position());
            return true;
        }

        false
    }

    fn lex_string_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

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
            state.add_token(ValaSyntaxKind::StringLiteral, start, state.get_position());
            return true;
        }

        false
    }

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
            state.add_token(ValaSyntaxKind::CharLiteral, start, state.get_position());
            return true;
        }

        state.set_position(start);
        false
    }

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
        let kind = match text {
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

    fn lex_operators<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

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
            if rest.starts_with(pat) {
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

    fn lex_single_char_tokens<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.current() {
            let kind = match ch {
                '(' => ValaSyntaxKind::LeftParen,
                ')' => ValaSyntaxKind::RightParen,
                '{' => ValaSyntaxKind::LeftBrace,
                '}' => ValaSyntaxKind::RightBrace,
                '[' => ValaSyntaxKind::LeftBracket,
                ']' => ValaSyntaxKind::RightBracket,
                ',' => ValaSyntaxKind::Comma,
                ';' => ValaSyntaxKind::Semicolon,
                _ => {
                    // 如果不是已知的单字符token，添加为错误并前进
                    state.advance(ch.len_utf8());
                    state.add_token(ValaSyntaxKind::Error, start, state.get_position());
                    return true;
                }
            };

            state.advance(ch.len_utf8());
            state.add_token(kind, start, state.get_position());
            true
        }
        else {
            false
        }
    }
}
