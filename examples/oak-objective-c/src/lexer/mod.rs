use crate::{kind::ObjectiveCSyntaxKind, language::ObjectiveCLanguage};
use oak_core::{Lexer, LexerCache, LexerState, OakError, TextEdit, lexer::LexOutput, source::Source};

type State<'a, S> = LexerState<'a, S, ObjectiveCLanguage>;

#[derive(Clone)]
pub struct ObjectiveCLexer<'config> {
    #[allow(dead_code)]
    config: &'config ObjectiveCLanguage,
}

impl<'config> Lexer<ObjectiveCLanguage> for ObjectiveCLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<ObjectiveCLanguage>) -> LexOutput<ObjectiveCLanguage> {
        let mut state = State::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> ObjectiveCLexer<'config> {
    pub fn new(config: &'config ObjectiveCLanguage) -> Self {
        Self { config }
    }

    /// 主词法分析循环
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

            // 如果没有匹配任何模式，添加错误 token 并前进
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(ObjectiveCSyntaxKind::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();
        while let Some(ch) = state.peek() {
            if ch.is_whitespace() {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }
        if state.get_position() > start {
            state.add_token(ObjectiveCSyntaxKind::Whitespace, start, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn skip_comment<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
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
            state.add_token(ObjectiveCSyntaxKind::CommentToken, start, state.get_position());
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
            state.add_token(ObjectiveCSyntaxKind::CommentToken, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_string_literal<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();

        // Objective-C string literal: @"..."
        if state.peek() == Some('@') && state.peek_next_n(1) == Some('"') {
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
            state.add_token(ObjectiveCSyntaxKind::String, start, state.get_position());
            return true;
        }

        // normal string: "..."
        if state.peek() == Some('"') {
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
            state.add_token(ObjectiveCSyntaxKind::String, start, state.get_position());
            return true;
        }

        false
    }

    fn lex_char_literal<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();
        if state.peek() != Some('\'') {
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
            state.add_token(ObjectiveCSyntaxKind::Character, start, state.get_position());
            return true;
        }

        state.set_position(start);
        false
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
        state.add_token(if is_float { ObjectiveCSyntaxKind::FloatLiteral } else { ObjectiveCSyntaxKind::IntegerLiteral }, start, end);
        true
    }

    fn lex_identifier_or_keyword<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();
        let ch = match state.peek() {
            Some(c) => c,
            None => return false,
        };

        if !(ch.is_ascii_alphabetic() || ch == '_' || ch == '@' || ch == '#') {
            return false;
        }

        state.advance(1);
        while let Some(c) = state.peek() {
            if c.is_ascii_alphanumeric() || c == '_' {
                state.advance(1);
            }
            else {
                break;
            }
        }

        let end = state.get_position();
        let text = state.get_text_in(oak_core::Range { start, end });
        let kind = match text.as_ref() {
            // Objective-C keywords
            "@interface" => ObjectiveCSyntaxKind::InterfaceKeyword,
            "@implementation" => ObjectiveCSyntaxKind::ImplementationKeyword,
            "@end" => ObjectiveCSyntaxKind::EndKeyword,
            "@property" => ObjectiveCSyntaxKind::PropertyKeyword,
            "@synthesize" => ObjectiveCSyntaxKind::SynthesizeKeyword,
            "@dynamic" => ObjectiveCSyntaxKind::DynamicKeyword,
            "@protocol" => ObjectiveCSyntaxKind::ProtocolKeyword,
            "@import" => ObjectiveCSyntaxKind::ImportKeyword,
            "#import" => ObjectiveCSyntaxKind::ImportKeyword,
            "#include" => ObjectiveCSyntaxKind::IncludeKeyword,

            // C keywords
            "if" => ObjectiveCSyntaxKind::IfKeyword,
            "else" => ObjectiveCSyntaxKind::ElseKeyword,
            "for" => ObjectiveCSyntaxKind::ForKeyword,
            "while" => ObjectiveCSyntaxKind::WhileKeyword,
            "do" => ObjectiveCSyntaxKind::DoKeyword,
            "switch" => ObjectiveCSyntaxKind::SwitchKeyword,
            "case" => ObjectiveCSyntaxKind::CaseKeyword,
            "default" => ObjectiveCSyntaxKind::DefaultKeyword,
            "break" => ObjectiveCSyntaxKind::BreakKeyword,
            "continue" => ObjectiveCSyntaxKind::ContinueKeyword,
            "return" => ObjectiveCSyntaxKind::ReturnKeyword,
            "void" => ObjectiveCSyntaxKind::VoidKeyword,
            "int" => ObjectiveCSyntaxKind::IntKeyword,
            "float" => ObjectiveCSyntaxKind::FloatKeyword,
            "double" => ObjectiveCSyntaxKind::DoubleKeyword,
            "char" => ObjectiveCSyntaxKind::CharKeyword,
            "BOOL" => ObjectiveCSyntaxKind::BoolKeyword,
            "id" => ObjectiveCSyntaxKind::IdKeyword,
            "self" => ObjectiveCSyntaxKind::SelfKeyword,
            "super" => ObjectiveCSyntaxKind::SuperKeyword,
            "nil" => ObjectiveCSyntaxKind::NilKeyword,
            "YES" => ObjectiveCSyntaxKind::YesKeyword,
            "NO" => ObjectiveCSyntaxKind::NoKeyword,

            _ => ObjectiveCSyntaxKind::Identifier,
        };

        state.add_token(kind, start, state.get_position());
        true
    }

    fn lex_operators<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // prefer longest matches first
        let patterns: &[(&str, ObjectiveCSyntaxKind)] =
            &[("==", ObjectiveCSyntaxKind::EqualEqual), ("!=", ObjectiveCSyntaxKind::NotEqual), (">=", ObjectiveCSyntaxKind::GreaterEqual), ("<=", ObjectiveCSyntaxKind::LessEqual), ("&&", ObjectiveCSyntaxKind::And), ("||", ObjectiveCSyntaxKind::Or)];

        for (pat, kind) in patterns {
            if rest.starts_with(pat) {
                state.advance(pat.len());
                state.add_token(*kind, start, state.get_position());
                return true;
            }
        }

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '+' => Some(ObjectiveCSyntaxKind::Plus),
                '-' => Some(ObjectiveCSyntaxKind::Minus),
                '*' => Some(ObjectiveCSyntaxKind::Star),
                '/' => Some(ObjectiveCSyntaxKind::Slash),
                '%' => Some(ObjectiveCSyntaxKind::Percent),
                '=' => Some(ObjectiveCSyntaxKind::Equal),
                '>' => Some(ObjectiveCSyntaxKind::Greater),
                '<' => Some(ObjectiveCSyntaxKind::Less),
                '!' => Some(ObjectiveCSyntaxKind::Not),
                '?' => Some(ObjectiveCSyntaxKind::Question),
                ':' => Some(ObjectiveCSyntaxKind::Colon),
                '.' => Some(ObjectiveCSyntaxKind::Dot),
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
        if let Some(ch) = state.peek() {
            let kind = match ch {
                '(' => ObjectiveCSyntaxKind::LeftParen,
                ')' => ObjectiveCSyntaxKind::RightParen,
                '[' => ObjectiveCSyntaxKind::LeftBracket,
                ']' => ObjectiveCSyntaxKind::RightBracket,
                '{' => ObjectiveCSyntaxKind::LeftBrace,
                '}' => ObjectiveCSyntaxKind::RightBrace,
                ',' => ObjectiveCSyntaxKind::Comma,
                ';' => ObjectiveCSyntaxKind::Semicolon,
                '@' => ObjectiveCSyntaxKind::At,
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
