use crate::{kind::DjangoSyntaxKind, language::DjangoLanguage};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{CommentLine, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S: Source> = LexerState<S, DjangoLanguage>;

static DJANGO_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static DJANGO_COMMENT: LazyLock<CommentLine> = LazyLock::new(|| CommentLine { line_markers: &["{#"] });
static DJANGO_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"', '\''], escape: Some('\\') });

#[derive(Clone)]
pub struct DjangoLexer<'config> {
    config: &'config DjangoLanguage,
}

impl<'config> DjangoLexer<'config> {
    pub fn new(config: &'config DjangoLanguage) -> Self {
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

            if self.lex_string(state) {
                continue;
            }

            if self.lex_number(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_django_tags(state) {
                continue;
            }

            if self.lex_operator(state) {
                continue;
            }

            if self.lex_delimiter(state) {
                continue;
            }

            if self.lex_html_text(state) {
                continue;
            }

            state.safe_check(safe_point);
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(DjangoSyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        match DJANGO_WHITESPACE.scan(state.rest(), state.get_position(), DjangoSyntaxKind::Whitespace) {
            Some(token) => {
                let start = state.get_position();
                state.advance(token.length());
                state.add_token(DjangoSyntaxKind::Whitespace, start, state.get_position());
                true
            }
            None => false,
        }
    }

    /// 跳过注释
    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        if state.rest().starts_with("{#") {
            let start = state.get_position();
            state.advance(2); // 跳过 "{#"

            // 查找注释结束标记 "#}"
            while state.not_at_end() {
                if state.rest().starts_with("#}") {
                    state.advance(2); // 跳过 "#}"
                    break;
                }
                state.advance(1);
            }

            state.add_token(DjangoSyntaxKind::Comment, start, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符串字面量
    fn lex_string_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        match DJANGO_STRING.scan(state.rest(), state.get_position(), DjangoSyntaxKind::String) {
            Some(token) => {
                let start = state.get_position();
                state.advance(token.length());
                state.add_token(DjangoSyntaxKind::String, start, state.get_position());
                true
            }
            None => false,
        }
    }

    /// 处理换行
    fn lex_newline<S: Source>(&self, state: &mut State<S>) -> bool {
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

    /// 处理标识符和关键字
    fn lex_identifier_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
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
                let text = state.get_text_in((start_pos..end_pos).into());

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
    fn lex_number<S: Source>(&self, state: &mut State<S>) -> bool {
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

    fn lex_string<S: Source>(&self, state: &mut State<S>) -> bool {
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
    fn lex_django_tags<S: Source>(&self, state: &mut State<S>) -> bool {
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
    fn lex_operator<S: Source>(&self, state: &mut State<S>) -> bool {
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

    fn lex_delimiter<S: Source>(&self, state: &mut State<S>) -> bool {
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
    fn lex_html_text<S: Source>(&self, state: &mut State<S>) -> bool {
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
    fn lex_incremental(
        &self,
        source: impl Source,
        _changed: usize,
        _cache: IncrementalCache<DjangoLanguage>,
    ) -> LexOutput<DjangoLanguage> {
        let mut state = LexerState::new_with_cache(source, _changed, _cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}
