use crate::{kind::DelphiSyntaxKind, language::DelphiLanguage};
use oak_core::{Lexer, LexerCache, LexerState, OakError, TextEdit, lexer::LexOutput, source::Source};

type State<'a, S> = LexerState<'a, S, DelphiLanguage>;

/// Lexer implementation for Delphi programming language
#[derive(Clone, Debug)]
pub struct DelphiLexer<'config> {
    _config: &'config DelphiLanguage,
}

impl<'config> DelphiLexer<'config> {
    pub fn new(config: &'config DelphiLanguage) -> Self {
        Self { _config: config }
    }

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

            if self.lex_operators(state) {
                continue;
            }

            if self.lex_single_char_tokens(state) {
                continue;
            }

            // 如果没有匹配任何规则，添加错误 token 并前进
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(DelphiSyntaxKind::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }

    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();
        let mut consumed = false;
        while let Some(ch) = state.peek() {
            if ch.is_whitespace() {
                consumed = true;
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }
        if consumed {
            state.add_token(DelphiSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        // Line comment: // ... until newline
        if state.consume_if_starts_with("//") {
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(DelphiSyntaxKind::LineComment, start, state.get_position());
            return true;
        }

        // Block comment: { ... }
        if state.consume_if_starts_with("{") {
            let mut depth = 1usize;
            while let Some(ch) = state.peek() {
                if ch == '{' {
                    depth += 1;
                }
                else if ch == '}' {
                    depth -= 1;
                    if depth == 0 {
                        state.advance(1);
                        break;
                    }
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(DelphiSyntaxKind::BlockComment, start, state.get_position());
            return true;
        }

        // Block comment: (* ... *)
        if state.consume_if_starts_with("(*") {
            while let Some(ch) = state.peek() {
                if state.consume_if_starts_with("*)") {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(DelphiSyntaxKind::BlockComment, start, state.get_position());
            return true;
        }

        false
    }

    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if let Some('\'') = state.peek() {
            state.advance(1);
            while let Some(ch) = state.peek() {
                if ch == '\'' {
                    state.advance(1);
                    if state.peek() == Some('\'') {
                        // Double single quote is an escaped single quote
                        state.advance(1);
                        continue;
                    }
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(DelphiSyntaxKind::String, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_number_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let first = match state.peek() {
            Some(c) => c,
            None => return false,
        };

        if !first.is_ascii_digit() && first != '$' {
            return false;
        }

        let mut is_float = false;

        // Hexadecimal number
        if first == '$' {
            state.advance(1);
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
            // Decimal number
            state.advance(1);
            while let Some(c) = state.peek() {
                if c.is_ascii_digit() {
                    state.advance(1);
                }
                else {
                    break;
                }
            }

            // Fractional part
            if state.peek() == Some('.') {
                let next = state.peek_next_n(1);
                if next.map(|c| c.is_ascii_digit()).unwrap_or(false) {
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

            // Exponent part
            if let Some(c) = state.peek() {
                if c == 'e' || c == 'E' {
                    let next = state.peek_next_n(1);
                    if next == Some('+') || next == Some('-') || next.map(|d| d.is_ascii_digit()).unwrap_or(false) {
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
        }

        state.add_token(if is_float { DelphiSyntaxKind::Float } else { DelphiSyntaxKind::Number }, start, state.get_position());
        true
    }

    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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
        let text = state.get_text_in((start..end).into());
        let kind = match text.to_lowercase().as_str() {
            "and" => DelphiSyntaxKind::And_,
            "array" => DelphiSyntaxKind::Array,
            "as" => DelphiSyntaxKind::As_,
            "begin" => DelphiSyntaxKind::Begin,
            "case" => DelphiSyntaxKind::Case,
            "class" => DelphiSyntaxKind::Class,
            "const" => DelphiSyntaxKind::Const,
            "div" => DelphiSyntaxKind::Div,
            "do" => DelphiSyntaxKind::Do,
            "downto" => DelphiSyntaxKind::Downto,
            "else" => DelphiSyntaxKind::Else,
            "end" => DelphiSyntaxKind::End,
            "except" => DelphiSyntaxKind::Except,
            "false" => DelphiSyntaxKind::False_,
            "finally" => DelphiSyntaxKind::Finally,
            "for" => DelphiSyntaxKind::For,
            "function" => DelphiSyntaxKind::Function,
            "if" => DelphiSyntaxKind::If,
            "implementation" => DelphiSyntaxKind::Implementation,
            "in" => DelphiSyntaxKind::In_,
            "interface" => DelphiSyntaxKind::Interface,
            "is" => DelphiSyntaxKind::Is_,
            "mod" => DelphiSyntaxKind::Mod,
            "nil" => DelphiSyntaxKind::Nil,
            "not" => DelphiSyntaxKind::Not_,
            "object" => DelphiSyntaxKind::Object,
            "of" => DelphiSyntaxKind::Of,
            "or" => DelphiSyntaxKind::Or_,
            "procedure" => DelphiSyntaxKind::Procedure,
            "program" => DelphiSyntaxKind::Program,
            "record" => DelphiSyntaxKind::Record,
            "repeat" => DelphiSyntaxKind::Repeat,
            "set" => DelphiSyntaxKind::Set,
            "then" => DelphiSyntaxKind::Then,
            "to" => DelphiSyntaxKind::To,
            "true" => DelphiSyntaxKind::True_,
            "try" => DelphiSyntaxKind::Try,
            "type" => DelphiSyntaxKind::Type,
            "unit" => DelphiSyntaxKind::Unit,
            "until" => DelphiSyntaxKind::Until,
            "uses" => DelphiSyntaxKind::Uses,
            "var" => DelphiSyntaxKind::Var,
            "while" => DelphiSyntaxKind::While,
            "with" => DelphiSyntaxKind::With,
            _ => DelphiSyntaxKind::Identifier,
        };

        state.add_token(kind, start, state.get_position());
        true
    }

    fn lex_operators<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        // Multi-character operators (longest first)
        let patterns: &[(&str, DelphiSyntaxKind)] = &[(":=", DelphiSyntaxKind::Assign), ("<=", DelphiSyntaxKind::LessEqual), (">=", DelphiSyntaxKind::GreaterEqual), ("<>", DelphiSyntaxKind::NotEqual), ("..", DelphiSyntaxKind::DotDot)];

        for (pat, kind) in patterns {
            if state.consume_if_starts_with(pat) {
                state.add_token(*kind, start, state.get_position());
                return true;
            }
        }

        // Single-character operators
        if let Some(ch) = state.peek() {
            let kind = match ch {
                '+' => Some(DelphiSyntaxKind::Plus),
                '-' => Some(DelphiSyntaxKind::Minus),
                '*' => Some(DelphiSyntaxKind::Star),
                '/' => Some(DelphiSyntaxKind::Slash),
                '=' => Some(DelphiSyntaxKind::Equal),
                '<' => Some(DelphiSyntaxKind::Less),
                '>' => Some(DelphiSyntaxKind::Greater),
                '.' => Some(DelphiSyntaxKind::Dot),
                ':' => Some(DelphiSyntaxKind::Colon),
                '^' => Some(DelphiSyntaxKind::Caret),
                '@' => Some(DelphiSyntaxKind::At),
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

    fn lex_single_char_tokens<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '(' => DelphiSyntaxKind::LeftParen,
                ')' => DelphiSyntaxKind::RightParen,
                '[' => DelphiSyntaxKind::LeftBracket,
                ']' => DelphiSyntaxKind::RightBracket,
                ',' => DelphiSyntaxKind::Comma,
                ';' => DelphiSyntaxKind::Semicolon,
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

impl<'config> Lexer<DelphiLanguage> for DelphiLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<DelphiLanguage>) -> LexOutput<DelphiLanguage> {
        let mut state = State::new_with_cache(source, 0, cache);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}
