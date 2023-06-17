#![doc = include_str!("readme.md")]

use crate::{kind::JasminSyntaxKind, language::JasminLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, JasminLanguage>;

/// Jasmin 词法分析
pub struct JasminLexer<'config> {
    config: &'config JasminLanguage,
}

impl<'config> JasminLexer<'config> {
    pub fn new(config: &'config JasminLanguage) -> Self {
        Self { config }
    }

    /// 为了向后兼容，提tokenize_source 方法
    pub fn tokenize_source(&self, source: &SourceText) -> LexOutput<JasminSyntaxKind> {
        self.lex(source)
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
            state.add_token(JasminSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(JasminSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(JasminSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State, source: &SourceText) -> bool {
        if !self.config.comments {
            return false;
        }

        let start_pos = state.get_position();

        if let Some(';') = state.peek() {
            state.advance(1);

            // 读取到行
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(JasminSyntaxKind::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符串字面量
    fn lex_string_literal(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.advance(1); // 跳过开始的引号
            let mut escaped = false;

            while let Some(ch) = state.peek() {
                if escaped {
                    escaped = false;
                    state.advance(ch.len_utf8());
                }
                else if ch == '\\' {
                    escaped = true;
                    state.advance(ch.len_utf8());
                }
                else if ch == '"' {
                    state.advance(1); // 跳过结束的引
                    break;
                }
                else if ch == '\n' || ch == '\r' {
                    // 字符串不能跨
                    break;
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            state.add_token(JasminSyntaxKind::StringLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理数字字面量
    fn lex_number(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        // 处理负号
        if let Some('-') = state.peek() {
            state.advance(1);
        }

        let mut has_digits = false;

        // 处理整数部分
        while let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                has_digits = true;
                state.advance(1);
            }
            else {
                break;
            }
        }

        // 处理小数点和小数部分
        if let Some('.') = state.peek() {
            state.advance(1);
            while let Some(ch) = state.peek() {
                if ch.is_ascii_digit() {
                    has_digits = true;
                    state.advance(1);
                }
                else {
                    break;
                }
            }
        }

        // 处理科学计数
        if let Some(ch) = state.peek() {
            if ch == 'e' || ch == 'E' {
                state.advance(1);
                if let Some(sign) = state.peek() {
                    if sign == '+' || sign == '-' {
                        state.advance(1);
                    }
                }
                while let Some(digit) = state.peek() {
                    if digit.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }
            }
        }

        if has_digits && state.get_position() > start_pos {
            state.add_token(JasminSyntaxKind::Number, start_pos, state.get_position());
            true
        }
        else {
            // 回退到开始位
            state.set_position(start_pos);
            false
        }
    }

    /// 处理指令（以点开头）
    fn lex_directive(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some('.') = state.peek() {
            state.advance(1);

            // 读取指令名称
            while let Some(ch) = state.peek() {
                if ch.is_alphanumeric() || ch == '_' {
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }

            if state.get_position() > start_pos + 1 {
                let text = source.get_text_in((start_pos + 1..state.get_position()).into()).unwrap_or("");
                let token_kind = match text.to_ascii_lowercase().as_str() {
                    "class" => JasminSyntaxKind::ClassKw,
                    "method" => JasminSyntaxKind::MethodKw,
                    "field" => JasminSyntaxKind::FieldKw,
                    "super" => JasminSyntaxKind::Super,
                    "source" | "sourcefile" => JasminSyntaxKind::SourceFileKw,
                    "stack" => JasminSyntaxKind::StackKw,
                    "locals" => JasminSyntaxKind::LocalsKw,
                    "end" => JasminSyntaxKind::EndKw,
                    "version" => JasminSyntaxKind::VersionKw,
                    "compiled" => JasminSyntaxKind::CompiledKw,
                    "from" => JasminSyntaxKind::FromKw,
                    "innerclass" => JasminSyntaxKind::InnerClassKw,
                    "nestmembers" => JasminSyntaxKind::NestMembersKw,
                    "bootstrapmethod" => JasminSyntaxKind::BootstrapMethodKw,
                    _ => JasminSyntaxKind::IdentifierToken,
                };

                state.add_token(token_kind, start_pos, state.get_position());
                true
            }
            else {
                // 只有一个点，作为普通符号处
                state.add_token(JasminSyntaxKind::Dot, start_pos, state.get_position());
                true
            }
        }
        else {
            false
        }
    }

    /// 处理标识符和关键字
    fn lex_identifier_or_keyword(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' || ch == '$' {
                state.advance(ch.len_utf8());

                // 继续读取标识符字
                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' || ch == '$' || ch == '/' || ch == '.' || ch == '<' || ch == '>' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = source.get_text_in((start_pos..state.get_position()).into()).unwrap_or("");
                let token_kind = self.keyword_or_identifier(text);

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

    /// 判断是关键字还是标识
    fn keyword_or_identifier(&self, text: &str) -> JasminSyntaxKind {
        match text {
            // 访问修饰
            "public" => JasminSyntaxKind::Public,
            "private" => JasminSyntaxKind::Private,
            "protected" => JasminSyntaxKind::Protected,
            "static" => JasminSyntaxKind::Static,
            "final" => JasminSyntaxKind::Final,
            "abstract" => JasminSyntaxKind::Abstract,
            "synchronized" => JasminSyntaxKind::Synchronized,
            "native" => JasminSyntaxKind::Native,
            "super" => JasminSyntaxKind::Super,
            "synthetic" => JasminSyntaxKind::Synthetic,
            "deprecated" => JasminSyntaxKind::Deprecated,
            "varargs" => JasminSyntaxKind::Varargs,

            // JVM 指令
            "aload_0" => JasminSyntaxKind::ALoad0,
            "aload_1" => JasminSyntaxKind::ALoad1,
            "aload_2" => JasminSyntaxKind::ALoad2,
            "aload_3" => JasminSyntaxKind::ALoad3,
            "iload_0" => JasminSyntaxKind::ILoad0,
            "iload_1" => JasminSyntaxKind::ILoad1,
            "iload_2" => JasminSyntaxKind::ILoad2,
            "iload_3" => JasminSyntaxKind::ILoad3,
            "ldc" => JasminSyntaxKind::Ldc,
            "ldc_w" => JasminSyntaxKind::LdcW,
            "ldc2_w" => JasminSyntaxKind::Ldc2W,
            "invokespecial" => JasminSyntaxKind::InvokeSpecial,
            "invokevirtual" => JasminSyntaxKind::InvokeVirtual,
            "invokestatic" => JasminSyntaxKind::InvokeStatic,
            "invokeinterface" => JasminSyntaxKind::InvokeInterface,
            "invokedynamic" => JasminSyntaxKind::InvokeDynamic,
            "getstatic" => JasminSyntaxKind::GetStatic,
            "putstatic" => JasminSyntaxKind::PutStatic,
            "getfield" => JasminSyntaxKind::GetField,
            "putfield" => JasminSyntaxKind::PutField,
            "return" => JasminSyntaxKind::Return,
            "ireturn" => JasminSyntaxKind::IReturn,
            "areturn" => JasminSyntaxKind::AReturn,
            "lreturn" => JasminSyntaxKind::LReturn,
            "freturn" => JasminSyntaxKind::FReturn,
            "dreturn" => JasminSyntaxKind::DReturn,
            "nop" => JasminSyntaxKind::Nop,
            "dup" => JasminSyntaxKind::Dup,
            "pop" => JasminSyntaxKind::Pop,
            "new" => JasminSyntaxKind::New,

            _ => JasminSyntaxKind::IdentifierToken,
        }
    }

    /// 处理操作符和分隔
    fn lex_operator_or_delimiter(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '{' => JasminSyntaxKind::LeftBrace,
                '}' => JasminSyntaxKind::RightBrace,
                '(' => JasminSyntaxKind::LeftParen,
                ')' => JasminSyntaxKind::RightParen,
                '[' => JasminSyntaxKind::LeftBracket,
                ']' => JasminSyntaxKind::RightBracket,
                ',' => JasminSyntaxKind::Comma,
                ':' => JasminSyntaxKind::Colon,
                ';' => JasminSyntaxKind::Semicolon,
                '/' => JasminSyntaxKind::Slash,
                '.' => JasminSyntaxKind::Dot,
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

impl<'config> Lexer<JasminLanguage> for JasminLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<JasminSyntaxKind> {
        let mut state = State::new(source);

        loop {
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_comment(&mut state, source) {
                continue;
            }

            if self.lex_string_literal(&mut state) {
                continue;
            }

            if self.lex_directive(&mut state, source) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_identifier_or_keyword(&mut state, source) {
                continue;
            }

            if self.lex_operator_or_delimiter(&mut state) {
                continue;
            }

            // 如果没有匹配任何规则，标记为错误并前进一个字符
            if let Some(ch) = state.peek() {
                let start_pos = state.get_position();
                state.advance(ch.len_utf8());
                state.add_token(JasminSyntaxKind::Error, start_pos, state.get_position());
            }
            else {
                break;
            }
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(JasminSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
