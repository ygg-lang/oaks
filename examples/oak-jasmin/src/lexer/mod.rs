use crate::{kind::JasminSyntaxKind, language::JasminLanguage};
use oak_core::{Lexer, LexerCache, LexerState, OakError, TextEdit, lexer::LexOutput, source::Source};

type State<'a, S> = LexerState<'a, S, JasminLanguage>;

/// Jasmin 词法分析器
#[derive(Clone)]
pub struct JasminLexer<'config> {
    _config: &'config JasminLanguage,
}

impl<'config> JasminLexer<'config> {
    pub fn new(config: &'config JasminLanguage) -> Self {
        Self { _config: config }
    }

    /// 判断是关键字还是标识符
    fn keyword_or_identifier(&self, text: &str) -> JasminSyntaxKind {
        match text {
            ".class" => JasminSyntaxKind::ClassKw,
            ".version" => JasminSyntaxKind::VersionKw,
            ".method" => JasminSyntaxKind::MethodKw,
            ".field" => JasminSyntaxKind::FieldKw,
            ".string" => JasminSyntaxKind::StringKw,
            ".source" => JasminSyntaxKind::SourceFileKw,
            ".stack" => JasminSyntaxKind::StackKw,
            ".locals" => JasminSyntaxKind::LocalsKw,
            ".end" => JasminSyntaxKind::EndKw,
            ".compiled" => JasminSyntaxKind::CompiledKw,
            ".from" => JasminSyntaxKind::FromKw,
            ".inner" => JasminSyntaxKind::InnerClassKw,
            ".nest" => JasminSyntaxKind::NestMembersKw,
            ".bootstrap" => JasminSyntaxKind::BootstrapMethodKw,

            "public" => JasminSyntaxKind::Public,
            "private" => JasminSyntaxKind::Private,
            "protected" => JasminSyntaxKind::Protected,
            "static" => JasminSyntaxKind::Static,
            "super" => JasminSyntaxKind::Super,
            "final" => JasminSyntaxKind::Final,
            "abstract" => JasminSyntaxKind::Abstract,
            "synchronized" => JasminSyntaxKind::Synchronized,
            "native" => JasminSyntaxKind::Native,
            "synthetic" => JasminSyntaxKind::Synthetic,
            "deprecated" => JasminSyntaxKind::Deprecated,
            "varargs" => JasminSyntaxKind::Varargs,

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
            "areturn" => JasminSyntaxKind::AReturn,
            "ireturn" => JasminSyntaxKind::IReturn,
            "pop" => JasminSyntaxKind::Pop,
            "new" => JasminSyntaxKind::New,

            _ => {
                // 检查是否是类型描述符
                if self.is_type_descriptor(text) { JasminSyntaxKind::TypeDescriptor } else { JasminSyntaxKind::IdentifierToken }
            }
        }
    }

    /// 检查是否是类型描述符
    fn is_type_descriptor(&self, text: &str) -> bool {
        if text.is_empty() {
            return false;
        }

        // 基本类型
        if matches!(text, "B" | "C" | "D" | "F" | "I" | "J" | "S" | "Z" | "V") {
            return true;
        }

        // 数组类型
        if text.starts_with('[') {
            return true;
        }

        // 对象类型
        if text.starts_with('L') && text.ends_with(';') {
            return true;
        }

        false
    }

    /// 跳过空白字符
    fn skip_whitespace<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start_pos = state.get_position();
        let mut consumed = false;

        while let Some(ch) = state.peek() {
            if ch.is_whitespace() {
                consumed = true;
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        if consumed {
            state.add_token(JasminSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 跳过注释
    fn skip_comment<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch == ';' {
                let start_pos = state.get_position();
                // 跳过到行尾
                while let Some(ch) = state.peek() {
                    state.advance(ch.len_utf8());
                    if ch == '\n' {
                        break;
                    }
                }
                state.add_token(JasminSyntaxKind::Comment, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理字符串字面量
    fn lex_string<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch == '"' {
                let start_pos = state.get_position();
                state.advance(1); // 跳过开始的引号

                while let Some(ch) = state.peek() {
                    if ch == '"' {
                        state.advance(1); // 跳过结束的引号
                        break;
                    }
                    else if ch == '\\' {
                        state.advance(1); // 跳过转义字符
                        if state.peek().is_some() {
                            state.advance(1); // 跳过被转义的字符
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(JasminSyntaxKind::StringLiteral, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理数字字面量
    fn lex_number<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(first) = state.peek() {
            // 只处理以数字开头的情况，简化逻辑
            if !first.is_ascii_digit() {
                return false;
            }

            // 消费数字
            while let Some(ch) = state.peek() {
                if ch.is_ascii_digit() {
                    state.advance(ch.len_utf8());
                }
                else if ch == '.' {
                    // 浮点数
                    state.advance(1);
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }
                    break;
                }
                else {
                    break;
                }
            }

            state.add_token(JasminSyntaxKind::Number, start_pos, state.get_position());
            return true;
        }

        false
    }

    /// 处理标识符或关键字
    fn lex_identifier_or_keyword<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();
        let first = match state.peek() {
            Some(ch) => ch,
            None => return false,
        };

        // 标识符必须以字母、下划线或点开始
        if !first.is_ascii_alphabetic() && first != '_' && first != '.' {
            return false;
        }

        // 消费第一个字符
        state.advance(first.len_utf8());

        // 消费后续字符
        while let Some(ch) = state.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '/' || ch == '$' || ch == '<' || ch == '>' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        let end = state.get_position();
        let text = state.get_text_in(oak_core::Range { start, end });
        let kind = self.keyword_or_identifier(&text);
        state.add_token(kind, start, state.get_position());
        true
    }

    /// 处理操作符和分隔符
    fn lex_operator_or_delimiter<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();
        let ch = match state.peek() {
            Some(ch) => ch,
            None => return false,
        };

        let kind = match ch {
            '{' => JasminSyntaxKind::LeftBrace,
            '}' => JasminSyntaxKind::RightBrace,
            '(' => JasminSyntaxKind::LeftParen,
            ')' => JasminSyntaxKind::RightParen,
            '[' => JasminSyntaxKind::LeftBracket,
            ']' => JasminSyntaxKind::RightBracket,
            ':' => JasminSyntaxKind::Colon,
            ';' => JasminSyntaxKind::Semicolon,
            '.' => JasminSyntaxKind::Dot,
            ',' => JasminSyntaxKind::Comma,
            '/' => JasminSyntaxKind::Slash,
            _ => return false,
        };

        state.advance(ch.len_utf8());
        state.add_token(kind, start, state.get_position());
        true
    }

    /// 主要的词法分析循环
    fn run<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            // 尝试各种词法规则
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

            if self.lex_operator_or_delimiter(state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(JasminSyntaxKind::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }
}

impl<'config> Lexer<JasminLanguage> for JasminLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<JasminLanguage>) -> LexOutput<JasminLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}
