use crate::{kind::CSyntaxKind, language::CLanguage};
use oak_core::{IncrementalCache, Lexer, LexerState, OakError, lexer::LexOutput, source::Source};
use std::sync::LazyLock;

type State<S: Source> = LexerState<S, CLanguage>;

pub struct CLexer<'config> {
    config: &'config CLanguage,
}

impl<'config> CLexer<'config> {
    pub fn new(config: &'config CLanguage) -> Self {
        Self { config }
    }

    fn run<S: Source>(&self, state: &mut State<S>) -> Result<(), OakError> {
        while state.not_at_end() {
            if self.skip_whitespace(state) {
                continue;
            }
            if self.skip_comment(state) {
                continue;
            }
            if self.lex_newline(state) {
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
            if self.lex_keyword_or_identifier(state) {
                continue;
            }
            if self.lex_operator_or_delimiter(state) {
                continue;
            }
            if self.lex_preprocessor(state) {
                continue;
            }
            if self.lex_text(state) {
                continue;
            }
            else {
                // 如果没有匹配到任何模式，跳过当前字符
                state.advance(1);
            }
        }
        Ok(())
    }

    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let mut count = 0;

        while let Some(ch) = state.current() {
            if ch.is_whitespace() && ch != '\n' && ch != '\r' {
                state.advance(1);
                count += 1;
            }
            else {
                break;
            }
        }

        if count > 0 {
            state.add_token(CSyntaxKind::Whitespace, start, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if let Some('/') = state.current() {
            if let Some('/') = state.peek() {
                // 单行注释
                state.advance(2);
                while let Some(ch) = state.current() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(1);
                }
                state.add_token(CSyntaxKind::Comment, start, state.get_position());
                return true;
            }
            else if let Some('*') = state.peek() {
                // 多行注释
                state.advance(2);
                while let Some(ch) = state.current() {
                    if ch == '*' && state.peek() == Some('/') {
                        state.advance(2);
                        break;
                    }
                    state.advance(1);
                }
                state.add_token(CSyntaxKind::Comment, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_newline<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.current() {
            if ch == '\n' {
                state.advance(1);
                state.add_token(CSyntaxKind::Whitespace, start, state.get_position());
                return true;
            }
            else if ch == '\r' {
                state.advance(1);
                if state.current() == Some('\n') {
                    state.advance(1);
                }
                state.add_token(CSyntaxKind::Whitespace, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_string<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if let Some('"') = state.current() {
            state.advance(1);
            while let Some(ch) = state.current() {
                if ch == '"' {
                    state.advance(1);
                    break;
                }
                else if ch == '\\' {
                    state.advance(1);
                    if state.current().is_some() {
                        state.advance(1);
                    }
                }
                else {
                    state.advance(1);
                }
            }
            state.add_token(CSyntaxKind::StringLiteral, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_char<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if let Some('\'') = state.current() {
            state.advance(1);
            while let Some(ch) = state.current() {
                if ch == '\'' {
                    state.advance(1);
                    break;
                }
                else if ch == '\\' {
                    state.advance(1);
                    if state.current().is_some() {
                        state.advance(1);
                    }
                }
                else {
                    state.advance(1);
                }
            }
            state.add_token(CSyntaxKind::CharLiteral, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_number<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.current() {
            if ch.is_ascii_digit() {
                state.advance(1);
                while let Some(ch) = state.current() {
                    if ch.is_ascii_alphanumeric() || ch == '.' || ch == 'e' || ch == 'E' || ch == '+' || ch == '-' {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                let text = state.get_text_in((start..state.get_position()).into());
                let kind = if text.contains('.') || text.contains('e') || text.contains('E') {
                    CSyntaxKind::FloatLiteral
                }
                else {
                    CSyntaxKind::IntegerLiteral
                };
                state.add_token(kind, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_keyword_or_identifier<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.current() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(1);
                while let Some(ch) = state.current() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                let text = state.get_text_in((start..state.get_position()).into());
                let kind = if C_KEYWORDS.contains(&text) {
                    match text {
                        "auto" => CSyntaxKind::Auto,
                        "register" => CSyntaxKind::Register,
                        "static" => CSyntaxKind::Static,
                        "extern" => CSyntaxKind::Extern,
                        "typedef" => CSyntaxKind::Typedef,
                        "void" => CSyntaxKind::Void,
                        "char" => CSyntaxKind::Char,
                        "short" => CSyntaxKind::Short,
                        "int" => CSyntaxKind::Int,
                        "long" => CSyntaxKind::Long,
                        "float" => CSyntaxKind::Float,
                        "double" => CSyntaxKind::Double,
                        "signed" => CSyntaxKind::Signed,
                        "unsigned" => CSyntaxKind::Unsigned,
                        "struct" => CSyntaxKind::Struct,
                        "union" => CSyntaxKind::Union,
                        "enum" => CSyntaxKind::Enum,
                        "const" => CSyntaxKind::Const,
                        "volatile" => CSyntaxKind::Volatile,
                        "restrict" => CSyntaxKind::Restrict,
                        "if" => CSyntaxKind::If,
                        "else" => CSyntaxKind::Else,
                        "switch" => CSyntaxKind::Switch,
                        "case" => CSyntaxKind::Case,
                        "default" => CSyntaxKind::Default,
                        "for" => CSyntaxKind::For,
                        "while" => CSyntaxKind::While,
                        "do" => CSyntaxKind::Do,
                        "break" => CSyntaxKind::Break,
                        "continue" => CSyntaxKind::Continue,
                        "goto" => CSyntaxKind::Goto,
                        "return" => CSyntaxKind::Return,
                        "sizeof" => CSyntaxKind::Sizeof,
                        "inline" => CSyntaxKind::Inline,
                        "_Bool" => CSyntaxKind::Bool,
                        "_Complex" => CSyntaxKind::Complex,
                        "_Imaginary" => CSyntaxKind::Imaginary,
                        "_Alignas" => CSyntaxKind::Alignas,
                        "_Alignof" => CSyntaxKind::Alignof,
                        "_Atomic" => CSyntaxKind::Atomic,
                        "_Static_assert" => CSyntaxKind::StaticAssert,
                        "_Thread_local" => CSyntaxKind::ThreadLocal,
                        "_Generic" => CSyntaxKind::Generic,
                        "_Noreturn" => CSyntaxKind::Noreturn,
                        _ => CSyntaxKind::Identifier,
                    }
                }
                else {
                    CSyntaxKind::Identifier
                };
                state.add_token(kind, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_operator_or_delimiter<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.current() {
            let three_char = if let Some(next_ch) = state.peek_next_n(1) {
                if let Some(third_ch) = state.peek_next_n(2) { Some(format!("{}{}{}", ch, next_ch, third_ch)) } else { None }
            }
            else {
                None
            };

            let two_char = if let Some(next_ch) = state.peek_next_n(1) { format!("{}{}", ch, next_ch) } else { String::new() };

            // 检查三字符操作符
            if let Some(ref three) = three_char {
                if let Some(&kind) = C_THREE_CHAR_OPERATORS.get(three.as_str()) {
                    state.advance(3);
                    state.add_token(kind, start, state.get_position());
                    return true;
                }
            }

            // 检查双字符操作符
            if let Some(&kind) = C_TWO_CHAR_OPERATORS.get(two_char.as_str()) {
                state.advance(2);
                state.add_token(kind, start, state.get_position());
                return true;
            }

            // 检查单字符操作符和分隔符
            let kind = match ch {
                '(' => CSyntaxKind::LeftParen,
                ')' => CSyntaxKind::RightParen,
                '[' => CSyntaxKind::LeftBracket,
                ']' => CSyntaxKind::RightBracket,
                '{' => CSyntaxKind::LeftBrace,
                '}' => CSyntaxKind::RightBrace,
                ',' => CSyntaxKind::Comma,
                ';' => CSyntaxKind::Semicolon,
                ':' => CSyntaxKind::Colon,
                '.' => CSyntaxKind::Dot,
                '?' => CSyntaxKind::Question,
                '+' => CSyntaxKind::Plus,
                '-' => CSyntaxKind::Minus,
                '*' => CSyntaxKind::Star,
                '/' => CSyntaxKind::Slash,
                '%' => CSyntaxKind::Percent,
                '=' => CSyntaxKind::Assign,
                '<' => CSyntaxKind::Less,
                '>' => CSyntaxKind::Greater,
                '!' => CSyntaxKind::LogicalNot,
                '&' => CSyntaxKind::BitAnd,
                '|' => CSyntaxKind::BitOr,
                '^' => CSyntaxKind::BitXor,
                '~' => CSyntaxKind::BitNot,
                _ => return false,
            };
            state.advance(1);
            state.add_token(kind, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_preprocessor<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if let Some('#') = state.current() {
            state.advance(1);
            while let Some(ch) = state.current() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(1);
            }
            state.add_token(CSyntaxKind::PreprocessorDirective, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_text<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.current() {
            if !ch.is_whitespace() && !ch.is_ascii_alphanumeric() && !"()[]{},.;:?+-*/%=<>!&|^~#\"'_".contains(ch) {
                state.advance(1);
                state.add_token(CSyntaxKind::Text, start, state.get_position());
                return true;
            }
        }
        false
    }
}

impl<'config> Lexer<CLanguage> for CLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        _changed: usize,
        _cache: IncrementalCache<CLanguage>,
    ) -> LexOutput<CLanguage> {
        let mut state = LexerState::new_with_cache(source, _changed, _cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

static C_KEYWORDS: LazyLock<&[&str]> = LazyLock::new(|| {
    &[
        "auto",
        "register",
        "static",
        "extern",
        "typedef",
        "void",
        "char",
        "short",
        "int",
        "long",
        "float",
        "double",
        "signed",
        "unsigned",
        "struct",
        "union",
        "enum",
        "const",
        "volatile",
        "restrict",
        "if",
        "else",
        "switch",
        "case",
        "default",
        "for",
        "while",
        "do",
        "break",
        "continue",
        "goto",
        "return",
        "sizeof",
        "inline",
        "_Bool",
        "_Complex",
        "_Imaginary",
        "_Alignas",
        "_Alignof",
        "_Atomic",
        "_Static_assert",
        "_Thread_local",
        "_Generic",
        "_Noreturn",
    ]
});

static C_TWO_CHAR_OPERATORS: LazyLock<std::collections::HashMap<&str, CSyntaxKind>> = LazyLock::new(|| {
    let mut map = std::collections::HashMap::new();
    map.insert("+=", CSyntaxKind::PlusAssign);
    map.insert("-=", CSyntaxKind::MinusAssign);
    map.insert("*=", CSyntaxKind::StarAssign);
    map.insert("/=", CSyntaxKind::SlashAssign);
    map.insert("%=", CSyntaxKind::PercentAssign);
    map.insert("==", CSyntaxKind::Equal);
    map.insert("!=", CSyntaxKind::NotEqual);
    map.insert("<=", CSyntaxKind::LessEqual);
    map.insert(">=", CSyntaxKind::GreaterEqual);
    map.insert("&&", CSyntaxKind::LogicalAnd);
    map.insert("||", CSyntaxKind::LogicalOr);
    map.insert("<<", CSyntaxKind::LeftShift);
    map.insert(">>", CSyntaxKind::RightShift);
    map.insert("&=", CSyntaxKind::AndAssign);
    map.insert("|=", CSyntaxKind::OrAssign);
    map.insert("^=", CSyntaxKind::XorAssign);
    map.insert("++", CSyntaxKind::Increment);
    map.insert("--", CSyntaxKind::Decrement);
    map.insert("->", CSyntaxKind::Arrow);
    map
});

static C_THREE_CHAR_OPERATORS: LazyLock<std::collections::HashMap<&str, CSyntaxKind>> = LazyLock::new(|| {
    let mut map = std::collections::HashMap::new();
    map.insert("<<=", CSyntaxKind::LeftShiftAssign);
    map.insert(">>=", CSyntaxKind::RightShiftAssign);
    map
});
