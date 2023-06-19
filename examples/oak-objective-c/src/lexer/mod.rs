use crate::{kind::ObjectiveCLanguageSyntaxKind, language::ObjectiveCLanguage};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{CommentLine, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S> = LexerState<S, ObjectiveCLanguage>;

static OC_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static OC_COMMENT: LazyLock<CommentLine> = LazyLock::new(|| CommentLine { line_markers: &["//"] });
static OC_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });
static OC_CHAR: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['\''], escape: Some('\\') });

#[derive(Clone)]
pub struct ObjectiveCLexer<'config> {
    config: &'config ObjectiveCLanguage,
}

impl<'config> Lexer<ObjectiveCLanguage> for ObjectiveCLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<ObjectiveCLanguage>,
    ) -> LexOutput<ObjectiveCLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> ObjectiveCLexer<'config> {
    pub fn new(config: &'config ObjectiveCLanguage) -> Self {
        Self { config }
    }

    /// 主词法分析循环
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
        state.add_token(ObjectiveCLanguageSyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        match OC_WHITESPACE.scan(state.rest(), state.get_position(), ObjectiveCLanguageSyntaxKind::Whitespace) {
            Some(token) => {
                state.advance_with(token);
                return true;
            }
            None => {}
        }
        false
    }

    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();
        // line comment: // ... until newline
        if rest.starts_with("//") {
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(ObjectiveCLanguageSyntaxKind::CommentToken, start, state.get_position());
            return true;
        }
        // block comment: /* ... */ with nesting support
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
            state.add_token(ObjectiveCLanguageSyntaxKind::CommentToken, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_string_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        // Objective-C string literal: @"..."
        if state.current() == Some('@') && state.peek_next_n(1) == Some('"') {
            state.advance(2); // consume @"
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
            state.add_token(ObjectiveCLanguageSyntaxKind::String, start, state.get_position());
            return true;
        }

        // normal string: "..."
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
            state.add_token(ObjectiveCLanguageSyntaxKind::String, start, state.get_position());
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
            state.add_token(ObjectiveCLanguageSyntaxKind::Character, start, state.get_position());
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

        // consume digits
        state.advance(1);
        while let Some(c) = state.peek() {
            if c.is_ascii_digit() {
                state.advance(1);
            }
            else {
                break;
            }
        }

        // fractional part
        if state.peek() == Some('.') {
            let n1 = state.peek_next_n(1);
            if n1.map(|c| c.is_ascii_digit()).unwrap_or(false) {
                is_float = true;
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
        }

        // exponent
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

        // suffix letters (e.g., f, l, u)
        while let Some(c) = state.peek() {
            if c.is_ascii_alphabetic() {
                state.advance(1);
            }
            else {
                break;
            }
        }

        let end = state.get_position();
        state.add_token(
            if is_float { ObjectiveCLanguageSyntaxKind::FloatLiteral } else { ObjectiveCLanguageSyntaxKind::IntegerLiteral },
            start,
            end,
        );
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
            // Objective-C keywords
            "@interface" => ObjectiveCLanguageSyntaxKind::InterfaceKeyword,
            "@implementation" => ObjectiveCLanguageSyntaxKind::ImplementationKeyword,
            "@end" => ObjectiveCLanguageSyntaxKind::EndKeyword,
            "@property" => ObjectiveCLanguageSyntaxKind::PropertyKeyword,
            "@synthesize" => ObjectiveCLanguageSyntaxKind::SynthesizeKeyword,
            "@dynamic" => ObjectiveCLanguageSyntaxKind::DynamicKeyword,
            "@protocol" => ObjectiveCLanguageSyntaxKind::ProtocolKeyword,
            "@import" => ObjectiveCLanguageSyntaxKind::ImportKeyword,
            "#import" => ObjectiveCLanguageSyntaxKind::ImportKeyword,
            "#include" => ObjectiveCLanguageSyntaxKind::IncludeKeyword,

            // C keywords
            "if" => ObjectiveCLanguageSyntaxKind::IfKeyword,
            "else" => ObjectiveCLanguageSyntaxKind::ElseKeyword,
            "for" => ObjectiveCLanguageSyntaxKind::ForKeyword,
            "while" => ObjectiveCLanguageSyntaxKind::WhileKeyword,
            "do" => ObjectiveCLanguageSyntaxKind::DoKeyword,
            "switch" => ObjectiveCLanguageSyntaxKind::SwitchKeyword,
            "case" => ObjectiveCLanguageSyntaxKind::CaseKeyword,
            "default" => ObjectiveCLanguageSyntaxKind::DefaultKeyword,
            "break" => ObjectiveCLanguageSyntaxKind::BreakKeyword,
            "continue" => ObjectiveCLanguageSyntaxKind::ContinueKeyword,
            "return" => ObjectiveCLanguageSyntaxKind::ReturnKeyword,
            "void" => ObjectiveCLanguageSyntaxKind::VoidKeyword,
            "int" => ObjectiveCLanguageSyntaxKind::IntKeyword,
            "float" => ObjectiveCLanguageSyntaxKind::FloatKeyword,
            "double" => ObjectiveCLanguageSyntaxKind::DoubleKeyword,
            "char" => ObjectiveCLanguageSyntaxKind::CharKeyword,
            "BOOL" => ObjectiveCLanguageSyntaxKind::BoolKeyword,
            "id" => ObjectiveCLanguageSyntaxKind::IdKeyword,
            "self" => ObjectiveCLanguageSyntaxKind::SelfKeyword,
            "super" => ObjectiveCLanguageSyntaxKind::SuperKeyword,
            "nil" => ObjectiveCLanguageSyntaxKind::NilKeyword,
            "YES" => ObjectiveCLanguageSyntaxKind::YesKeyword,
            "NO" => ObjectiveCLanguageSyntaxKind::NoKeyword,

            _ => ObjectiveCLanguageSyntaxKind::Identifier,
        };

        state.add_token(kind, start, state.get_position());
        true
    }

    fn lex_operators<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // prefer longest matches first
        let patterns: &[(&str, ObjectiveCLanguageSyntaxKind)] = &[
            ("==", ObjectiveCLanguageSyntaxKind::EqualEqual),
            ("!=", ObjectiveCLanguageSyntaxKind::NotEqual),
            (">=", ObjectiveCLanguageSyntaxKind::GreaterEqual),
            ("<=", ObjectiveCLanguageSyntaxKind::LessEqual),
            ("&&", ObjectiveCLanguageSyntaxKind::And),
            ("||", ObjectiveCLanguageSyntaxKind::Or),
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
                '+' => Some(ObjectiveCLanguageSyntaxKind::Plus),
                '-' => Some(ObjectiveCLanguageSyntaxKind::Minus),
                '*' => Some(ObjectiveCLanguageSyntaxKind::Star),
                '/' => Some(ObjectiveCLanguageSyntaxKind::Slash),
                '%' => Some(ObjectiveCLanguageSyntaxKind::Percent),
                '=' => Some(ObjectiveCLanguageSyntaxKind::Equal),
                '>' => Some(ObjectiveCLanguageSyntaxKind::Greater),
                '<' => Some(ObjectiveCLanguageSyntaxKind::Less),
                '!' => Some(ObjectiveCLanguageSyntaxKind::Not),
                '?' => Some(ObjectiveCLanguageSyntaxKind::Question),
                ':' => Some(ObjectiveCLanguageSyntaxKind::Colon),
                '.' => Some(ObjectiveCLanguageSyntaxKind::Dot),
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
                '(' => ObjectiveCLanguageSyntaxKind::LeftParen,
                ')' => ObjectiveCLanguageSyntaxKind::RightParen,
                '[' => ObjectiveCLanguageSyntaxKind::LeftBracket,
                ']' => ObjectiveCLanguageSyntaxKind::RightBracket,
                '{' => ObjectiveCLanguageSyntaxKind::LeftBrace,
                '}' => ObjectiveCLanguageSyntaxKind::RightBrace,
                ',' => ObjectiveCLanguageSyntaxKind::Comma,
                ';' => ObjectiveCLanguageSyntaxKind::Semicolon,
                '@' => ObjectiveCLanguageSyntaxKind::At,
                _ => return false,
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
