use crate::{language::JasmLanguage, syntax::JasmSyntaxKind};
use alloc::string::String;
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, JasmLanguage>;

/// JASM 词法分析

pub struct JasmLexer<'config> {
    _config: &'config JasmLanguage,
}

impl<'config> JasmLexer<'config> {
    /// 创建新的 JASM lexer
    pub fn new(config: &'config JasmLanguage) -> Self {
        Self { _config: config }
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
            state.add_token(JasmSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(JasmSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(JasmSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            if let Some('/') = source.get_char_at(start_pos + 1) {
                state.advance(2);

                // 读取到行末
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }

                state.add_token(JasmSyntaxKind::Comment, start_pos, state.get_position());
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
    fn lex_string(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.advance(1);
            let mut escaped = false;

            while let Some(ch) = state.peek() {
                if escaped {
                    escaped = false;
                    state.advance(ch.len_utf8());
                }
                else if ch == '\\' {
                    escaped = true;
                    state.advance(1);
                }
                else if ch == '"' {
                    state.advance(1);
                    state.add_token(JasmSyntaxKind::StringLiteral, start_pos, state.get_position());
                    return true;
                }
                else if ch == '\n' || ch == '\r' {
                    // 字符串不能跨行
                    break;
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            // 未闭合的字符            state.set_position(start_pos);
            false
        }
        else {
            false
        }
    }

    /// 处理数字
    fn lex_number(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() || (ch == '-' && source.get_char_at(start_pos + 1).map_or(false, |c| c.is_ascii_digit())) {
                if ch == '-' {
                    state.advance(1);
                }

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() || ch == '.' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                state.add_token(JasmSyntaxKind::Number, start_pos, state.get_position());
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
    fn lex_identifier(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                let mut text = String::new();

                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        text.push(ch);
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 检查是否为关键字或指令
                let kind = match text.as_str() {
                    // 关键                    "class" => JasmSyntaxKind::ClassKw,
                    "version" => JasmSyntaxKind::VersionKw,
                    "method" => JasmSyntaxKind::MethodKw,
                    "field" => JasmSyntaxKind::FieldKw,
                    "string" => JasmSyntaxKind::StringKw,
                    "sourcefile" => JasmSyntaxKind::SourceFileKw,
                    "stack" => JasmSyntaxKind::StackKw,
                    "locals" => JasmSyntaxKind::LocalsKw,
                    "end" => JasmSyntaxKind::EndKw,
                    "compiled" => JasmSyntaxKind::CompiledKw,
                    "from" => JasmSyntaxKind::FromKw,
                    "innerclass" => JasmSyntaxKind::InnerClassKw,
                    "nestmembers" => JasmSyntaxKind::NestMembersKw,
                    "bootstrapmethod" => JasmSyntaxKind::BootstrapMethodKw,

                    // 访问修饰                    "public" => JasmSyntaxKind::Public,
                    "private" => JasmSyntaxKind::Private,
                    "protected" => JasmSyntaxKind::Protected,
                    "static" => JasmSyntaxKind::Static,
                    "super" => JasmSyntaxKind::Super,
                    "final" => JasmSyntaxKind::Final,
                    "abstract" => JasmSyntaxKind::Abstract,
                    "synchronized" => JasmSyntaxKind::Synchronized,
                    "native" => JasmSyntaxKind::Native,
                    "synthetic" => JasmSyntaxKind::Synthetic,
                    "deprecated" => JasmSyntaxKind::Deprecated,
                    "varargs" => JasmSyntaxKind::Varargs,

                    // JVM 指令
                    "aload_0" => JasmSyntaxKind::ALoad0,
                    "aload_1" => JasmSyntaxKind::ALoad1,
                    "aload_2" => JasmSyntaxKind::ALoad2,
                    "aload_3" => JasmSyntaxKind::ALoad3,
                    "iload_0" => JasmSyntaxKind::ILoad0,
                    "iload_1" => JasmSyntaxKind::ILoad1,
                    "iload_2" => JasmSyntaxKind::ILoad2,
                    "iload_3" => JasmSyntaxKind::ILoad3,
                    "ldc" => JasmSyntaxKind::Ldc,
                    "ldc_w" => JasmSyntaxKind::LdcW,
                    "ldc2_w" => JasmSyntaxKind::Ldc2W,
                    "invokespecial" => JasmSyntaxKind::InvokeSpecial,
                    "invokevirtual" => JasmSyntaxKind::InvokeVirtual,
                    "invokestatic" => JasmSyntaxKind::InvokeStatic,
                    "invokeinterface" => JasmSyntaxKind::InvokeInterface,
                    "invokedynamic" => JasmSyntaxKind::InvokeDynamic,
                    "getstatic" => JasmSyntaxKind::GetStatic,
                    "putstatic" => JasmSyntaxKind::PutStatic,
                    "getfield" => JasmSyntaxKind::GetField,
                    "putfield" => JasmSyntaxKind::PutField,
                    "return" => JasmSyntaxKind::Return,
                    "ireturn" => JasmSyntaxKind::IReturn,
                    "areturn" => JasmSyntaxKind::AReturn,
                    "lreturn" => JasmSyntaxKind::LReturn,
                    "freturn" => JasmSyntaxKind::FReturn,
                    "dreturn" => JasmSyntaxKind::DReturn,
                    "nop" => JasmSyntaxKind::Nop,
                    "dup" => JasmSyntaxKind::Dup,
                    "pop" => JasmSyntaxKind::Pop,
                    "new" => JasmSyntaxKind::New,

                    // 默认为标识符
                    _ => JasmSyntaxKind::IdentifierToken,
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

    /// 处理特殊字符
    fn lex_special_char(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '{' => JasmSyntaxKind::LeftBrace,
                '}' => JasmSyntaxKind::RightBrace,
                '(' => JasmSyntaxKind::LeftParen,
                ')' => JasmSyntaxKind::RightParen,
                '[' => JasmSyntaxKind::LeftBracket,
                ']' => JasmSyntaxKind::RightBracket,
                ':' => JasmSyntaxKind::Colon,
                ';' => JasmSyntaxKind::Semicolon,
                '.' => JasmSyntaxKind::Dot,
                ',' => JasmSyntaxKind::Comma,
                '/' => JasmSyntaxKind::Slash,
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

impl<'config> Lexer<JasmLanguage> for JasmLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<JasmSyntaxKind> {
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

            if self.lex_string(&mut state) {
                continue;
            }

            if self.lex_number(&mut state, source) {
                continue;
            }

            if self.lex_identifier(&mut state) {
                continue;
            }

            if self.lex_special_char(&mut state) {
                continue;
            }

            // 如果没有匹配任何规则，标记为错误并前进一个字符
            if let Some(ch) = state.peek() {
                let start_pos = state.get_position();
                state.advance(ch.len_utf8());
                state.add_token(JasmSyntaxKind::Error, start_pos, state.get_position());
            }
            else {
                break;
            }
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(JasmSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
