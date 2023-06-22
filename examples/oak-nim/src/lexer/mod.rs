use crate::{kind::NimSyntaxKind, language::NimLanguage};
use oak_core::{Lexer, LexerCache, LexerState, lexer::LexOutput, source::Source};
use std::borrow::Cow;

type State<'s, S> = LexerState<'s, S, NimLanguage>;

#[derive(Clone, Debug)]
pub struct NimLexer<'config> {
    _config: &'config NimLanguage,
}

impl<'config> NimLexer<'config> {
    pub fn new(config: &'config NimLanguage) -> Self {
        Self { _config: config }
    }

    /// 跳过空白字符
    fn skip_whitespace<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
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
            state.add_token(NimSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(NimSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(NimSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('#') = state.peek() {
            state.advance(1);

            // 检查是否是文档注释 ##
            if let Some('#') = state.peek() {
                state.advance(1);
            }

            // 读取到行
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            let kind = NimSyntaxKind::CommentToken;

            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符串字面量
    fn lex_string<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1);
                    break;
                }
                if ch == '\\' {
                    state.advance(1);
                    if let Some(c) = state.peek() {
                        state.advance(c.len_utf8());
                    }
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            state.add_token(NimSyntaxKind::StringLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符字面量
    fn lex_char<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\'') = state.peek() {
            state.advance(1);

            if let Some('\\') = state.peek() {
                state.advance(1);
                if let Some(c) = state.peek() {
                    state.advance(c.len_utf8());
                }
            }
            else if let Some(c) = state.peek() {
                if c != '\'' {
                    state.advance(c.len_utf8());
                }
            }

            if let Some('\'') = state.peek() {
                state.advance(1);
            }

            state.add_token(NimSyntaxKind::CharLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理数字
    fn lex_number<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 简单的浮点数处理
                let mut is_float = false;
                if let Some('.') = state.peek() {
                    state.advance(1);
                    is_float = true;
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() || ch == '_' {
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }
                }

                let kind = if is_float { NimSyntaxKind::FloatLiteral } else { NimSyntaxKind::IntLiteral };
                state.add_token(kind, start_pos, state.get_position());
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

    /// 处理标识符和关键字
    fn lex_identifier<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
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

                let text = state.get_text_in((start_pos..state.get_position()).into());
                let kind = match text {
                    Cow::Borrowed("and") => NimSyntaxKind::AndKeyword,
                    Cow::Borrowed("or") => NimSyntaxKind::OrKeyword,
                    Cow::Borrowed("not") => NimSyntaxKind::NotKeyword,
                    Cow::Borrowed("if") => NimSyntaxKind::IfKeyword,
                    Cow::Borrowed("else") => NimSyntaxKind::ElseKeyword,
                    Cow::Borrowed("elif") => NimSyntaxKind::ElifKeyword,
                    Cow::Borrowed("while") => NimSyntaxKind::WhileKeyword,
                    Cow::Borrowed("for") => NimSyntaxKind::ForKeyword,
                    Cow::Borrowed("proc") => NimSyntaxKind::ProcKeyword,
                    Cow::Borrowed("func") => NimSyntaxKind::FuncKeyword,
                    Cow::Borrowed("var") => NimSyntaxKind::VarKeyword,
                    Cow::Borrowed("let") => NimSyntaxKind::LetKeyword,
                    Cow::Borrowed("const") => NimSyntaxKind::ConstKeyword,
                    Cow::Borrowed("type") => NimSyntaxKind::TypeKeyword,
                    Cow::Borrowed("import") => NimSyntaxKind::ImportKeyword,
                    Cow::Borrowed("from") => NimSyntaxKind::FromKeyword,
                    Cow::Borrowed("include") => NimSyntaxKind::IncludeKeyword,
                    Cow::Borrowed("return") => NimSyntaxKind::ReturnKeyword,
                    Cow::Borrowed("yield") => NimSyntaxKind::YieldKeyword,
                    Cow::Borrowed("break") => NimSyntaxKind::BreakKeyword,
                    Cow::Borrowed("continue") => NimSyntaxKind::ContinueKeyword,
                    Cow::Borrowed("try") => NimSyntaxKind::TryKeyword,
                    Cow::Borrowed("except") => NimSyntaxKind::ExceptKeyword,
                    Cow::Borrowed("finally") => NimSyntaxKind::FinallyKeyword,
                    Cow::Borrowed("raise") => NimSyntaxKind::RaiseKeyword,
                    Cow::Borrowed("case") => NimSyntaxKind::CaseKeyword,
                    Cow::Borrowed("of") => NimSyntaxKind::OfKeyword,
                    Cow::Borrowed("when") => NimSyntaxKind::WhenKeyword,
                    Cow::Borrowed("is") => NimSyntaxKind::IsKeyword,
                    Cow::Borrowed("in") => NimSyntaxKind::InKeyword,
                    Cow::Borrowed("nil") => NimSyntaxKind::NilKeyword,
                    _ => NimSyntaxKind::Identifier,
                };

                state.add_token(kind, start_pos, state.get_position());
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

    /// 处理操作符
    fn lex_operator<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        if let Some(ch) = state.peek() {
            let start_pos = state.get_position();

            match ch {
                '+' => {
                    state.advance(1);
                    state.add_token(NimSyntaxKind::Plus, start_pos, state.get_position());
                    true
                }
                '-' => {
                    state.advance(1);
                    state.add_token(NimSyntaxKind::Minus, start_pos, state.get_position());
                    true
                }
                '*' => {
                    state.advance(1);
                    state.add_token(NimSyntaxKind::Star, start_pos, state.get_position());
                    true
                }
                '/' => {
                    state.advance(1);
                    state.add_token(NimSyntaxKind::Slash, start_pos, state.get_position());
                    true
                }
                '=' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        state.add_token(NimSyntaxKind::EqualEqual, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(NimSyntaxKind::Equal, start_pos, state.get_position());
                    }
                    true
                }
                '!' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        state.add_token(NimSyntaxKind::NotEqual, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(NimSyntaxKind::Error, start_pos, state.get_position());
                    }
                    true
                }
                '<' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        state.add_token(NimSyntaxKind::LessEqual, start_pos, state.get_position());
                    }
                    else if state.peek() == Some('<') {
                        state.advance(1);
                        state.add_token(NimSyntaxKind::LeftShift, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(NimSyntaxKind::Less, start_pos, state.get_position());
                    }
                    true
                }
                '>' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        state.add_token(NimSyntaxKind::GreaterEqual, start_pos, state.get_position());
                    }
                    else if state.peek() == Some('>') {
                        state.advance(1);
                        state.add_token(NimSyntaxKind::RightShift, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(NimSyntaxKind::Greater, start_pos, state.get_position());
                    }
                    true
                }
                '(' => {
                    state.advance(1);
                    state.add_token(NimSyntaxKind::LeftParen, start_pos, state.get_position());
                    true
                }
                ')' => {
                    state.advance(1);
                    state.add_token(NimSyntaxKind::RightParen, start_pos, state.get_position());
                    true
                }
                '[' => {
                    state.advance(1);
                    state.add_token(NimSyntaxKind::LeftBracket, start_pos, state.get_position());
                    true
                }
                ']' => {
                    state.advance(1);
                    state.add_token(NimSyntaxKind::RightBracket, start_pos, state.get_position());
                    true
                }
                '{' => {
                    state.advance(1);
                    state.add_token(NimSyntaxKind::LeftBrace, start_pos, state.get_position());
                    true
                }
                '}' => {
                    state.advance(1);
                    state.add_token(NimSyntaxKind::RightBrace, start_pos, state.get_position());
                    true
                }
                ',' => {
                    state.advance(1);
                    state.add_token(NimSyntaxKind::Comma, start_pos, state.get_position());
                    true
                }
                ';' => {
                    state.advance(1);
                    state.add_token(NimSyntaxKind::Semicolon, start_pos, state.get_position());
                    true
                }
                ':' => {
                    state.advance(1);
                    state.add_token(NimSyntaxKind::Colon, start_pos, state.get_position());
                    true
                }
                '.' => {
                    state.advance(1);
                    state.add_token(NimSyntaxKind::Dot, start_pos, state.get_position());
                    true
                }
                _ => false,
            }
        }
        else {
            false
        }
    }

    pub fn run<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> Result<(), oak_core::OakError> {
        while state.not_at_end() {
            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_newline(state) {
                continue;
            }

            if self.lex_comment(state) {
                continue;
            }

            if self.lex_string(state) {
                continue;
            }

            if self.lex_char(state) {
                continue;
            }

            if self.lex_number(state) {
                continue;
            }

            if self.lex_identifier(state) {
                continue;
            }

            if self.lex_operator(state) {
                continue;
            }

            // 如果没有匹配到任何模式，添加错误 kind
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(NimSyntaxKind::Error, start_pos, state.get_position());
            }
        }
        Ok(())
    }
}

impl<'config> Lexer<NimLanguage> for NimLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl LexerCache<NimLanguage>) -> LexOutput<NimLanguage> {
        let mut state = State::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}
