use crate::{CSharpSyntaxKind, language::CSharpLanguage};
use oak_core::{IncrementalCache, Lexer, LexerState, lexer::LexOutput, source::Source};

type State<S> = LexerState<S, CSharpLanguage>;

pub struct CSharpLexer;

impl CSharpLexer {
    pub fn new(_config: &CSharpLanguage) -> Self {
        Self
    }

    /// 跳过空白字符
    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
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
            state.add_token(CSharpSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(CSharpSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(CSharpSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment<S: Source>(&self, state: &mut State<S>) -> bool {
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
                state.add_token(CSharpSyntaxKind::Comment, start_pos, state.get_position());
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
                state.add_token(CSharpSyntaxKind::Comment, start_pos, state.get_position());
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
    fn lex_string<S: Source>(&self, state: &mut State<S>) -> bool {
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
            state.add_token(CSharpSyntaxKind::StringLiteral, start_pos, state.get_position());
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
                        state.advance(1);
                    }
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }
            state.add_token(CSharpSyntaxKind::CharLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理数字字面量
    fn lex_number<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() || ch == '.' || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 处理后缀 (f, d, m, l, ul, etc.)
                if let Some(ch) = state.peek() {
                    if ch.is_ascii_alphabetic() {
                        state.advance(ch.len_utf8());
                        if let Some(ch2) = state.peek() {
                            if ch2.is_ascii_alphabetic() {
                                state.advance(ch2.len_utf8());
                            }
                        }
                    }
                }

                state.add_token(CSharpSyntaxKind::NumberLiteral, start_pos, state.get_position());
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
    fn lex_keyword_or_identifier<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' || ch == '@' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = state.get_text_in((start_pos..state.get_position()).into());
                let token_kind = match text {
                    // C# 关键字
                    "abstract" => CSharpSyntaxKind::Abstract,
                    "as" => CSharpSyntaxKind::As,
                    "base" => CSharpSyntaxKind::Base,
                    "bool" => CSharpSyntaxKind::Bool,
                    "break" => CSharpSyntaxKind::Break,
                    "byte" => CSharpSyntaxKind::Byte,
                    "case" => CSharpSyntaxKind::Case,
                    "catch" => CSharpSyntaxKind::Catch,
                    "char" => CSharpSyntaxKind::Char,
                    "checked" => CSharpSyntaxKind::Checked,
                    "class" => CSharpSyntaxKind::Class,
                    "const" => CSharpSyntaxKind::Const,
                    "continue" => CSharpSyntaxKind::Continue,
                    "decimal" => CSharpSyntaxKind::Decimal,
                    "default" => CSharpSyntaxKind::Default,
                    "delegate" => CSharpSyntaxKind::Delegate,
                    "do" => CSharpSyntaxKind::Do,
                    "double" => CSharpSyntaxKind::Double,
                    "else" => CSharpSyntaxKind::Else,
                    "enum" => CSharpSyntaxKind::Enum,
                    "event" => CSharpSyntaxKind::Event,
                    "explicit" => CSharpSyntaxKind::Explicit,
                    "extern" => CSharpSyntaxKind::Extern,
                    "false" => CSharpSyntaxKind::False,
                    "finally" => CSharpSyntaxKind::Finally,
                    "fixed" => CSharpSyntaxKind::Fixed,
                    "float" => CSharpSyntaxKind::Float,
                    "for" => CSharpSyntaxKind::For,
                    "foreach" => CSharpSyntaxKind::Foreach,
                    "goto" => CSharpSyntaxKind::Goto,
                    "if" => CSharpSyntaxKind::If,
                    "implicit" => CSharpSyntaxKind::Implicit,
                    "in" => CSharpSyntaxKind::In,
                    "int" => CSharpSyntaxKind::Int,
                    "interface" => CSharpSyntaxKind::Interface,
                    "internal" => CSharpSyntaxKind::Internal,
                    "is" => CSharpSyntaxKind::Is,
                    "lock" => CSharpSyntaxKind::Lock,
                    "long" => CSharpSyntaxKind::Long,
                    "namespace" => CSharpSyntaxKind::Namespace,
                    "new" => CSharpSyntaxKind::New,
                    "null" => CSharpSyntaxKind::Null,
                    "object" => CSharpSyntaxKind::Object,
                    "operator" => CSharpSyntaxKind::Operator,
                    "out" => CSharpSyntaxKind::Out,
                    "override" => CSharpSyntaxKind::Override,
                    "params" => CSharpSyntaxKind::Params,
                    "private" => CSharpSyntaxKind::Private,
                    "protected" => CSharpSyntaxKind::Protected,
                    "public" => CSharpSyntaxKind::Public,
                    "readonly" => CSharpSyntaxKind::Readonly,
                    "ref" => CSharpSyntaxKind::Ref,
                    "return" => CSharpSyntaxKind::Return,
                    "sbyte" => CSharpSyntaxKind::Sbyte,
                    "sealed" => CSharpSyntaxKind::Sealed,
                    "short" => CSharpSyntaxKind::Short,
                    "sizeof" => CSharpSyntaxKind::Sizeof,
                    "stackalloc" => CSharpSyntaxKind::Stackalloc,
                    "static" => CSharpSyntaxKind::Static,
                    "string" => CSharpSyntaxKind::String,
                    "struct" => CSharpSyntaxKind::Struct,
                    "switch" => CSharpSyntaxKind::Switch,
                    "this" => CSharpSyntaxKind::This,
                    "throw" => CSharpSyntaxKind::Throw,
                    "true" => CSharpSyntaxKind::True,
                    "try" => CSharpSyntaxKind::Try,
                    "typeof" => CSharpSyntaxKind::Typeof,
                    "uint" => CSharpSyntaxKind::Uint,
                    "ulong" => CSharpSyntaxKind::Ulong,
                    "unchecked" => CSharpSyntaxKind::Unchecked,
                    "unsafe" => CSharpSyntaxKind::Unsafe,
                    "ushort" => CSharpSyntaxKind::Ushort,
                    "using" => CSharpSyntaxKind::Using,
                    "virtual" => CSharpSyntaxKind::Virtual,
                    "void" => CSharpSyntaxKind::Void,
                    "volatile" => CSharpSyntaxKind::Volatile,
                    "while" => CSharpSyntaxKind::While,
                    _ => CSharpSyntaxKind::Identifier,
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
    fn lex_operator<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '+' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CSharpSyntaxKind::PlusAssign
                    }
                    else if let Some('+') = state.peek() {
                        state.advance(1);
                        CSharpSyntaxKind::Increment
                    }
                    else {
                        CSharpSyntaxKind::Plus
                    }
                }
                '-' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CSharpSyntaxKind::MinusAssign
                    }
                    else if let Some('-') = state.peek() {
                        state.advance(1);
                        CSharpSyntaxKind::Decrement
                    }
                    else {
                        CSharpSyntaxKind::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CSharpSyntaxKind::StarAssign
                    }
                    else {
                        CSharpSyntaxKind::Star
                    }
                }
                '/' => {
                    // 这里不处理注释，因为已经在 lex_comment 中处理了
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CSharpSyntaxKind::SlashAssign
                    }
                    else {
                        CSharpSyntaxKind::Slash
                    }
                }
                '%' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CSharpSyntaxKind::PercentAssign
                    }
                    else {
                        CSharpSyntaxKind::Percent
                    }
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CSharpSyntaxKind::Equal
                    }
                    else {
                        CSharpSyntaxKind::Assign
                    }
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CSharpSyntaxKind::NotEqual
                    }
                    else {
                        CSharpSyntaxKind::LogicalNot
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CSharpSyntaxKind::LessEqual
                    }
                    else if let Some('<') = state.peek() {
                        state.advance(1);
                        CSharpSyntaxKind::LeftShift
                    }
                    else {
                        CSharpSyntaxKind::Less
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CSharpSyntaxKind::GreaterEqual
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        CSharpSyntaxKind::RightShift
                    }
                    else {
                        CSharpSyntaxKind::Greater
                    }
                }
                '&' => {
                    state.advance(1);
                    if let Some('&') = state.peek() {
                        state.advance(1);
                        CSharpSyntaxKind::LogicalAnd
                    }
                    else {
                        CSharpSyntaxKind::Ampersand
                    }
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.peek() {
                        state.advance(1);
                        CSharpSyntaxKind::LogicalOr
                    }
                    else {
                        CSharpSyntaxKind::Pipe
                    }
                }
                '^' => {
                    state.advance(1);
                    CSharpSyntaxKind::Caret
                }
                '~' => {
                    state.advance(1);
                    CSharpSyntaxKind::Tilde
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
    fn lex_delimiter<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => CSharpSyntaxKind::LeftParen,
                ')' => CSharpSyntaxKind::RightParen,
                '[' => CSharpSyntaxKind::LeftBracket,
                ']' => CSharpSyntaxKind::RightBracket,
                '{' => CSharpSyntaxKind::LeftBrace,
                '}' => CSharpSyntaxKind::RightBrace,
                ';' => CSharpSyntaxKind::Semicolon,
                ',' => CSharpSyntaxKind::Comma,
                '.' => CSharpSyntaxKind::Dot,
                ':' => CSharpSyntaxKind::Colon,
                '?' => CSharpSyntaxKind::Question,
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
}

impl Lexer<CSharpLanguage> for CSharpLexer {
    fn lex_incremental(
        &self,
        source: impl Source,
        _changed: usize,
        _cache: IncrementalCache<CSharpLanguage>,
    ) -> LexOutput<CSharpLanguage> {
        let mut state = LexerState::new_with_cache(source, _changed, _cache);

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

            if self.lex_keyword_or_identifier(&mut state) {
                continue;
            }

            if self.lex_operator(&mut state) {
                continue;
            }

            if self.lex_delimiter(&mut state) {
                continue;
            }

            // 如果没有匹配到任何模式，处理错误字符并前进
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(CSharpSyntaxKind::Error, start_pos, state.get_position());
            }
            else {
                break;
            }
        }

        state.finish(Ok(()))
    }
}
