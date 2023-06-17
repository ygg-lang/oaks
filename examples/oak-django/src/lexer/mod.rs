use crate::{kind::DjangoSyntaxKind, language::DjangoLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, DjangoLanguage>;

pub struct DjangoLexer<'config> {
    config: &'config DjangoLanguage,
}

impl<'config> DjangoLexer<'config> {
    pub fn new(config: &'config DjangoLanguage) -> Self {
        Self { config }
    }

    /// 跳过空白字符
    fn skip_whitespace(&self, state: &mut State) -> bool {
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
            state.add_token(DjangoSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(DjangoSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(DjangoSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理标识符和关键

    fn lex_identifier_or_keyword(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let end_pos = state.get_position();
                let text = source.get_text_in((start_pos..end_pos).into()).unwrap_or("");

                let token_kind = match text {
                    "if" => DjangoSyntaxKind::If,
                    "elif" => DjangoSyntaxKind::Elif,
                    "else" => DjangoSyntaxKind::Else,
                    "endif" => DjangoSyntaxKind::Endif,
                    "for" => DjangoSyntaxKind::For,
                    "empty" => DjangoSyntaxKind::Empty,
                    "endfor" => DjangoSyntaxKind::Endfor,
                    "block" => DjangoSyntaxKind::Block,
                    "endblock" => DjangoSyntaxKind::Endblock,
                    "extends" => DjangoSyntaxKind::Extends,
                    "include" => DjangoSyntaxKind::Include,
                    "load" => DjangoSyntaxKind::Load,
                    "with" => DjangoSyntaxKind::With,
                    "endwith" => DjangoSyntaxKind::Endwith,
                    "autoescape" => DjangoSyntaxKind::Autoescape,
                    "endautoescape" => DjangoSyntaxKind::Endautoescape,
                    "csrf_token" => DjangoSyntaxKind::Csrf,
                    "url" => DjangoSyntaxKind::Url,
                    "static" => DjangoSyntaxKind::Static,
                    "now" => DjangoSyntaxKind::Now,
                    "cycle" => DjangoSyntaxKind::Cycle,
                    "filter" => DjangoSyntaxKind::Filter,
                    "endfilter" => DjangoSyntaxKind::Endfilter,
                    "spaceless" => DjangoSyntaxKind::Spaceless,
                    "endspaceless" => DjangoSyntaxKind::Endspaceless,
                    "verbatim" => DjangoSyntaxKind::Verbatim,
                    "endverbatim" => DjangoSyntaxKind::Endverbatim,
                    "and" => DjangoSyntaxKind::And,
                    "or" => DjangoSyntaxKind::Or,
                    "not" => DjangoSyntaxKind::Not,
                    "in" => DjangoSyntaxKind::In,
                    _ => DjangoSyntaxKind::Identifier,
                };

                state.add_token(token_kind, start_pos, state.get_position());
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

    /// 处理数字
    fn lex_number(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(ch.len_utf8());

                // 处理整数部分
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 处理小数部分
                if let Some('.') = state.peek() {
                    let dot_pos = state.get_position();
                    state.advance(1);

                    if let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            while let Some(ch) = state.peek() {
                                if ch.is_ascii_digit() {
                                    state.advance(ch.len_utf8());
                                }
                                else {
                                    break;
                                }
                            }
                        }
                        else {
                            // 回退点号
                            state.set_position(dot_pos);
                        }
                    }
                    else {
                        // 回退点号
                        state.set_position(dot_pos);
                    }
                }

                state.add_token(DjangoSyntaxKind::Number, start_pos, state.get_position());
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

    /// 处理字符

    fn lex_string(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '"' || quote == '\'' {
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch == quote {
                        state.advance(1);
                        state.add_token(DjangoSyntaxKind::String, start_pos, state.get_position());
                        return true;
                    }
                    else if ch == '\\' {
                        state.advance(1);
                        if state.peek().is_some() {
                            state.advance(1);
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                // 未闭合的字符

                state.add_token(DjangoSyntaxKind::Error, start_pos, state.get_position());
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

    /// 处理 Django 标签
    fn lex_django_tags(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('{') = state.peek() {
            state.advance(1);

            if let Some(next_ch) = state.peek() {
                match next_ch {
                    '{' => {
                        // 变量标签 {{
                        state.advance(1);
                        state.add_token(DjangoSyntaxKind::VariableStart, start_pos, state.get_position());
                        true
                    }
                    '%' => {
                        // 模板标签 {%
                        state.advance(1);
                        state.add_token(DjangoSyntaxKind::TagStart, start_pos, state.get_position());
                        true
                    }
                    '#' => {
                        // 注释标签 {#
                        state.advance(1);
                        state.add_token(DjangoSyntaxKind::CommentStart, start_pos, state.get_position());
                        true
                    }
                    _ => {
                        // 回退
                        state.set_position(start_pos);
                        false
                    }
                }
            }
            else {
                // 回退
                state.set_position(start_pos);
                false
            }
        }
        else if let Some('%') = state.peek() {
            state.advance(1);
            if let Some('}') = state.peek() {
                state.advance(1);
                state.add_token(DjangoSyntaxKind::TagEnd, start_pos, state.get_position());
                true
            }
            else {
                state.set_position(start_pos);
                false
            }
        }
        else if let Some('#') = state.peek() {
            state.advance(1);
            if let Some('}') = state.peek() {
                state.advance(1);
                state.add_token(DjangoSyntaxKind::CommentEnd, start_pos, state.get_position());
                true
            }
            else {
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// 处理操作
    fn lex_operator(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        DjangoSyntaxKind::EqualEqual
                    }
                    else {
                        DjangoSyntaxKind::Equal
                    }
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        DjangoSyntaxKind::NotEqual
                    }
                    else {
                        return false;
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        DjangoSyntaxKind::LessEqual
                    }
                    else {
                        DjangoSyntaxKind::Less
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        DjangoSyntaxKind::GreaterEqual
                    }
                    else {
                        DjangoSyntaxKind::Greater
                    }
                }
                '+' => {
                    state.advance(1);
                    DjangoSyntaxKind::Plus
                }
                '-' => {
                    state.advance(1);
                    DjangoSyntaxKind::Minus
                }
                '*' => {
                    state.advance(1);
                    DjangoSyntaxKind::Star
                }
                '/' => {
                    state.advance(1);
                    DjangoSyntaxKind::Slash
                }
                '%' => {
                    state.advance(1);
                    DjangoSyntaxKind::Percent
                }
                '|' => {
                    state.advance(1);
                    DjangoSyntaxKind::Pipe
                }
                _ => return false,
            };

            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理分隔

    fn lex_delimiter(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => DjangoSyntaxKind::LeftParen,
                ')' => DjangoSyntaxKind::RightParen,
                '[' => DjangoSyntaxKind::LeftBracket,
                ']' => DjangoSyntaxKind::RightBracket,
                ',' => DjangoSyntaxKind::Comma,
                '.' => DjangoSyntaxKind::Dot,
                ':' => DjangoSyntaxKind::Colon,
                ';' => DjangoSyntaxKind::Semicolon,
                _ => return false,
            };

            state.advance(ch.len_utf8());
            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理 HTML 文本
    fn lex_html_text(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            // 遇到 Django 标签开始符号时停止
            if ch == '{' || ch == '%' || ch == '#' {
                break;
            }
            // 遇到特殊字符时停
            if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
                break;
            }
            state.advance(ch.len_utf8());
        }

        if state.get_position() > start_pos {
            state.add_token(DjangoSyntaxKind::HtmlText, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<DjangoLanguage> for DjangoLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<DjangoSyntaxKind> {
        let mut state = LexerState::new(source);

        while state.not_at_end() {
            // 尝试各种词法规则
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_django_tags(&mut state) {
                continue;
            }

            if self.lex_identifier_or_keyword(&mut state, source) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_string(&mut state) {
                continue;
            }

            if self.lex_operator(&mut state) {
                continue;
            }

            if self.lex_delimiter(&mut state) {
                continue;
            }

            if self.lex_html_text(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(DjangoSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(DjangoSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
