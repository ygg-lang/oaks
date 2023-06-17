use crate::{kind::CssSyntaxKind, language::CssLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, CssLanguage>;

pub struct CssLexer<'config> {
    config: &'config CssLanguage,
}

impl<'config> CssLexer<'config> {
    pub fn new(config: &'config CssLanguage) -> Self {
        Self { config }
    }

    /// 跳过空白字符
    fn skip_whitespace(&self, state: &mut State<'_>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        if state.get_position() > start_pos {
            state.add_token(CssSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline(&self, state: &mut State<'_>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(CssSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(CssSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State<'_>) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            if let Some('*') = state.peek_next_n(1) {
                state.advance(2); // Skip /*

                while let Some(ch) = state.peek() {
                    if ch == '*' && state.peek_next_n(1) == Some('/') {
                        state.advance(2); // Skip */
                        break;
                    }
                    state.advance(ch.len_utf8());
                }

                state.add_token(CssSyntaxKind::Comment, start_pos, state.get_position());
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    /// 处理字符串字面量
    fn lex_string(&self, state: &mut State<'_>) -> bool {
        let start_pos = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '"' || quote == '\'' {
                state.advance(1); // Skip opening quote

                while let Some(ch) = state.peek() {
                    if ch == quote {
                        state.advance(1); // Skip closing quote
                        break;
                    }
                    else if ch == '\\' {
                        state.advance(1); // Skip escape character
                        if state.peek().is_some() {
                            state.advance(state.peek().unwrap().len_utf8()); // Skip escaped character
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(CssSyntaxKind::StringLiteral, start_pos, state.get_position());
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    /// 处理数字字面
    fn lex_number(&self, state: &mut State<'_>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() || (ch == '.' && state.peek_next_n(1).map_or(false, |c| c.is_ascii_digit())) {
                // Integer part
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                // Decimal part
                if let Some('.') = state.peek() {
                    state.advance(1);
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }

                // Exponent part
                if let Some(ch) = state.peek() {
                    if ch == 'e' || ch == 'E' {
                        state.advance(1);
                        if let Some(sign) = state.peek() {
                            if sign == '+' || sign == '-' {
                                state.advance(1);
                            }
                        }
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                    }
                }

                // Check for units
                while let Some(ch) = state.peek() {
                    if ch.is_alphabetic() || ch == '%' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                state.add_token(CssSyntaxKind::NumberLiteral, start_pos, state.get_position());
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    /// 处理颜色字面
    fn lex_color(&self, state: &mut State<'_>) -> bool {
        let start_pos = state.get_position();

        if let Some('#') = state.peek() {
            state.advance(1); // Skip #

            let mut hex_count = 0;
            while let Some(ch) = state.peek() {
                if ch.is_ascii_hexdigit() && hex_count < 8 {
                    state.advance(1);
                    hex_count += 1;
                }
                else {
                    break;
                }
            }

            let token_kind = if matches!(hex_count, 3 | 4 | 6 | 8) { CssSyntaxKind::ColorLiteral } else { CssSyntaxKind::Hash };

            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理 URL 字面
    fn lex_url(&self, state: &mut State<'_>) -> bool {
        let start_pos = state.get_position();

        if let Some('u') = state.peek() {
            if state.peek_next_n(1) == Some('r') && state.peek_next_n(2) == Some('l') && state.peek_next_n(3) == Some('(') {
                state.advance(4); // Skip "url("

                // Skip whitespace
                while let Some(ch) = state.peek() {
                    if ch.is_whitespace() {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // Handle quoted or unquoted URL
                if let Some(quote) = state.peek() {
                    if quote == '"' || quote == '\'' {
                        self.lex_string(state);
                    }
                    else {
                        while let Some(ch) = state.peek() {
                            if ch == ')' || ch.is_whitespace() {
                                break;
                            }
                            state.advance(ch.len_utf8());
                        }
                    }
                }

                // Skip whitespace
                while let Some(ch) = state.peek() {
                    if ch.is_whitespace() {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // Skip closing )
                if let Some(')') = state.peek() {
                    state.advance(1);
                }

                state.add_token(CssSyntaxKind::UrlLiteral, start_pos, state.get_position());
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    /// 处理标识
    fn lex_identifier(&self, state: &mut State<'_>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' || ch == '-' {
                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' || ch == '-' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                state.add_token(CssSyntaxKind::Identifier, start_pos, state.get_position());
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    /// 处理 at-rule
    fn lex_at_rule(&self, state: &mut State<'_>, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some('@') = state.peek() {
            state.advance(1); // Skip @

            let rule_start = state.get_position();
            while let Some(ch) = state.peek() {
                if ch.is_alphabetic() || ch == '-' {
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }

            let rule_name = source.get_text_in((rule_start..state.get_position()).into()).unwrap_or("");
            let token_kind = match rule_name {
                "import" => CssSyntaxKind::AtImport,
                "media" => CssSyntaxKind::AtMedia,
                "keyframes" => CssSyntaxKind::AtKeyframes,
                "font-face" => CssSyntaxKind::AtFontFace,
                "charset" => CssSyntaxKind::AtCharset,
                "namespace" => CssSyntaxKind::AtNamespace,
                "supports" => CssSyntaxKind::AtSupports,
                "page" => CssSyntaxKind::AtPage,
                "document" => CssSyntaxKind::AtDocument,
                _ => CssSyntaxKind::AtRule,
            };

            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理操作
    fn lex_operator(&self, state: &mut State<'_>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                ':' => CssSyntaxKind::Colon,
                ';' => CssSyntaxKind::Semicolon,
                ',' => CssSyntaxKind::Comma,
                '.' => CssSyntaxKind::Dot,
                '#' => CssSyntaxKind::Hash,
                '+' => CssSyntaxKind::Plus,
                '-' => CssSyntaxKind::Minus,
                '*' => CssSyntaxKind::Star,
                '/' => CssSyntaxKind::Slash,
                '=' => CssSyntaxKind::Equals,
                '~' => CssSyntaxKind::Tilde,
                '|' => CssSyntaxKind::Pipe,
                '^' => CssSyntaxKind::Caret,
                '$' => CssSyntaxKind::Dollar,
                '>' => CssSyntaxKind::GreaterThan,
                _ => return false,
            };

            state.advance(1);
            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理分隔
    fn lex_delimiter(&self, state: &mut State<'_>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => CssSyntaxKind::LeftParen,
                ')' => CssSyntaxKind::RightParen,
                '{' => CssSyntaxKind::LeftBrace,
                '}' => CssSyntaxKind::RightBrace,
                '[' => CssSyntaxKind::LeftBracket,
                ']' => CssSyntaxKind::RightBracket,
                _ => return false,
            };

            state.advance(1);
            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<CssLanguage> for CssLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<CssSyntaxKind> {
        let mut state = LexerState::new(source);

        while state.not_at_end() {
            // 尝试各种词法规则
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_comment(&mut state) {
                continue;
            }

            if self.lex_string(&mut state) {
                continue;
            }

            if self.lex_url(&mut state) {
                continue;
            }

            if self.lex_color(&mut state) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_at_rule(&mut state, source) {
                continue;
            }

            if self.lex_identifier(&mut state) {
                continue;
            }

            if self.lex_delimiter(&mut state) {
                continue;
            }

            if self.lex_operator(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(CssSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(CssSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
