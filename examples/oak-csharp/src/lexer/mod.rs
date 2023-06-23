#![doc = include_str!("readme.md")]
use crate::language::CSharpLanguage;
pub mod token_type;
use oak_core::{
    Lexer, LexerCache, LexerState,
    lexer::LexOutput,
    source::{Source, TextEdit},
};
pub use token_type::CSharpTokenType;

type State<'a, S> = LexerState<'a, S, CSharpLanguage>;

pub struct CSharpLexer<'config> {
    _config: &'config CSharpLanguage,
}

impl<'config> CSharpLexer<'config> {
    pub fn new(config: &'config CSharpLanguage) -> Self {
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
            state.add_token(CSharpTokenType::Whitespace, start_pos, state.get_position());
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
            state.add_token(CSharpTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(CSharpTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            state.advance(1);
            if let Some('/') = state.peek() {
                // 单行注释
                state.advance(1);
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }
                state.add_token(CSharpTokenType::Comment, start_pos, state.get_position());
                return true;
            }
            else if let Some('*') = state.peek() {
                // 多行注释
                state.advance(1);
                while let Some(ch) = state.peek() {
                    if ch == '*' {
                        state.advance(1);
                        if let Some('/') = state.peek() {
                            state.advance(1);
                            break;
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }
                state.add_token(CSharpTokenType::Comment, start_pos, state.get_position());
                return true;
            }
            else {
                // 回退，这不是注释
                state.set_position(start_pos);
                return false;
            }
        }
        false
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
                        state.advance(1)
                    }
                }
                else {
                    state.advance(ch.len_utf8())
                }
            }
            state.add_token(CSharpTokenType::StringLiteral, start_pos, state.get_position());
            true
        }
        else if let Some('\'') = state.peek() {
            // 字符字面量
            state.advance(1);
            while let Some(ch) = state.peek() {
                if ch == '\'' {
                    state.advance(1);
                    break;
                }
                else if ch == '\\' {
                    state.advance(1);
                    if let Some(_) = state.peek() {
                        state.advance(1)
                    }
                }
                else {
                    state.advance(ch.len_utf8())
                }
            }
            state.add_token(CSharpTokenType::CharLiteral, start_pos, state.get_position());
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
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() || ch == '.' || ch == '_' { state.advance(ch.len_utf8()) } else { break }
                }

                // 处理后缀 (f, d, m, l, ul, etc.)
                if let Some(ch) = state.peek() {
                    if ch.is_ascii_alphabetic() {
                        state.advance(ch.len_utf8());
                        if let Some(ch2) = state.peek() {
                            if ch2.is_ascii_alphabetic() {
                                state.advance(ch2.len_utf8())
                            }
                        }
                    }
                }

                state.add_token(CSharpTokenType::NumberLiteral, start_pos, state.get_position());
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

    /// 处理关键字或标识符
    fn lex_keyword_or_identifier<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' || ch == '@' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' { state.advance(ch.len_utf8()) } else { break }
                }

                let text = state.get_text_in((start_pos..state.get_position()).into());
                let token_kind = match text.as_ref() {
                    // C# 关键字
                    "abstract" => CSharpTokenType::Abstract,
                    "as" => CSharpTokenType::As,
                    "async" => CSharpTokenType::AsyncKeyword,
                    "await" => CSharpTokenType::AwaitKeyword,
                    "base" => CSharpTokenType::Base,
                    "bool" => CSharpTokenType::Bool,
                    "break" => CSharpTokenType::Break,
                    "byte" => CSharpTokenType::Byte,
                    "case" => CSharpTokenType::Case,
                    "catch" => CSharpTokenType::Catch,
                    "char" => CSharpTokenType::Char,
                    "checked" => CSharpTokenType::Checked,
                    "class" => CSharpTokenType::Class,
                    "const" => CSharpTokenType::Const,
                    "continue" => CSharpTokenType::Continue,
                    "decimal" => CSharpTokenType::Decimal,
                    "default" => CSharpTokenType::Default,
                    "delegate" => CSharpTokenType::Delegate,
                    "do" => CSharpTokenType::Do,
                    "double" => CSharpTokenType::Double,
                    "else" => CSharpTokenType::Else,
                    "enum" => CSharpTokenType::Enum,
                    "event" => CSharpTokenType::Event,
                    "explicit" => CSharpTokenType::Explicit,
                    "extern" => CSharpTokenType::Extern,
                    "false" => CSharpTokenType::False,
                    "finally" => CSharpTokenType::Finally,
                    "fixed" => CSharpTokenType::Fixed,
                    "float" => CSharpTokenType::Float,
                    "for" => CSharpTokenType::For,
                    "foreach" => CSharpTokenType::Foreach,
                    "goto" => CSharpTokenType::Goto,
                    "if" => CSharpTokenType::If,
                    "implicit" => CSharpTokenType::Implicit,
                    "in" => CSharpTokenType::In,
                    "int" => CSharpTokenType::Int,
                    "interface" => CSharpTokenType::Interface,
                    "internal" => CSharpTokenType::Internal,
                    "is" => CSharpTokenType::Is,
                    "lock" => CSharpTokenType::Lock,
                    "long" => CSharpTokenType::Long,
                    "namespace" => CSharpTokenType::Namespace,
                    "new" => CSharpTokenType::New,
                    "null" => CSharpTokenType::Null,
                    "object" => CSharpTokenType::Object,
                    "operator" => CSharpTokenType::Operator,
                    "out" => CSharpTokenType::Out,
                    "override" => CSharpTokenType::Override,
                    "params" => CSharpTokenType::Params,
                    "private" => CSharpTokenType::Private,
                    "protected" => CSharpTokenType::Protected,
                    "public" => CSharpTokenType::Public,
                    "readonly" => CSharpTokenType::Readonly,
                    "record" => CSharpTokenType::Record,
                    "ref" => CSharpTokenType::Ref,
                    "return" => CSharpTokenType::Return,
                    "sbyte" => CSharpTokenType::Sbyte,
                    "sealed" => CSharpTokenType::Sealed,
                    "short" => CSharpTokenType::Short,
                    "sizeof" => CSharpTokenType::Sizeof,
                    "stackalloc" => CSharpTokenType::Stackalloc,
                    "static" => CSharpTokenType::Static,
                    "string" => CSharpTokenType::String,
                    "struct" => CSharpTokenType::Struct,
                    "switch" => CSharpTokenType::Switch,
                    "this" => CSharpTokenType::This,
                    "throw" => CSharpTokenType::Throw,
                    "true" => CSharpTokenType::True,
                    "try" => CSharpTokenType::Try,
                    "typeof" => CSharpTokenType::Typeof,
                    "uint" => CSharpTokenType::Uint,
                    "ulong" => CSharpTokenType::Ulong,
                    "unchecked" => CSharpTokenType::Unchecked,
                    "unsafe" => CSharpTokenType::Unsafe,
                    "ushort" => CSharpTokenType::Ushort,
                    "using" => CSharpTokenType::Using,
                    "virtual" => CSharpTokenType::Virtual,
                    "void" => CSharpTokenType::Void,
                    "volatile" => CSharpTokenType::Volatile,
                    "while" => CSharpTokenType::While,
                    _ => CSharpTokenType::Identifier,
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

    /// 处理操作符
    fn lex_operator<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '+' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CSharpTokenType::PlusAssign
                    }
                    else if let Some('+') = state.peek() {
                        state.advance(1);
                        CSharpTokenType::Increment
                    }
                    else {
                        CSharpTokenType::Plus
                    }
                }
                '-' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CSharpTokenType::MinusAssign
                    }
                    else if let Some('-') = state.peek() {
                        state.advance(1);
                        CSharpTokenType::Decrement
                    }
                    else {
                        CSharpTokenType::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CSharpTokenType::StarAssign
                    }
                    else {
                        CSharpTokenType::Star
                    }
                }
                '/' => {
                    // 这里不处理注释，因为已经在 lex_comment 中处理了
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CSharpTokenType::SlashAssign
                    }
                    else {
                        CSharpTokenType::Slash
                    }
                }
                '%' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CSharpTokenType::PercentAssign
                    }
                    else {
                        CSharpTokenType::Percent
                    }
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CSharpTokenType::Equal
                    }
                    else {
                        CSharpTokenType::Assign
                    }
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CSharpTokenType::NotEqual
                    }
                    else {
                        CSharpTokenType::LogicalNot
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CSharpTokenType::LessEqual
                    }
                    else if let Some('<') = state.peek() {
                        state.advance(1);
                        CSharpTokenType::LeftShift
                    }
                    else {
                        CSharpTokenType::Less
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CSharpTokenType::GreaterEqual
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        CSharpTokenType::RightShift
                    }
                    else {
                        CSharpTokenType::Greater
                    }
                }
                '&' => {
                    state.advance(1);
                    if let Some('&') = state.peek() {
                        state.advance(1);
                        CSharpTokenType::LogicalAnd
                    }
                    else {
                        CSharpTokenType::Ampersand
                    }
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.peek() {
                        state.advance(1);
                        CSharpTokenType::LogicalOr
                    }
                    else {
                        CSharpTokenType::Pipe
                    }
                }
                '^' => {
                    state.advance(1);
                    CSharpTokenType::Caret
                }
                '~' => {
                    state.advance(1);
                    CSharpTokenType::Tilde
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

    /// 处理分隔符
    fn lex_delimiter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => CSharpTokenType::LeftParen,
                ')' => CSharpTokenType::RightParen,
                '[' => CSharpTokenType::LeftBracket,
                ']' => CSharpTokenType::RightBracket,
                '{' => CSharpTokenType::LeftBrace,
                '}' => CSharpTokenType::RightBrace,
                ';' => CSharpTokenType::Semicolon,
                ',' => CSharpTokenType::Comma,
                '.' => CSharpTokenType::Dot,
                ':' => CSharpTokenType::Colon,
                '?' => CSharpTokenType::Question,
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

    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

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

            if self.lex_number(state) {
                continue;
            }

            if self.lex_keyword_or_identifier(state) {
                continue;
            }

            if self.lex_operator(state) {
                continue;
            }

            if self.lex_delimiter(state) {
                continue;
            }

            // 如果没有匹配到任何模式，处理错误字符并前进
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(CSharpTokenType::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point)
        }
        Ok(())
    }
}

impl<'config> Lexer<CSharpLanguage> for CSharpLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, text: &S, _edits: &[TextEdit], mut cache: &'a mut impl LexerCache<CSharpLanguage>) -> LexOutput<CSharpLanguage> {
        let mut state = LexerState::new(text);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, &mut cache)
    }
}
