use crate::{kind::TypeScriptSyntaxKind, language::TypeScriptLanguage};
use oak_core::{Lexer, LexerCache, LexerState, OakError, TextEdit, lexer::LexOutput, source::Source};

#[derive(Clone, Debug)]
pub struct TypeScriptLexer<'config> {
    _config: &'config TypeScriptLanguage,
}

type State<'a, S> = LexerState<'a, S, TypeScriptLanguage>;

impl<'config> TypeScriptLexer<'config> {
    pub fn new(config: &'config TypeScriptLanguage) -> Self {
        Self { _config: config }
    }
}

impl<'config> Lexer<TypeScriptLanguage> for TypeScriptLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, text: &S, edits: &[TextEdit], cache: &'a mut impl LexerCache<TypeScriptLanguage>) -> LexOutput<TypeScriptLanguage> {
        let relex_from = edits.iter().map(|e| e.span.start).min().unwrap_or(0);
        let mut state: State<'_, S> = LexerState::new_with_cache(text, relex_from, cache);

        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> TypeScriptLexer<'config> {
    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_newline(state) {
                continue;
            }

            if self.skip_comment(state) {
                continue;
            }

            if self.lex_string_literal(state) {
                continue;
            }

            if self.lex_template_literal(state) {
                continue;
            }

            if self.lex_numeric_literal(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_operator_or_punctuation(state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(TypeScriptSyntaxKind::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }

    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let mut found = false;

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' {
                state.advance(ch.len_utf8());
                found = true;
            }
            else {
                break;
            }
        }

        if found {
            state.add_token(TypeScriptSyntaxKind::Whitespace, start, state.get_position());
        }

        found
    }

    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            if ch == '\n' {
                state.advance(1);
                state.add_token(TypeScriptSyntaxKind::Newline, start, state.get_position());
                return true;
            }
            else if ch == '\r' {
                state.advance(1);
                if state.peek() == Some('\n') {
                    state.advance(1);
                }
                state.add_token(TypeScriptSyntaxKind::Newline, start, state.get_position());
                return true;
            }
        }

        false
    }

    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // 行注释: // ...
        if rest.starts_with("//") {
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(TypeScriptSyntaxKind::LineComment, start, state.get_position());
            return true;
        }

        // 块注释: /* ... */
        if rest.starts_with("/*") {
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch == '*' && state.peek_next_n(1) == Some('/') {
                    state.advance(2);
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(TypeScriptSyntaxKind::BlockComment, start, state.get_position());
            return true;
        }

        false
    }

    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '"' || quote == '\'' {
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch == quote {
                        state.advance(1);
                        break;
                    }
                    else if ch == '\\' {
                        state.advance(1);
                        if let Some(_) = state.peek() {
                            state.advance(1);
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(TypeScriptSyntaxKind::StringLiteral, start, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_template_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if state.peek() == Some('`') {
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch == '`' {
                    state.advance(1);
                    break;
                }
                else if ch == '\\' {
                    state.advance(1);
                    if let Some(_) = state.peek() {
                        state.advance(1);
                    }
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            state.add_token(TypeScriptSyntaxKind::TemplateString, start, state.get_position());
            return true;
        }

        false
    }

    fn lex_numeric_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);

                // 处理十六进制
                if ch == '0' && (state.peek() == Some('x') || state.peek() == Some('X')) {
                    state.advance(1);
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_hexdigit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }
                else {
                    // 处理十进制
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(1);
                        }
                        else if ch == '.' && state.peek_next_n(1).map_or(false, |c| c.is_ascii_digit()) {
                            state.advance(1);
                            while let Some(ch) = state.peek() {
                                if ch.is_ascii_digit() {
                                    state.advance(1);
                                }
                                else {
                                    break;
                                }
                            }
                            break;
                        }
                        else {
                            break;
                        }
                    }
                }

                // 检查 BigInt 后缀
                if state.peek() == Some('n') {
                    state.advance(1);
                    state.add_token(TypeScriptSyntaxKind::BigIntLiteral, start, state.get_position());
                }
                else {
                    state.add_token(TypeScriptSyntaxKind::NumericLiteral, start, state.get_position());
                }

                return true;
            }
        }

        false
    }

    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' || ch == '$' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' || ch == '$' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 获取标识符文本并检查是否为关键字
                let end = state.get_position();
                let text = state.get_text_in(oak_core::Range { start, end });
                let kind = self.keyword_or_identifier(&text);

                state.add_token(kind, start, state.get_position());
                return true;
            }
        }

        false
    }

    fn keyword_or_identifier(&self, text: &str) -> TypeScriptSyntaxKind {
        TypeScriptSyntaxKind::from_keyword(text).unwrap_or(TypeScriptSyntaxKind::IdentifierName)
    }

    fn lex_operator_or_punctuation<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        let ops = [
            ("===", TypeScriptSyntaxKind::EqualEqualEqual),
            ("!==", TypeScriptSyntaxKind::NotEqualEqual),
            (">>>", TypeScriptSyntaxKind::UnsignedRightShift),
            ("...", TypeScriptSyntaxKind::DotDotDot),
            ("**=", TypeScriptSyntaxKind::StarStarEqual),
            ("<<=", TypeScriptSyntaxKind::LeftShiftEqual),
            (">>=", TypeScriptSyntaxKind::RightShiftEqual),
            ("&&=", TypeScriptSyntaxKind::AmpersandAmpersandEqual),
            ("||=", TypeScriptSyntaxKind::PipePipeEqual),
            ("??=", TypeScriptSyntaxKind::QuestionQuestionEqual),
            ("**", TypeScriptSyntaxKind::StarStar),
            ("<=", TypeScriptSyntaxKind::LessEqual),
            (">=", TypeScriptSyntaxKind::GreaterEqual),
            ("==", TypeScriptSyntaxKind::EqualEqual),
            ("!=", TypeScriptSyntaxKind::NotEqual),
            ("&&", TypeScriptSyntaxKind::AmpersandAmpersand),
            ("||", TypeScriptSyntaxKind::PipePipe),
            ("<<", TypeScriptSyntaxKind::LeftShift),
            (">>", TypeScriptSyntaxKind::RightShift),
            ("++", TypeScriptSyntaxKind::PlusPlus),
            ("--", TypeScriptSyntaxKind::MinusMinus),
            ("=>", TypeScriptSyntaxKind::Arrow),
            ("?.", TypeScriptSyntaxKind::QuestionDot),
            ("??", TypeScriptSyntaxKind::QuestionQuestion),
            ("+=", TypeScriptSyntaxKind::PlusEqual),
            ("-=", TypeScriptSyntaxKind::MinusEqual),
            ("*=", TypeScriptSyntaxKind::StarEqual),
            ("/=", TypeScriptSyntaxKind::SlashEqual),
            ("%=", TypeScriptSyntaxKind::PercentEqual),
            ("&=", TypeScriptSyntaxKind::AmpersandEqual),
            ("|=", TypeScriptSyntaxKind::PipeEqual),
            ("^=", TypeScriptSyntaxKind::CaretEqual),
        ];

        for (op, kind) in ops {
            if rest.starts_with(op) {
                state.advance(op.len());
                state.add_token(kind, start, state.get_position());
                return true;
            }
        }

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '+' => TypeScriptSyntaxKind::Plus,
                '-' => TypeScriptSyntaxKind::Minus,
                '*' => TypeScriptSyntaxKind::Star,
                '/' => TypeScriptSyntaxKind::Slash,
                '%' => TypeScriptSyntaxKind::Percent,
                '<' => TypeScriptSyntaxKind::Less,
                '>' => TypeScriptSyntaxKind::Greater,
                '!' => TypeScriptSyntaxKind::Exclamation,
                '&' => TypeScriptSyntaxKind::Ampersand,
                '|' => TypeScriptSyntaxKind::Pipe,
                '^' => TypeScriptSyntaxKind::Caret,
                '~' => TypeScriptSyntaxKind::Tilde,
                '=' => TypeScriptSyntaxKind::Equal,
                '?' => TypeScriptSyntaxKind::Question,
                '(' => TypeScriptSyntaxKind::LeftParen,
                ')' => TypeScriptSyntaxKind::RightParen,
                '{' => TypeScriptSyntaxKind::LeftBrace,
                '}' => TypeScriptSyntaxKind::RightBrace,
                '[' => TypeScriptSyntaxKind::LeftBracket,
                ']' => TypeScriptSyntaxKind::RightBracket,
                ';' => TypeScriptSyntaxKind::Semicolon,
                ',' => TypeScriptSyntaxKind::Comma,
                '.' => TypeScriptSyntaxKind::Dot,
                ':' => TypeScriptSyntaxKind::Colon,
                _ => return false,
            };

            state.advance(1);
            state.add_token(kind, start, state.get_position());
            return true;
        }

        false
    }
}
