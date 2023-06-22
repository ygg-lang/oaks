use crate::{kind::NixSyntaxKind, language::NixLanguage};
use oak_core::{
    Lexer, LexerCache, LexerState,
    lexer::LexOutput,
    source::{Source, TextEdit},
};

type State<'a, S> = LexerState<'a, S, NixLanguage>;

#[derive(Clone, Debug)]
pub struct NixLexer<'config> {
    _config: &'config NixLanguage,
}

impl<'config> NixLexer<'config> {
    pub fn new(config: &'config NixLanguage) -> Self {
        Self { _config: config }
    }

    /// 跳过空白字符
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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
            state.add_token(NixSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(NixSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(NixSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('#') = state.peek() {
            state.advance(1);

            // 读取到行
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(NixSyntaxKind::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符串字面量
    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch == '"' {
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

            state.add_token(NixSyntaxKind::String, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理数字字面量
    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() || ch == '.' {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }
                state.add_token(NixSyntaxKind::Number, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理标识符和关键字
    fn lex_identifier<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());
                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' || ch == '-' || ch == '\'' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = state.get_text_in((start_pos..state.get_position()).into());
                let kind = match &*text {
                    "let" => NixSyntaxKind::Let,
                    "in" => NixSyntaxKind::In,
                    "if" => NixSyntaxKind::If,
                    "then" => NixSyntaxKind::Then,
                    "else" => NixSyntaxKind::Else,
                    "with" => NixSyntaxKind::With,
                    "inherit" => NixSyntaxKind::Inherit,
                    "rec" => NixSyntaxKind::Rec,
                    "import" => NixSyntaxKind::Import,
                    "true" | "false" => NixSyntaxKind::Boolean,
                    "null" => NixSyntaxKind::Null,
                    _ => NixSyntaxKind::Identifier,
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

    /// 处理操作
    fn lex_operator<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '+' => {
                    state.advance(1);
                    if let Some('+') = state.peek() {
                        state.advance(1);
                        NixSyntaxKind::Concatenation
                    }
                    else {
                        NixSyntaxKind::Plus
                    }
                }
                '-' => {
                    state.advance(1);
                    if let Some('>') = state.peek() {
                        state.advance(1);
                        NixSyntaxKind::Implication
                    }
                    else {
                        NixSyntaxKind::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    NixSyntaxKind::Star
                }
                '/' => {
                    state.advance(1);
                    if let Some('/') = state.peek() {
                        state.advance(1);
                        NixSyntaxKind::Update
                    }
                    else {
                        NixSyntaxKind::Slash
                    }
                }
                '%' => {
                    state.advance(1);
                    NixSyntaxKind::Percent
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        NixSyntaxKind::Equal
                    }
                    else {
                        NixSyntaxKind::Assign
                    }
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        NixSyntaxKind::NotEqual
                    }
                    else {
                        return false;
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        NixSyntaxKind::LessEqual
                    }
                    else {
                        NixSyntaxKind::Less
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        NixSyntaxKind::GreaterEqual
                    }
                    else {
                        NixSyntaxKind::Greater
                    }
                }
                '&' => {
                    state.advance(1);
                    if let Some('&') = state.peek() {
                        state.advance(1);
                        NixSyntaxKind::LogicalAnd
                    }
                    else {
                        return false;
                    }
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.peek() {
                        state.advance(1);
                        NixSyntaxKind::LogicalOr
                    }
                    else {
                        return false;
                    }
                }
                '?' => {
                    state.advance(1);
                    NixSyntaxKind::Question
                }
                '(' => {
                    state.advance(1);
                    NixSyntaxKind::LeftParen
                }
                ')' => {
                    state.advance(1);
                    NixSyntaxKind::RightParen
                }
                '{' => {
                    state.advance(1);
                    NixSyntaxKind::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    NixSyntaxKind::RightBrace
                }
                '[' => {
                    state.advance(1);
                    NixSyntaxKind::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    NixSyntaxKind::RightBracket
                }
                ';' => {
                    state.advance(1);
                    NixSyntaxKind::Semicolon
                }
                ':' => {
                    state.advance(1);
                    NixSyntaxKind::Colon
                }
                ',' => {
                    state.advance(1);
                    NixSyntaxKind::Comma
                }
                '.' => {
                    state.advance(1);
                    NixSyntaxKind::Dot
                }
                '@' => {
                    state.advance(1);
                    NixSyntaxKind::At
                }
                '$' => {
                    state.advance(1);
                    NixSyntaxKind::Dollar
                }
                '#' => {
                    state.advance(1);
                    NixSyntaxKind::Hash
                }
                _ => return false,
            };

            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<NixLanguage> for NixLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, text: &'a S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<NixLanguage>) -> LexOutput<NixLanguage> {
        let mut state = State::new(text);

        while state.not_at_end() {
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
            if self.lex_number(&mut state) {
                continue;
            }
            if self.lex_identifier(&mut state) {
                continue;
            }
            if self.lex_operator(&mut state) {
                continue;
            }

            // 如果没有匹配到任何模式，添加错误 kind
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(NixSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        let eof_pos = state.get_position();
        state.add_token(NixSyntaxKind::Eof, eof_pos, eof_pos);
        state.finish_with_cache(Ok(()), cache)
    }
}
