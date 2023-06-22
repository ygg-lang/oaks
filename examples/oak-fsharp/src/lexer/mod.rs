use crate::{kind::FSharpSyntaxKind, language::FSharpLanguage};
use oak_core::{
    Lexer, LexerCache, LexerState, OakError,
    lexer::{LexOutput, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<'a, S> = LexerState<'a, S, FSharpLanguage>;

static FS_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });

/// F# 词法分析器
#[derive(Clone)]
pub struct FSharpLexer<'config> {
    _config: &'config FSharpLanguage,
}

impl<'config> Lexer<FSharpLanguage> for FSharpLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl LexerCache<FSharpLanguage>) -> LexOutput<FSharpLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> FSharpLexer<'config> {
    pub fn new(config: &'config FSharpLanguage) -> Self {
        Self { _config: config }
    }

    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            // 跳过空白字符
            if self.skip_whitespace(state) {
                continue;
            }

            // 处理注释
            if self.skip_comment(state) {
                continue;
            }

            // 处理字符串字面量
            if self.lex_string_literal(state) {
                continue;
            }

            // 处理字符字面量
            if self.lex_char_literal(state) {
                continue;
            }

            // 处理数字字面量
            if self.lex_number(state) {
                continue;
            }

            // 处理标识符和关键字
            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            // 处理操作符和标点符号
            if self.lex_operator_or_punctuation(state) {
                continue;
            }

            // 如果没有匹配任何模式，跳过当前字符
            let start = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(FSharpSyntaxKind::Error, start, state.get_position());
            }
        }

        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        FS_WHITESPACE.scan(state, FSharpSyntaxKind::Whitespace)
    }

    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // 行注释: // ... 直到换行
        if rest.starts_with("//") {
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(FSharpSyntaxKind::LineComment, start, state.get_position());
            return true;
        }

        // 块注释: (* ... *) 支持嵌套
        if rest.starts_with("(*") {
            state.advance(2);
            let mut depth = 1usize;
            while let Some(ch) = state.peek() {
                if ch == '(' && state.peek_next_n(1) == Some('*') {
                    state.advance(2);
                    depth += 1;
                    continue;
                }
                if ch == '*' && state.peek_next_n(1) == Some(')') {
                    state.advance(2);
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                    continue;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(FSharpSyntaxKind::BlockComment, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        // 原始字符串: @"..."
        if state.peek() == Some('@') && state.peek_next_n(1) == Some('"') {
            state.advance(2); // 跳过 @"
            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1);
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(FSharpSyntaxKind::StringLiteral, start, state.get_position());
            return true;
        }

        // 普通字符串: "..."
        if state.peek() == Some('"') {
            state.advance(1); // 跳过 "
            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1);
                    break;
                }
                if ch == '\\' {
                    state.advance(1); // 跳过转义字符
                    if let Some(escaped) = state.peek() {
                        state.advance(escaped.len_utf8());
                    }
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }
            state.add_token(FSharpSyntaxKind::StringLiteral, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_char_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if state.peek() == Some('\'') {
            state.advance(1); // 跳过 '
            if let Some(ch) = state.peek() {
                if ch == '\\' {
                    state.advance(1); // 跳过转义字符
                    if let Some(escaped) = state.peek() {
                        state.advance(escaped.len_utf8());
                    }
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }
            if state.peek() == Some('\'') {
                state.advance(1); // 跳过结束的 '
            }
            state.add_token(FSharpSyntaxKind::CharLiteral, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if !state.current().map_or(false, |c| c.is_ascii_digit()) {
            return false;
        }

        let start = state.get_position();

        // 处理整数部分
        while state.current().map_or(false, |c| c.is_ascii_digit()) {
            state.advance(1);
        }

        // 处理小数点
        if state.current() == Some('.') && state.peek().map_or(false, |c| c.is_ascii_digit()) {
            state.advance(1); // 跳过 '.'
            while state.current().map_or(false, |c| c.is_ascii_digit()) {
                state.advance(1);
            }
            state.add_token(FSharpSyntaxKind::FloatLiteral, start, state.get_position());
        }
        else {
            // 处理科学计数法
            if matches!(state.current(), Some('e') | Some('E')) {
                state.advance(1);
                if matches!(state.current(), Some('+') | Some('-')) {
                    state.advance(1);
                }
                while state.current().map_or(false, |c| c.is_ascii_digit()) {
                    state.advance(1);
                }
                state.add_token(FSharpSyntaxKind::FloatLiteral, start, state.get_position());
            }
            else {
                // 处理数字后缀
                if state.current().map_or(false, |c| c.is_ascii_alphabetic()) {
                    while state.current().map_or(false, |c| c.is_ascii_alphanumeric()) {
                        state.advance(1);
                    }
                }
                state.add_token(FSharpSyntaxKind::IntegerLiteral, start, state.get_position());
            }
        }

        true
    }

    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if !state.current().map_or(false, |c| c.is_ascii_alphabetic() || c == '_') {
            return false;
        }

        let start = state.get_position();

        // 读取标识符
        while state.current().map_or(false, |c| c.is_ascii_alphanumeric() || c == '_') {
            state.advance(1);
        }

        let text = state.get_text_from(start);
        let kind = self.classify_identifier(&text);
        state.add_token(kind, start, state.get_position());
        true
    }

    fn classify_identifier(&self, text: &str) -> FSharpSyntaxKind {
        match text {
            // F# 关键字
            "abstract" => FSharpSyntaxKind::Abstract,
            "and" => FSharpSyntaxKind::And,
            "as" => FSharpSyntaxKind::As,
            "assert" => FSharpSyntaxKind::Assert,
            "base" => FSharpSyntaxKind::Base,
            "begin" => FSharpSyntaxKind::Begin,
            "class" => FSharpSyntaxKind::Class,
            "default" => FSharpSyntaxKind::Default,
            "do" => FSharpSyntaxKind::Do,
            "done" => FSharpSyntaxKind::Done,
            "downcast" => FSharpSyntaxKind::Downcast,
            "downto" => FSharpSyntaxKind::Downto,
            "elif" => FSharpSyntaxKind::Elif,
            "else" => FSharpSyntaxKind::Else,
            "end" => FSharpSyntaxKind::End,
            "exception" => FSharpSyntaxKind::Exception,
            "extern" => FSharpSyntaxKind::Extern,
            "false" => FSharpSyntaxKind::False,
            "finally" => FSharpSyntaxKind::Finally,
            "for" => FSharpSyntaxKind::For,
            "fun" => FSharpSyntaxKind::Fun,
            "function" => FSharpSyntaxKind::Function,
            "global" => FSharpSyntaxKind::Global,
            "if" => FSharpSyntaxKind::If,
            "in" => FSharpSyntaxKind::In,
            "inherit" => FSharpSyntaxKind::Inherit,
            "inline" => FSharpSyntaxKind::Inline,
            "interface" => FSharpSyntaxKind::Interface,
            "internal" => FSharpSyntaxKind::Internal,
            "lazy" => FSharpSyntaxKind::Lazy,
            "let" => FSharpSyntaxKind::Let,
            "match" => FSharpSyntaxKind::Match,
            "member" => FSharpSyntaxKind::Member,
            "module" => FSharpSyntaxKind::Module,
            "mutable" => FSharpSyntaxKind::Mutable,
            "namespace" => FSharpSyntaxKind::Namespace,
            "new" => FSharpSyntaxKind::New,
            "not" => FSharpSyntaxKind::Not,
            "null" => FSharpSyntaxKind::Null,
            "of" => FSharpSyntaxKind::Of,
            "open" => FSharpSyntaxKind::Open,
            "or" => FSharpSyntaxKind::Or,
            "override" => FSharpSyntaxKind::Override,
            "private" => FSharpSyntaxKind::Private,
            "public" => FSharpSyntaxKind::Public,
            "rec" => FSharpSyntaxKind::Rec,
            "return" => FSharpSyntaxKind::Return,
            "sig" => FSharpSyntaxKind::Sig,
            "static" => FSharpSyntaxKind::Static,
            "struct" => FSharpSyntaxKind::Struct,
            "then" => FSharpSyntaxKind::Then,
            "to" => FSharpSyntaxKind::To,
            "true" => FSharpSyntaxKind::True,
            "try" => FSharpSyntaxKind::Try,
            "type" => FSharpSyntaxKind::Type,
            "upcast" => FSharpSyntaxKind::Upcast,
            "use" => FSharpSyntaxKind::Use,
            "val" => FSharpSyntaxKind::Val,
            "void" => FSharpSyntaxKind::Void,
            "when" => FSharpSyntaxKind::When,
            "while" => FSharpSyntaxKind::While,
            "with" => FSharpSyntaxKind::With,
            "yield" => FSharpSyntaxKind::Yield,
            "async" => FSharpSyntaxKind::Async,
            "seq" => FSharpSyntaxKind::Seq,
            "raise" => FSharpSyntaxKind::Raise,
            "failwith" => FSharpSyntaxKind::Failwith,
            _ => FSharpSyntaxKind::Identifier,
        }
    }

    fn lex_operator_or_punctuation<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let current = state.current();
        if current.is_none() {
            return false;
        }

        let start = state.get_position();
        let c = current.unwrap();
        let next = state.peek();

        // 双字符操作符
        match (c, next) {
            ('-', Some('>')) => {
                state.advance(2);
                state.add_token(FSharpSyntaxKind::Arrow, start, state.get_position());
                return true;
            }
            (':', Some(':')) => {
                state.advance(2);
                state.add_token(FSharpSyntaxKind::Cons, start, state.get_position());
                return true;
            }
            ('=', Some('=')) => {
                state.advance(2);
                state.add_token(FSharpSyntaxKind::Equal, start, state.get_position());
                return true;
            }
            ('<', Some('=')) => {
                state.advance(2);
                state.add_token(FSharpSyntaxKind::LessEqual, start, state.get_position());
                return true;
            }
            ('>', Some('=')) => {
                state.advance(2);
                state.add_token(FSharpSyntaxKind::GreaterEqual, start, state.get_position());
                return true;
            }
            ('<', Some('>')) => {
                state.advance(2);
                state.add_token(FSharpSyntaxKind::NotEqual, start, state.get_position());
                return true;
            }
            ('|', Some('>')) => {
                state.advance(2);
                state.add_token(FSharpSyntaxKind::Pipe, start, state.get_position());
                return true;
            }
            _ => {}
        }

        // 单字符操作符和标点符号
        let kind = match c {
            '+' => FSharpSyntaxKind::Plus,
            '-' => FSharpSyntaxKind::Minus,
            '*' => FSharpSyntaxKind::Star,
            '/' => FSharpSyntaxKind::Slash,
            '%' => FSharpSyntaxKind::Percent,
            '=' => FSharpSyntaxKind::Equal,
            '<' => FSharpSyntaxKind::LessThan,
            '>' => FSharpSyntaxKind::GreaterThan,
            '&' => FSharpSyntaxKind::Ampersand,
            '|' => FSharpSyntaxKind::Pipe,
            '^' => FSharpSyntaxKind::Caret,
            '!' => FSharpSyntaxKind::Not,
            '?' => FSharpSyntaxKind::Question,
            ':' => FSharpSyntaxKind::Colon,
            ';' => FSharpSyntaxKind::Semicolon,
            ',' => FSharpSyntaxKind::Comma,
            '.' => FSharpSyntaxKind::Dot,
            '(' => FSharpSyntaxKind::LeftParen,
            ')' => FSharpSyntaxKind::RightParen,
            '[' => FSharpSyntaxKind::LeftBracket,
            ']' => FSharpSyntaxKind::RightBracket,
            '{' => FSharpSyntaxKind::LeftBrace,
            '}' => FSharpSyntaxKind::RightBrace,
            _ => return false,
        };

        state.advance(1);
        state.add_token(kind, start, state.get_position());
        true
    }
}
