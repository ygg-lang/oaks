use crate::{kind::JasminSyntaxKind, language::JasminLanguage};
use oak_core::{IncrementalCache, Lexer, LexerState, SourceText, lexer::LexOutput, source::Source};
use std::range::Range;

type State<S: Source> = LexerState<S, JasminLanguage>;

/// Jasmin 词法分析器
#[derive(Clone)]
pub struct JasminLexer<'config> {
    config: &'config JasminLanguage,
}

impl<'config> JasminLexer<'config> {
    pub fn new(config: &'config JasminLanguage) -> Self {
        Self { config }
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
    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();
        let mut consumed = false;

        while let Some(ch) = state.current() {
            if ch.is_whitespace() {
                consumed = true;
                state.advance(1);
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
    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some(ch) = state.current() {
            if ch == ';' {
                let start_pos = state.get_position();
                // 跳过到行尾
                while let Some(ch) = state.current() {
                    state.advance(1);
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
    fn lex_string<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some(ch) = state.current() {
            if ch == '"' {
                let start_pos = state.get_position();
                state.advance(1); // 跳过开始的引号

                while let Some(ch) = state.current() {
                    if ch == '"' {
                        state.advance(1); // 跳过结束的引号
                        break;
                    }
                    else if ch == '\\' {
                        state.advance(1); // 跳过转义字符
                        if state.current().is_some() {
                            state.advance(1); // 跳过被转义的字符
                        }
                    }
                    else {
                        state.advance(1);
                    }
                }

                state.add_token(JasminSyntaxKind::StringLiteral, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理数字字面量
    fn lex_number<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(first) = state.current() {
            // 只处理以数字开头的情况，简化逻辑
            if !first.is_ascii_digit() {
                return false;
            }

            // 消费数字
            while let Some(ch) = state.current() {
                if ch.is_ascii_digit() {
                    state.advance(1);
                }
                else if ch == '.' {
                    // 浮点数
                    state.advance(1);
                    while let Some(ch) = state.current() {
                        if ch.is_ascii_digit() {
                            state.advance(1);
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
    fn lex_identifier_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let first = match state.current() {
            Some(ch) => ch,
            None => return false,
        };

        // 标识符必须以字母、下划线或点开始
        if !first.is_ascii_alphabetic() && first != '_' && first != '.' {
            return false;
        }

        // 消费第一个字符
        state.advance(1);

        // 消费后续字符
        while let Some(ch) = state.current() {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '/' || ch == '$' || ch == '<' || ch == '>' {
                state.advance(1);
            }
            else {
                break;
            }
        }

        let range = start..state.get_position();
        let text = state.get_text_in(range.into());
        let kind = self.keyword_or_identifier(text);
        state.add_token(kind, start, state.get_position());
        true
    }

    /// 处理操作符和分隔符
    fn lex_operator_or_delimiter<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let ch = match state.current() {
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

        state.advance(1);
        state.add_token(kind, start, state.get_position());
        true
    }
}

impl JasminLexer<'static> {
    pub fn default() -> Self {
        static DEFAULT_LANGUAGE: JasminLanguage = JasminLanguage { extended: false, comments: true };
        Self { config: &DEFAULT_LANGUAGE }
    }

    /// 对输入文本进行词法分析，返回 LexOutput
    pub fn tokenize(&self, input: &str) -> LexOutput<JasminLanguage> {
        let source = SourceText::new(input);
        self.lex(&source)
    }
}

impl<'config> Lexer<JasminLanguage> for JasminLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<JasminLanguage>,
    ) -> LexOutput<JasminLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> JasminLexer<'config> {
    /// 主要的词法分析循环
    fn run<S: Source>(&self, state: &mut State<S>) -> Result<(), oak_core::OakError> {
        while state.not_at_end() {
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
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(JasminSyntaxKind::Eof, eof_pos, eof_pos);

        Ok(())
    }
}
